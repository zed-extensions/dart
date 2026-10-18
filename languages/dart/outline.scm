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

; Test and group blocks.
; Covers official Dart and Flutter test methods, as well as methods from popular packages
((expression_statement
  (identifier) @_fn @context
  (selector
    (argument_part
      (arguments
        .
        (argument
          (string_literal) @name)))))
  (#any-of? @_fn
    "group" "test" "testWidgets" "testGoldens" "goldenTest" "patrolTest")) @item
