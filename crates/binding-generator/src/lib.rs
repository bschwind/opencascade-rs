use std::{
    collections::HashSet,
    fs::{read_dir, read_to_string},
    path::Path,
};
use tree_sitter::{Node, Query, QueryCursor, StreamingIterator};

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
            let forward_classes =
                Query::new(&language.into(), &queries::forward_declared_classes()).unwrap();
            let mut cursor = QueryCursor::new();

            let mut matches =
                cursor.matches(&forward_classes, tree.root_node(), header_contents.as_bytes());
            while let Some(class) = matches.next() {
                let class_name =
                    class.captures[0].node.utf8_text(header_contents.as_bytes()).unwrap();
                forward_declare_classes.insert(class_name.to_string());
            }

            // Classes to bind to
            let classes_query = Query::new(&language.into(), &queries::class()).unwrap();
            let mut cursor = QueryCursor::new();

            let mut matches =
                cursor.matches(&classes_query, tree.root_node(), header_contents.as_bytes());

            while let Some(class) = matches.next() {
                let class_node = class.captures[0].node;
                let class_text =
                    class.captures[0].node.utf8_text(header_contents.as_bytes()).unwrap();
                println!("{}", class_text);
                // dbg!(class_node.to_sexp());

                let new_class = OcctClass::new(&header_contents, class_node);
                classes.push(new_class);
            }
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
    pub fn new(_src: &str, _class_node: Node) -> Self {
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
