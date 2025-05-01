use regex::Regex;
use std::{
    collections::HashSet,
    fs::{read_dir, read_to_string},
    path::Path,
};
use tree_sitter::{Node, Query, QueryCursor, QueryMatch, StreamingIterator};

mod queries;

#[derive(Debug)]
pub struct OcctPackage {
    name: String,
    forward_declare_classes: HashSet<String>,
    classes: Vec<OcctClass>,
    enums: Vec<OcctEnum>,
}

impl OcctPackage {
    pub fn new(occt_src_dir: impl AsRef<Path>, package_name: impl AsRef<str>) -> Self {
        let package_name = package_name.as_ref();
        let package_dir = occt_src_dir.as_ref().join(package_name);

        let mut forward_declare_classes = HashSet::new();
        let mut classes = vec![];
        let enums = vec![];

        // Read all header files in the package directory.
        for header_file in read_dir(package_dir)
            .unwrap()
            .filter_map(|p| p.ok())
            .filter(|p| p.path().extension().is_some_and(|e| e.to_str() == Some("hxx")))
        {
            // The Standard_DEPRECATED macro causes the parser to fail, comment it out.
            let header_contents = read_to_string(header_file.path())
                .unwrap()
                .replace("DEFINE_STANDARD_ALLOC", "//DEFINE_STANDARD_ALLOC")
                .replace("Standard_DEPRECATED", "//Standard_DEPRECATED")
                .replace("Standard_EXPORT", "/*Standard_EXPORT*/");

            let handle_regex = Regex::new(r"Handle\((?<inner>[^)]*)\)").unwrap();
            let header_contents = handle_regex.replace(&header_contents, "Handle_${inner}");

            let mut parser = tree_sitter::Parser::new();
            let language = tree_sitter_cpp::LANGUAGE;

            parser.set_language(&language.into()).expect("Error loading C++ parser");

            let tree = parser.parse(&*header_contents, None).unwrap();

            // Forward declare classes
            for_each_match(
                &queries::forward_declared_classes(),
                tree.root_node(),
                header_contents.as_bytes(),
                |_q, query_match| {
                    let class_name =
                        query_match.captures[0].node.utf8_text(header_contents.as_bytes()).unwrap();
                    forward_declare_classes.insert(class_name.to_string());
                },
            );

            let src_bytes = header_contents.as_bytes();

            // Classes to bind to
            for_each_match(&queries::class(), tree.root_node(), src_bytes, |query, query_match| {
                let result = QueryResult { query, query_match, src_contents: src_bytes };
                let class_node = result.capture_node("class");
                let class_name = result.capture_text("class_name");

                let new_class =
                    OcctClass::new(class_name.to_string(), *class_node, header_contents.as_bytes());
                classes.push(new_class);
            });
        }

        // dbg!(&classes);

        Self { name: package_name.into(), forward_declare_classes, classes, enums }
    }
}

#[derive(Debug)]
pub struct OcctClass {
    name: String,
    functions: Vec<Function>,
    constructors: Vec<()>,
    methods: Vec<Function>,
    static_methods: Vec<()>,
}

impl OcctClass {
    pub fn new(name: String, class_node: Node, header_contents: &[u8]) -> Self {
        // Build up regions of public and private access in the header file.
        let mut access_regions = vec![];
        for_each_match(
            &queries::access_specifier(),
            class_node,
            header_contents,
            |_query, query_match| {
                let access_node = query_match.captures[0].node;
                let access_text = query_match.captures[0].node.utf8_text(header_contents).unwrap();
                let is_public = access_text == "public";

                access_regions.push((is_public, access_node.start_position().row));
            },
        );

        access_regions.sort_by_key(|a| a.1);

        let mut functions = vec![];

        // Only extract public functions
        for_each_match(&queries::functions(), class_node, header_contents, |query, query_match| {
            let index = query.capture_index_for_name("function").unwrap();
            let func_node = query_match.captures.iter().find(|c| c.index == index).unwrap().node;

            let result = QueryResult { query, query_match, src_contents: header_contents };

            // Find the closest access specifier that was declared before us,
            // defaulting to private if none exist.
            let is_public = access_regions
                .iter()
                .rev()
                .find(|region| func_node.start_position().row >= region.1)
                .map(|r| r.0)
                .unwrap_or(false);

            if is_public {
                let function_text = result.capture_text("function");
                let function = Function::new(function_text, &name);

                functions.push(function);
            }
        });

        Self { name, functions, constructors: vec![], methods: vec![], static_methods: vec![] }
    }
}

#[derive(Debug)]
pub struct OcctEnum {
    name: String,
}

fn for_each_match(
    query_str: &str,
    node: Node,
    src_contents: &[u8],
    mut func: impl FnMut(&Query, &QueryMatch),
) {
    // TODO(bschwind) - Query construction is slow, cache this
    let query = Query::new(&tree_sitter_cpp::LANGUAGE.into(), query_str).unwrap();
    let mut cursor = QueryCursor::new();

    let mut matches = cursor.matches(&query, node, src_contents);

    while let Some(query_match) = matches.next() {
        func(&query, query_match);
    }
}

struct QueryResult<'a> {
    query: &'a Query,
    query_match: &'a QueryMatch<'a, 'a>,
    src_contents: &'a [u8],
}

impl<'a> QueryResult<'a> {
    fn capture_text(&self, name: &str) -> &'a str {
        let index = self.query.capture_index_for_name(name).unwrap();
        self.query_match
            .captures
            .iter()
            .find(|c| c.index == index)
            .unwrap()
            .node
            .utf8_text(self.src_contents)
            .unwrap()
    }

    fn capture_node(&self, name: &str) -> &'a Node {
        let index = self.query.capture_index_for_name(name).unwrap();
        &self.query_match.captures.iter().find(|c| c.index == index).unwrap().node
    }

    fn capture_text_opt(&self, name: &str) -> Option<&'a str> {
        let index = self.query.capture_index_for_name(name).unwrap();
        self.query_match
            .captures
            .iter()
            .find(|c| c.index == index)
            .and_then(|c| c.node.utf8_text(self.src_contents).ok())
    }
}

#[derive(Debug)]
pub enum FunctionType {
    Constructor,
    Static,
    Method,
}

#[derive(Debug)]
pub struct Function {
    name: String,
    return_type: Option<String>,
    args: Vec<String>,
    function_type: FunctionType,
    is_virtual: bool,
}

impl Function {
    pub fn new(function_text: &str, class_name: &str) -> Self {
        let mut parser = tree_sitter::Parser::new();
        let language = tree_sitter_cpp::LANGUAGE;

        parser.set_language(&language.into()).expect("Error loading C++ parser");

        let tree = parser.parse(function_text, None).unwrap();

        let mut storage_class = None;
        let mut function_name = None;
        let mut return_type = None;

        for_each_match(
            &queries::function_definition(),
            tree.root_node(),
            function_text.as_bytes(),
            |query, query_match| {
                let result =
                    QueryResult { query, query_match, src_contents: function_text.as_bytes() };

                storage_class =
                    result.capture_text_opt("storage_specifier").map(|sc| sc.to_string());
                function_name = Some(result.capture_text("name").to_string());
                return_type = result.capture_text_opt("type").map(|t| t.to_string());
            },
        );

        let function_name = function_name.unwrap();

        // TODO(bschwind) - Brittle, but may be fine for now.
        let is_virtual = function_text.contains("virtual");
        let is_static = storage_class == Some("static".to_string());
        let is_constructor = function_name == class_name;

        let function_type = if is_constructor {
            FunctionType::Constructor
        } else if is_static {
            FunctionType::Static
        } else {
            FunctionType::Method
        };

        Self {
            name: function_name,
            return_type: if return_type == Some("void".to_string()) { None } else { return_type },
            args: vec![],
            function_type,
            is_virtual,
        }
    }
}
