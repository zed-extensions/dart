; Dart main — matches dart: SDK imports.
; Since `dart format` places dart: imports first, the @_import capture
; position will be earlier than flutter/test imports, so specific patterns
; (flutter-main, flutter-test-main, etc.) overwrite this when they match.
(program
  (import_or_export
    (library_import
      (import_specification
        (configurable_uri
          (uri
            (string_literal) @_import
            (#match? @_import "^\"dart:"))))))
  (function_signature
    name: (_) @run
    (#eq? @run "main"))
  (#set! tag dart-main))

; Dart main for parse errors (ERROR root).
(ERROR
  (import_or_export
    (library_import
      (import_specification
        (configurable_uri
          (uri
            (string_literal) @_import
            (#match? @_import "^\"dart:"))))))
  (function_signature
    name: (_) @run
    (#eq? @run "main"))
  (#set! tag dart-main))

; Flutter main
(program
  (import_or_export
    (library_import
      (import_specification
        (configurable_uri
          (uri
            (string_literal) @_import
            (#match? @_import "package:flutter/(material|widgets|cupertino).dart"))))))
  (function_signature
    name: (_) @run
    (#eq? @run "main"))
  (#set! tag flutter-main))

; Flutter main (parse error fallback)
(ERROR
  (import_or_export
    (library_import
      (import_specification
        (configurable_uri
          (uri
            (string_literal) @_import
            (#match? @_import "package:flutter/(material|widgets|cupertino).dart"))))))
  (function_signature
    name: (_) @run
    (#eq? @run "main"))
  (#set! tag flutter-main))

; Flutter test main
(program
  (import_or_export
    (library_import
      (import_specification
        (configurable_uri
          (uri
            (string_literal) @_import
            (#match? @_import "package:flutter_test/flutter_test.dart"))))))
  (function_signature
    name: (_) @run
    (#eq? @run "main"))
  (#set! tag flutter-test-main))

; Flutter test main (parse error fallback)
(ERROR
  (import_or_export
    (library_import
      (import_specification
        (configurable_uri
          (uri
            (string_literal) @_import
            (#match? @_import "package:flutter_test/flutter_test.dart"))))))
  (function_signature
    name: (_) @run
    (#eq? @run "main"))
  (#set! tag flutter-test-main))

; Flutter test group (block body: void main() { group(...) })
; Arrow-body variant removed: when main() => group(...), both identifiers
; share the same line, causing group to overwrite the more useful test-main tag.
(program
  (import_or_export
    (library_import
      (import_specification
        (configurable_uri
          (uri
            (string_literal) @_import
            (#match? @_import "package:flutter_test/flutter_test.dart"))))))
  (function_body
    (block
      (expression_statement
        ((identifier) @run
          (#eq? @run "group")))))
  (#set! tag flutter-test-group))

; Flutter test single (block body)
(program
  (import_or_export
    (library_import
      (import_specification
        (configurable_uri
          (uri
            (string_literal) @_import
            (#match? @_import "package:flutter_test/flutter_test.dart"))))))
  (function_body
    (block
      (expression_statement
        (selector
          (argument_part
            (arguments
              (argument
                (function_expression
                  (function_expression_body
                    (block
                      (expression_statement
                        ((identifier) @run
                          (#eq? @run "test")))))))))))))
  (#set! tag flutter-test-single))

; Flutter test single (arrow body: void main() => group("name", () { test(...) }))
(program
  (import_or_export
    (library_import
      (import_specification
        (configurable_uri
          (uri
            (string_literal) @_import
            (#match? @_import "package:flutter_test/flutter_test.dart"))))))
  (function_body
    (selector
      (argument_part
        (arguments
          (argument
            (function_expression
              (function_expression_body
                (block
                  (expression_statement
                    ((identifier) @run
                      (#eq? @run "test")))))))))))
  (#set! tag flutter-test-single))

; Dart test file
(program
  (import_or_export
    (library_import
      (import_specification
        (configurable_uri
          (uri
            (string_literal) @_import
            (#match? @_import "package:test/test.dart"))))))
  (function_signature
    name: (_) @run
    (#eq? @run "main"))
  (#set! tag dart-test-file))

; Dart test group (block body: void main() { group(...) })
; Arrow-body variant removed: same reason as flutter-test-group.
(program
  (import_or_export
    (library_import
      (import_specification
        (configurable_uri
          (uri
            (string_literal) @_import
            (#match? @_import "package:test/test.dart"))))))
  (function_body
    (block
      (expression_statement
        ((identifier) @run
          (#eq? @run "group")))))
  (#set! tag dart-test-group))

; Dart test single (block body)
(program
  (import_or_export
    (library_import
      (import_specification
        (configurable_uri
          (uri
            (string_literal) @_import
            (#match? @_import "package:test/test.dart"))))))
  (function_body
    (block
      (expression_statement
        (selector
          (argument_part
            (arguments
              (argument
                (function_expression
                  (function_expression_body
                    (block
                      (expression_statement
                        ((identifier) @run
                          (#eq? @run "test")))))))))))))
  (#set! tag dart-test-single))

; Dart test single (arrow body)
(program
  (import_or_export
    (library_import
      (import_specification
        (configurable_uri
          (uri
            (string_literal) @_import
            (#match? @_import "package:test/test.dart"))))))
  (function_body
    (selector
      (argument_part
        (arguments
          (argument
            (function_expression
              (function_expression_body
                (block
                  (expression_statement
                    ((identifier) @run
                      (#eq? @run "test")))))))))))
  (#set! tag dart-test-single))
