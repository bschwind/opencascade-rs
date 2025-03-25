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

            let mut parser = tree_sitter::Parser::new();
            let language = tree_sitter_cpp::LANGUAGE;

            parser.set_language(&language.into()).expect("Error loading C++ parser");

            let tree = parser.parse(&header_contents, None).unwrap();

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

            // Classes to bind to
            for_each_match(
                &queries::class(),
                tree.root_node(),
                header_contents.as_bytes(),
                |_q, query_match| {
                    let class_node = query_match.captures[0].node;

                    let new_class =
                        OcctClass::new(&header_contents, class_node, header_contents.as_bytes());
                    classes.push(new_class);
                },
            );
        }

        dbg!(&forward_declare_classes);

        Self { name: package_name.into(), forward_declare_classes, classes, enums }
    }
}

#[derive(Debug)]
pub struct OcctClass {
    name: String,
    constructors: Vec<()>,
    methods: Vec<()>,
    static_methods: Vec<()>,
}

impl OcctClass {
    pub fn new(_src: &str, class_node: Node, header_contents: &[u8]) -> Self {
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

        // Only extract public functions
        for_each_match(&queries::functions(), class_node, header_contents, |query, query_match| {
            let index = query.capture_index_for_name("method").unwrap();
            let func_node = query_match.captures.iter().find(|c| c.index == index).unwrap().node;

            // Find the closest access specifier that was declared before us,
            // defaulting to private if none exist.
            let is_public = access_regions
                .iter()
                .rev()
                .find(|region| func_node.start_position().row >= region.1)
                .map(|r| r.0)
                .unwrap_or(false);

            let func_text = func_node.utf8_text(header_contents).unwrap();

            dbg!(is_public);
            dbg!(func_text);
        });

        Self {
            name: "lol".to_string(),
            constructors: vec![],
            methods: vec![],
            static_methods: vec![],
        }
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
    let query = Query::new(&tree_sitter_cpp::LANGUAGE.into(), query_str).unwrap();
    let mut cursor = QueryCursor::new();

    let mut matches = cursor.matches(&query, node, src_contents);

    while let Some(query_match) = matches.next() {
        func(&query, query_match);
    }
}
