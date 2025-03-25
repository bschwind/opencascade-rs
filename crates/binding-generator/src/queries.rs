pub fn forward_declared_classes() -> String {
    "
    (
        class_specifier
        name: (type_identifier) @name
        !body
    )
    "
    .to_string()
}

pub fn class() -> String {
    "
    (
        class_specifier
        (field_declaration_list)
    ) @class
    "
    .to_string()
}

pub fn access_specifier() -> String {
    "(access_specifier) @access".to_string()
}

pub fn functions() -> String {
    "
    (
        (field_declaration_list
            (field_declaration
                (storage_class_specifier)? @storage
                (type_identifier) @return_type
                (function_declarator) @func
            ) @method
        ) @fields
    )
    "
    .to_string()
}
