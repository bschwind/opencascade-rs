use std::{
    fs::{read_dir, read_to_string},
    path::Path,
};
use tree_sitter::{Query, QueryCursor, StreamingIterator};

mod queries;

#[derive(Debug)]
pub struct OcctPackage {
    name: String,
    classes: Vec<OcctClass>,
    enums: Vec<OcctEnum>,
}

impl OcctPackage {
    pub fn new(occt_src_dir: impl AsRef<Path>, package_name: impl AsRef<str>) -> Self {
        let package_name = package_name.as_ref();
        let package_dir = occt_src_dir.as_ref().join(package_name);

        let classes = vec![];
        let enums = vec![];

        // Read all header files in the package directory.
        for header_file in read_dir(package_dir)
            .unwrap()
            .filter_map(|p| p.ok())
            .filter(|p| p.path().extension().is_some_and(|e| e.to_str() == Some("hxx")))
        {
            let header_contents = read_to_string(header_file.path()).unwrap();

            let mut parser = tree_sitter::Parser::new();
            let language = tree_sitter_cpp::LANGUAGE;

            parser.set_language(&language.into()).expect("Error loading C++ parser");

            let tree = parser.parse(&header_contents, None).unwrap();

            let forward_classes =
                Query::new(&language.into(), &queries::forward_declared_classes()).unwrap();
            let mut cursor = QueryCursor::new();

            let mut matches =
                cursor.matches(&forward_classes, tree.root_node(), header_contents.as_bytes());
            while let Some(class) = matches.next() {
                dbg!(class.captures[0].node.utf8_text(header_contents.as_bytes()).unwrap());
            }
        }

        Self { name: package_name.into(), classes, enums }
    }
}

#[derive(Debug)]
pub struct OcctClass {
    name: String,
    constructors: Vec<()>,
    methods: Vec<()>,
    static_methods: Vec<()>,
}

#[derive(Debug)]
pub struct OcctEnum {
    name: String,
}
