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
        name: (type_identifier) @class_name
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
            [
                (field_declaration
                     (storage_class_specifier)? @storage
                     [(type_identifier) (primitive_type)] @return_type
                     (function_declarator
                         declarator: (field_identifier) @func_name
                     ) @func
                )
                (function_definition
                    (storage_class_specifier)? @storage
                    [(type_identifier) (primitive_type)] @return_type
                    (function_declarator
                        declarator: (field_identifier) @func_name
                    ) @func
                )
            ] @method
        ) @fields
    )
    "
    .to_string()
}
