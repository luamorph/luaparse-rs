use luaparse_rs::{Parser, Luau};

#[test]
fn test_local_declaration() {
    let input = "local x = 5";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    assert_eq!(ast.block.statements.len(), 1);
}

#[test]
fn test_function_declaration() {
    let input = r#"
        function addd(a, b)
            return a + b
        end
    "#;

    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

}

#[test]
fn test_if_statement() {
    let input = r#"
        if true then
            print("hi")
        end
    "#;

    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

}

#[test]
fn test_table_construction() {
    let input = "local t = {a = 1, b = 2, 3, 4}
    --[[ this is a block comment
    spanning multiple lines ]]";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();
}

#[test]
fn test_simple_assignment() {
    use luaparse_rs::{Parser, Luau, ast::StmtKind};

    let input = "x = 10";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    assert!(matches!(
        &ast.block.statements[0].kind,
        StmtKind::Assignment(_)
    ));
}

#[test]
fn test_multiple_assignment() {
    use luaparse_rs::{Parser, Luau, ast::StmtKind};

    let input = "x, y, z = 1, 2, 3";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    if let StmtKind::Assignment(assign) = &ast.block.statements[0].kind {
        assert_eq!(assign.targets.len(), 3);
        assert_eq!(assign.values.len(), 3);
    } else {
        panic!("Expected Assignment");
    }
}

#[test]
fn test_field_assignment() {
    use luaparse_rs::{Parser, Luau, ast::StmtKind};

    let input = "obj.field = 20";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    assert!(matches!(
        &ast.block.statements[0].kind,
        StmtKind::Assignment(_)
    ));
}

#[test]
fn test_index_assignment() {
    use luaparse_rs::{Parser, Luau, ast::StmtKind};

    let input = "arr[123] = 1234";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    assert!(matches!(
        &ast.block.statements[0].kind,
        StmtKind::Assignment(_)
    ));
}

#[test]
fn test_nested_field_assignment() {
    use luaparse_rs::{Parser, Luau, ast::StmtKind};

    let input = "obj.nested.field = 123";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    assert!(matches!(
        &ast.block.statements[0].kind,
        StmtKind::Assignment(_)
    ));
}

#[test]
fn test_compound_assignment() {
    use luaparse_rs::{Parser, Luau, ast::StmtKind};

    let input = "x += 5";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    assert!(matches!(
        &ast.block.statements[0].kind,
        StmtKind::CompoundAssignment(_)
    ));
}

#[test]
fn test_call_statement() {
    use luaparse_rs::{Parser, Luau, ast::StmtKind};

    let input = "print('hello')";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    assert!(matches!(
        &ast.block.statements[0].kind,
        StmtKind::CallStatement(_)
    ));
}

#[test]
fn test_method_call_statement() {
    use luaparse_rs::{Parser, Luau, ast::StmtKind};

    let input = "obj:method(10)";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    assert!(matches!(
        &ast.block.statements[0].kind,
        StmtKind::CallStatement(_)
    ));
}

#[test]
fn test_parenthesized_assignment() {
    use luaparse_rs::{Parser, Luau, ast::StmtKind};

    let input = "(x()).field = 99";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    assert!(matches!(
        &ast.block.statements[0].kind,
        StmtKind::Assignment(_)
    ));
}

#[test]
fn test_invalid_assignment() {
    use luaparse_rs::{Parser, Luau};

    let input = "5 = x";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_err());
}

#[test]
fn test_simple_interpolation() {
    use luaparse_rs::{Parser, Luau, ast::{StmtKind, ExprKind, InterpolationSegment}};

    let input = r#"local msg = `hello {name}`"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    if let StmtKind::LocalDeclaration(decl) = &ast.block.statements[0].kind {
        if let Some(values) = &decl.values {
            if let ExprKind::InterpolatedString(interp) = &values[0].kind {
                assert_eq!(interp.segments.len(), 2);

                // First segment: "hello "
                assert!(matches!(
                    &interp.segments[0],
                    InterpolationSegment::Text(t) if t == "hello "
                ));

                // Second segment: {name}
                if let InterpolationSegment::Expression(expr) = &interp.segments[1] {
                    assert!(matches!(&expr.kind, ExprKind::Identifier(_)));
                } else {
                    panic!("Expected expression segment");
                }
            } else {
                panic!("Expected InterpolatedString");
            }
        }
    }
}

#[test]
fn test_complex_interpolation() {
    use luaparse_rs::{Parser, Luau, ast::{StmtKind, ExprKind}};

    let input = r#"local msg = `result: {x + y * 2}`"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    if let StmtKind::LocalDeclaration(decl) = &ast.block.statements[0].kind {
        if let Some(values) = &decl.values {
            if let ExprKind::InterpolatedString(interp) = &values[0].kind {
                assert_eq!(interp.segments.len(), 2);
            }
        }
    }
}

#[test]
fn test_multiple_interpolations() {
    use luaparse_rs::{Parser, Luau, ast::{StmtKind, ExprKind, InterpolationSegment}};

    let input = r#"local msg = `{a} + {b} = {a + b}`"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    if let StmtKind::LocalDeclaration(decl) = &ast.block.statements[0].kind {
        if let Some(values) = &decl.values {
            if let ExprKind::InterpolatedString(interp) = &values[0].kind {
                // Should have: {a}, " + ", {b}, " = ", {a + b}
                assert_eq!(interp.segments.len(), 5);

                assert!(matches!(&interp.segments[0], InterpolationSegment::Expression(_)));
                assert!(matches!(&interp.segments[1], InterpolationSegment::Text(t) if t == " + "));
                assert!(matches!(&interp.segments[2], InterpolationSegment::Expression(_)));
                assert!(matches!(&interp.segments[3], InterpolationSegment::Text(t) if t == " = "));
                assert!(matches!(&interp.segments[4], InterpolationSegment::Expression(_)));
            }
        }
    }
}

#[test]
fn test_nested_interpolation_braces() {
    use luaparse_rs::{Parser, Luau, ast::{StmtKind, ExprKind}};

    let input = r#"local msg = `table: {t.field}`"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    if let StmtKind::LocalDeclaration(decl) = &ast.block.statements[0].kind {
        if let Some(values) = &decl.values {
            assert!(matches!(&values[0].kind, ExprKind::InterpolatedString(_)));
        }
    }
}

#[test]
fn test_interpolation_with_function_call() {
    use luaparse_rs::{Parser, Luau, ast::{StmtKind, ExprKind, InterpolationSegment}};

    let input = r#"local msg = `result: {tostring(x)}`"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    if let StmtKind::LocalDeclaration(decl) = &ast.block.statements[0].kind {
        if let Some(values) = &decl.values {
            if let ExprKind::InterpolatedString(interp) = &values[0].kind {
                if let InterpolationSegment::Expression(expr) = &interp.segments[1] {
                    assert!(matches!(&expr.kind, ExprKind::Call(_)));
                }
            }
        }
    }
}

#[test]
fn test_empty_interpolation_error() {
    use luaparse_rs::{Parser, Luau};

    let input = r#"local msg = `hello {}`"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_err());
}

#[test]
fn test_escaped_braces() {
    use luaparse_rs::{Parser, Luau, ast::{StmtKind, ExprKind, InterpolationSegment}};

    let input = r#"local msg = `literal \{ and \}`"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    if let StmtKind::LocalDeclaration(decl) = &ast.block.statements[0].kind {
        if let Some(values) = &decl.values {
            if let ExprKind::InterpolatedString(interp) = &values[0].kind {
                // Should only have text, no expressions
                assert_eq!(interp.segments.len(), 1);
                assert!(matches!(&interp.segments[0], InterpolationSegment::Text(t) if t.contains('{')));
            }
        }
    }
}

#[test]
fn test_semicolons() {
    use luaparse_rs::{Parser, Luau};

    let input = "local x = 1; local y = 2; print(x)";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();
    assert_eq!(ast.block.statements.len(), 3);
}

#[test]
fn test_trailing_comma_table() {
    use luaparse_rs::{Parser, Luau};

    let input = "local t = { a = 1, b = 2, }";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
}

#[test]
fn test_hex_float() {
    use luaparse_rs::{Parser, Luau};

    let input = "local x = 0x1.8p2";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
}

#[test]
fn test_string_escapes() {
    use luaparse_rs::{Parser, Luau};

    let input = r#"local s = "\x41\u{1F600}\z
        continued""#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
}

#[test]
fn test_numeric_underscores() {
    use luaparse_rs::{Parser, Luau};

    let input = "local n = 1_000_000";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
}

#[test]
fn test_immediate_invocation() {
    use luaparse_rs::{Parser, Luau, ast::StmtKind};

    let input = r#"
        local x = (function()
            return 5
        end)()
    "#;

    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    assert!(matches!(&ast.block.statements[0].kind, StmtKind::LocalDeclaration(_)));
}

#[test]
fn test_parenthesized_call() {
    use luaparse_rs::{Parser, Luau, ast::StmtKind};

    let input = "(print)('hello')";

    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();
}

#[test]
fn test_string_literal_call() {
    use luaparse_rs::{Parser, Luau, ast::{StmtKind, ExprKind}};

    let input = r#"print"hello world""#;

    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    if let StmtKind::CallStatement(expr) = &ast.block.statements[0].kind {
        assert!(matches!(&expr.kind, ExprKind::Call(_)));
    } else {
        panic!("Expected call statement");
    }
}

#[test]
fn test_table_literal_call() {
    use luaparse_rs::{Parser, Luau, ast::StmtKind};

    let input = r#"func{x = 1, y = 2}"#;

    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();
}

#[test]
fn test_chained_literal_calls() {
    use luaparse_rs::{Parser, Luau, ast::{StmtKind, ExprKind}};

    let input = r#"local x = func("str"){}"#;

    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    if let StmtKind::LocalDeclaration(decl) = &ast.block.statements[0].kind {
        if let Some(values) = &decl.values {
            // Outer call with table arg
            if let ExprKind::Call(outer_call) = &values[0].kind {
                // Inner should be call with string arg
                assert!(matches!(&outer_call.function.kind, ExprKind::Call(_)));
            } else {
                panic!("Expected outer call");
            }
        }
    }
}

#[test]
fn test_string_then_table_call() {
    use luaparse_rs::{Parser, Luau};

    let input = r#"func"hello"{x = 1}"#;

    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();
}

#[test]
fn test_multiple_string_calls() {
    use luaparse_rs::{Parser, Luau};

    let input = r#"func"a""b""c""#;

    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();
}

#[test]
fn test_varargs_must_be_last() {
    use luaparse_rs::{Parser, Luau};

    let input = "function f(..., a) end";

    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_err());

    if let Err(e) = result {
        let msg = format!("{:?}", e);
        assert!(msg.contains("must be the last parameter") || msg.contains("varargs"));
    }
}

#[test]
fn test_varargs_multiple_positions() {
    use luaparse_rs::{Parser, Luau};

    let input = "function f(a, ..., b, ...) end";

    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_err());
}

#[test]
fn test_varargs_at_end_valid() {
    use luaparse_rs::{Parser, Luau};

    let input = "function f(a, b, ...) end";

    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
}

#[test]
fn test_long_string_nested() {
    use luaparse_rs::{Parser, Luau};

    let input = r#"
        local s = [=[
            This has [[ nested ]] brackets
        ]=]
    "#;

    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();
}

#[test]
fn test_long_string_different_levels() {
    use luaparse_rs::{Parser, Luau};

    let input = r#"
        local a = [[simple]]
        local b = [=[one equal]=]
        local c = [==[two equals]==]
    "#;

    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
}

#[test]
fn test_unterminated_long_string() {
    use luaparse_rs::{Parser, Luau};

    let input = "local s = [[unterminated";
    let parser = Parser::<Luau>::new(input);

    assert!(parser.is_err());
}

#[test]
fn test_invalid_hex_number() {
    use luaparse_rs::{Parser, Luau};

    let input = "local x = 0x";
    let parser = Parser::<Luau>::new(input);

    assert!(parser.is_err());
}

#[test]
fn test_invalid_exponent() {
    use luaparse_rs::{Parser, Luau};

    let input = "local x = 1e";
    let parser = Parser::<Luau>::new(input);

    assert!(parser.is_err());
}

#[test]
fn test_valid_hex_float() {
    use luaparse_rs::{Parser, Luau};

    let input = "local x = 0x1.8p2";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
}

#[test]
fn test_unicode_identifiers() {
    use luaparse_rs::{Parser, Luau};

    let input = r#"
        local café = 1231
        local 变量 = 123123
        local μ = 1.23456789
    "#;

    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();
}

#[test]
fn test_shebang() {
    use luaparse_rs::{Parser, Luau};

    let input = r#"#!/usr/bin/env lua
    local x = 5
    print(x)"#;

    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
}

#[test]
fn test_shebang_not_at_start() {
    use luaparse_rs::{Parser, Luau};

    let input = r#"
    local x = 5
    #!/usr/bin/env lua"#;

    let parser = Parser::<Luau>::new(input);

    assert!(parser.is_err()); // Shebang must be at start
}