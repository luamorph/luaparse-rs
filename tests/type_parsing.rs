use luaparse_rs::{Parser, Luau};

#[test]
fn test_simple_type_alias() {
    let input = "type Name = string";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse_with_types();

    assert!(result.is_ok());
    let ast = result.unwrap();
    assert_eq!(ast.type_declarations.len(), 1);
}

#[test]
fn test_table_type() {
    let input = "type Person = { name: string, age: number }";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse_with_types();

    assert!(result.is_ok());
    let ast = result.unwrap();
}

#[test]
fn test_function_type() {
    let input = "type Handler = (x: number, y: string) -> boolean";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse_with_types();

    assert!(result.is_ok());
    let ast = result.unwrap();
}

#[test]
fn test_generic_type() {
    let input = "type Array<T> = { [number]: T }";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse_with_types();

    assert!(result.is_ok());
    let ast = result.unwrap();
}

#[test]
fn test_union_type() {
    let input = "type Result = number | string | nil";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse_with_types();

    assert!(result.is_ok());
    let ast = result.unwrap();
}

#[test]
fn test_typeof() {
    let input = "type MyType = typeof(someValue)";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse_with_types();

    assert!(result.is_ok());
    let ast = result.unwrap();
}

#[test]
fn test_const_attribute_lua54() {
    use luaparse_rs::{Parser, Lua54, ast::StmtKind};

    let input = "local x <const> = 5";
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();

    if let StmtKind::LocalDeclaration(decl) = &ast.block.statements[0].kind {
        assert_eq!(decl.names[0].name.name, "x");
    }
}

#[test]
fn test_close_attribute_lua54() {
    use luaparse_rs::{Parser, Lua54};

    let input = "local f <close> = io.open('file')";
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();

    assert!(result.is_ok());
    let ast = result.unwrap();
}
