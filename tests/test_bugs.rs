use luaparse::{Parser, Lua51, Luau};

#[test]
fn test_normal_line_comment() {
    let input = "-- this is a comment\nlocal x = 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let ast = parser.parse().unwrap();
    assert!(!ast.comments.is_empty());
}

#[test]
fn test_empty_line_comment() {
    let input = "--\nlocal x = 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Empty line comment should parse: {:?}", result.err());
}

#[test]
fn test_comment_at_eof() {
    let input = "local x = 1\n--";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Comment at EOF should parse: {:?}", result.err());
}

#[test]
fn test_dash_bracket_not_block() {
    let input = "--[not a block\nlocal x = 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "--[ non-block should parse: {:?}", result.err());
}

#[test]
fn test_block_comment() {
    let input = "--[[ block comment ]]\nlocal x = 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let ast = parser.parse().unwrap();
    assert!(!ast.comments.is_empty());
}

#[test]
fn test_comment_only_spaces() {
    let input = "--   \nlocal x = 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Comment with only spaces should parse: {:?}", result.err());
}

#[test]
fn test_typeof_in_luau_type() {
    let input = "local x: typeof(y) = z";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "typeof in type position should parse: {:?}", result.err());
}

#[test]
fn test_native_attribute_function() {
    let input = "@native\nfunction foo()\nend";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "@native function should parse: {:?}", result.err());
}
