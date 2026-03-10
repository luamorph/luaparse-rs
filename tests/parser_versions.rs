#[test]
fn test_continue_as_identifier_lua51() {
    use luaparse_rs::{Parser, Lua51, ast::StmtKind};

    let input = "local continue = 5";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    if let Some(stmt) = ast.block.statements.first() {
        if let StmtKind::LocalDeclaration(decl) = &stmt.kind {
            assert_eq!(decl.names[0].name.name, "continue");
        } else {
            panic!("Expected LocalDeclaration, got {:?}", stmt.kind);
        }
    } else {
        panic!("No statements found");
    }
}

#[test]
fn test_continue_as_keyword_luau() {
    use luaparse_rs::s::s::s::{Parser, Luau, ast::StmtKind};

    let input = "continue";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    assert!(matches!(
        &ast.block.statements[0].kind,
        StmtKind::ContinueStatement
    ));
}

#[test]
fn test_type_as_identifier_lua51() {
    use luaparse_rs::s::s::s::{Parser, Lua51, ast::StmtKind};

    let input = "local type = 'string'";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    if let Some(stmt) = ast.block.statements.first() {
        if let StmtKind::LocalDeclaration(decl) = &stmt.kind {
            assert_eq!(decl.names[0].name.name, "type");
        }
    }
}

#[test]
fn test_export_as_identifier_lua51() {
    use luaparse_rs::s::s::{Parser, Lua51, ast::StmtKind};

    let input = "local export = 10";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    if let Some(stmt) = ast.block.statements.first() {
        if let StmtKind::LocalDeclaration(decl) = &stmt.kind {
            assert_eq!(decl.names[0].name.name, "export");
        }
    }
}

#[test]
fn test_goto_label_lua52() {
    use luaparse_rs::s::{Parser, Lua52, ast::StmtKind};

    let input = r#"
        goto skip
        print("a")
        ::skip::
        print("b")
    "#;

    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    assert!(ast.block.statements.iter().any(|s| matches!(
        s.kind,
        StmtKind::GotoStatement(_)
    )));
    assert!(ast.block.statements.iter().any(|s| matches!(
        s.kind,
        StmtKind::LabelStatement(_)
    )));
}

#[test]
fn test_goto_as_identifier_lua51() {
    use luaparse_rs::{Parser, Lua51, ast::StmtKind};

    let input = "local goto = 'label'";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    if let Some(stmt) = ast.block.statements.first() {
        if let StmtKind::LocalDeclaration(decl) = &stmt.kind {
            assert_eq!(decl.names[0].name.name, "goto");
        }
    }
}

#[test]
fn test_bitwise_ops_lua53() {
    use luaparse_rs::{Parser, Lua53, ast::ExprKind};

    let input = "local x = 5 & 3";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();
}

#[test]
fn test_bitwise_not_lua53() {
    use luaparse_rs::{Parser, Lua53};

    let input = "local x = ~5";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
}

use luaparse_rs::{Parser, Lua52, Lua53, Lua54, Luau, ast::StmtKind};

#[test]
fn test_goto_forward_jump_lua52() {
    let input = r#"
        goto skip
        print("never")
        ::skip::
        print("always")
    "#;

    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    let has_goto = ast.block.statements.iter()
        .any(|s| matches!(&s.kind, StmtKind::GotoStatement(_)));
    let has_label = ast.block.statements.iter()
        .any(|s| matches!(&s.kind, StmtKind::LabelStatement(_)));

    assert!(has_goto);
    assert!(has_label);
}

#[test]
fn test_goto_fails_lua51() {
    use luaparse_rs::s::Lua51;

    let input = "goto label";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();

    // "goto label" is two identifiers - invalid statement syntax
    assert!(result.is_err());
}

#[test]
fn test_bitwise_precedence_lua53() {
    use luaparse_rs::s::ast::{ExprKind, BinaryOperator};

    // 1 | 2 & 3 should parse as 1 | (2 & 3)
    let input = "local x = 1 | 2 & 3";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    if let StmtKind::LocalDeclaration(decl) = &ast.block.statements[0].kind {
        if let Some(values) = &decl.values {
            if let ExprKind::Binary(binary) = &values[0].kind {
                assert_eq!(binary.operator, BinaryOperator::BitwiseOr);
                // right side should be 2 & 3
                if let ExprKind::Binary(right_binary) = &binary.right.kind {
                    assert_eq!(right_binary.operator, BinaryOperator::BitwiseAnd);
                } else {
                    panic!("expected & on right side");
                }
            }
        }
    }
}

#[test]
fn test_bitwise_fails_lua52() {
    let input = "local x = 5 & 3";
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();

    // lexer  should fail or parser should reject
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e, luaparse_rs::s::ParseError::UnsupportedFeature { .. }));
    }
}

#[test]
fn test_const_attribute_lua54() {
    use luaparse_rs::ast::StmtKind;

    let input = "local x <const> = 5";
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    if let StmtKind::LocalDeclaration(decl) = &ast.block.statements[0].kind {
        assert_eq!(decl.names[0].name.name, "x");
        assert!(decl.names[0].attribute.is_some());
        assert_eq!(decl.names[0].attribute.as_ref().unwrap().name, "const");
    }
}

#[test]
fn test_close_attribute_lua54() {
    let input = "local f <close> = io.open('test')";
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    if let StmtKind::LocalDeclaration(decl) = &ast.block.statements[0].kind {
        assert!(decl.names[0].attribute.is_some());
        assert_eq!(decl.names[0].attribute.as_ref().unwrap().name, "close");
    }
}

#[test]
fn test_export_type_luau() {
    let input = "export type MyType = string";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse_with_types();

    assert!(result.is_ok());
    let ast = result.unwrap();

    assert_eq!(ast.type_declarations.len(), 1);
    assert!(ast.type_declarations[0].exported);
}

#[test]
fn test_export_function_luau() {
    let input = "export function test() end";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    assert!(matches!(
        &ast.block.statements[0].kind,
        StmtKind::ExportStatement(_)
    ));
}
