use luaparse_rs::{Lua53, Parser};

fn parse_lua53(source: &str) {
    let parser = Parser::<Lua53>::new(source).unwrap();
    parser.parse().unwrap();
}

#[test]
fn parses_api_decimal_escape_in_string() {
    parse_lua53(r#"local signature = "\27Lua""#);
}


#[test]
fn parses_calls_luac_header_string() {
    parse_lua53(r#"
        local header = {
            "\27Lua",
            5*16 + 3,
            0x5678,
        }
    "#);
}

#[test]
fn parses_comment_between_binary_operator_and_rhs() {
    parse_lua53(r#"
        local function f(...) return ... end
        assert(f(1,2,'a')
        ~=          -- force SETLINE before nil
        nil, "")
    "#);
}

#[test]
fn parses_empty_function_call_parenthesized() {
    parse_lua53(r#"
        local function dostring(x) return assert(load(x), "")() end
    "#);
}

#[test]
fn parses_hash_first_line_comment() {
    parse_lua53("# testing special comment on first line\nprint('ok')");
}

#[test]
fn parses_leading_zero_decimal_literal() {
    parse_lua53("assert(8 == 08)");
}
