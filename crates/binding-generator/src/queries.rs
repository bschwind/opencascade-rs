pub fn forward_declared_classes() -> String {
    "(class_specifier
        name: (type_identifier) @name
        !body
    )"
    .to_string()
}
