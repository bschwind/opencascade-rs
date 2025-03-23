pub fn forward_declared_classes() -> String {
    "(class_specifier
        name: (type_identifier) @name
        !body
    )"
    .to_string()
}

pub fn class() -> String {
    "(class_specifier
        (field_declaration_list
            (access_specifier) @access (#eq? @access \"public\")
        )
    ) @class
    "
    .to_string()
}
