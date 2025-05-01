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
                (function_definition)
                (_
                    (function_declarator)
                )
                (_
                    (_
                        (function_declarator)
                    )
                )
            ] @function
        ) @fields
    )
    "
    .to_string()
}

pub fn function_definition() -> String {
    "
    (
            (storage_class_specifier)? @storage_specifier
            ([(type_identifier) (primitive_type)])? @type
            [
                (function_declarator
                    (identifier) @name
                    (parameter_list) @params
                    (type_qualifier)? @is_const
                )
                (call_expression
                    (identifier) @name
                    (argument_list) @params
                )
                (init_declarator
                    (identifier) @name
                    (argument_list) @params
                )
            ]
    )
    "
    .to_string()
}
