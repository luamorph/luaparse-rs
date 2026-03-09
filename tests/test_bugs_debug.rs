use luaparse::{Lua51};

#[test]
fn test_lex_dash_bracket() {
    let input = "--[not a block\nlocal x = 1";
    let result = luaparse::lexer::lex(input);
    assert!(result.is_ok(), "Lexer should handle --[: {:?}", result.err());
}
