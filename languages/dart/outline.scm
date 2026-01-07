(class_definition
    "class" @context
    name: (_) @name) @item

(function_signature
    name: (_) @name) @item

(getter_signature
    "get" @context
    name: (_) @name) @item

(setter_signature
    "set" @context
    name: (_) @name) @item

(enum_declaration
    "enum" @context
    name: (_) @name) @item

(extension_declaration
    "extension" @context
    name: (_) @name) @item

(static_final_declaration
    (identifier) @name) @item

(initialized_identifier
    (identifier) @name) @item
