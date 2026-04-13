use luaparse_rs::{Parser, Luau, Lua51, Lua52, Lua53, Lua54};

#[test]
fn test_assign_call_result_field_lua51() {
    let input = r#"foo().bar = 1"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_assign_call_result_index_lua51() {
    let input = r#"foo()[1] = 1"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_assign_method_result_field_lua51() {
    let input = r#"foo:bar().baz = 1"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_assign_chained_call_field_lua51() {
    let input = r#"a.b().c = 1"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_assign_double_call_field_lua51() {
    let input = r#"foo()().bar = 1"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_assign_simple_field_lua51() {
    let input = r#"a.b = 1"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_assign_nested_field_lua51() {
    let input = r#"a.b.c = 1"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_assign_index_lua51() {
    let input = r#"a[1] = 1"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_call_statement_lua51() {
    let input = r#"foo()"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_bare_call_not_assignable_lua51() {
    let input = r#"foo() = 1"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err());
}
