use luaparse::{Parser, Luau, Lua51, Lua52, Lua53, Lua54};

#[test]
fn test_const_simple_luau() {
    let input = r#"const x = 5"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Luau should support 'const x = 5': {:?}", result.err());
}

#[test]
fn test_const_multiple_vars_luau() {
    let input = r#"const x, y, z = 1, 2, 3"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Luau should support 'const x, y, z = 1, 2, 3': {:?}", result.err());
}

#[test]
fn test_const_with_type_annotation_luau() {
    let input = r#"const x: number = 5"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Luau should support 'const x: number = 5': {:?}", result.err());
}

#[test]
fn test_const_string_value_luau() {
    let input = r#"const name: string = "hello""#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Luau should support const with string: {:?}", result.err());
}

#[test]
fn test_const_with_expression_luau() {
    let input = r#"const x = 1 + 2 * 3"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Luau should support const with expression: {:?}", result.err());
}

#[test]
fn test_const_mixed_with_local_luau() {
    let input = "local x = 1\nconst y = 2\nlocal z = x + y";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Luau should support mixing local and const: {:?}", result.err());
}

#[test]
fn test_const_in_block_luau() {
    let input = "do\n    const x = 10\n    local y = x + 1\nend";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Luau should support const inside do block: {:?}", result.err());
}

#[test]
fn test_const_in_function_body_luau() {
    let input = "local function foo()\n    const x = 42\n    return x\nend";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Luau should support const inside function body: {:?}", result.err());
}

#[test]
fn test_const_multiple_typed_luau() {
    let input = r#"const x: number, y: string = 1, "hello""#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Luau should support const with multiple typed vars: {:?}", result.err());
}

#[test]
fn test_const_table_value_luau() {
    let input = r#"const t = {1, 2, 3}"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Luau should support const with table value: {:?}", result.err());
}

#[test]
fn test_const_call_value_luau() {
    let input = r#"const x = foo()"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Luau should support const with function call value: {:?}", result.err());
}

#[test]
fn test_const_no_value_fails_luau() {
    let input = r#"const x"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "const without value should fail");
}

#[test]
fn test_const_function_fails_luau() {
    let input = r#"const function foo() end"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "const function should fail");
}

#[test]
fn test_const_as_identifier_lua51() {
    let input = r#"const = 5"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Lua 5.1 should treat 'const' as identifier: {:?}", result.err());
}

#[test]
fn test_const_as_variable_name_lua51() {
    let input = r#"local const = 10"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Lua 5.1 should allow 'const' as variable name: {:?}", result.err());
}

#[test]
fn test_const_as_identifier_lua52() {
    let input = r#"const = 5"#;
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Lua 5.2 should treat 'const' as identifier: {:?}", result.err());
}

#[test]
fn test_const_as_identifier_lua53() {
    let input = r#"const = 5"#;
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Lua 5.3 should treat 'const' as identifier: {:?}", result.err());
}

#[test]
fn test_const_as_identifier_lua54() {
    let input = r#"const = 5"#;
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Lua 5.4 should treat 'const' as identifier: {:?}", result.err());
}

#[test]
fn test_lua54_const_attribute_still_works() {
    let input = r#"local x <const> = 5"#;
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Lua 5.4 should still support <const> attribute: {:?}", result.err());
}

#[test]
fn test_const_sets_is_const_flag_luau() {
    let input = r#"const x = 5"#;
    let ast = Parser::<Luau>::new(input).unwrap().parse().unwrap();
    let stmt = &ast.block.statements[0];
    match &stmt.kind {
        luaparse::ast::StmtKind::LocalDeclaration(decl) => {
            assert!(decl.is_const, "const declaration should have is_const = true");
            assert!(decl.values.is_some(), "const declaration must have values");
        }
        other => panic!("Expected LocalDeclaration, got {:?}", other),
    }
}

#[test]
fn test_local_does_not_set_is_const_luau() {
    let input = r#"local x = 5"#;
    let ast = Parser::<Luau>::new(input).unwrap().parse().unwrap();
    let stmt = &ast.block.statements[0];
    match &stmt.kind {
        luaparse::ast::StmtKind::LocalDeclaration(decl) => {
            assert!(!decl.is_const, "local declaration should have is_const = false");
        }
        other => panic!("Expected LocalDeclaration, got {:?}", other),
    }
}
