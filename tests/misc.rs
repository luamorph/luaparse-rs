use luaparse_rs::{Parser, Luau, Lua51, Lua52, Lua53, Lua54};

#[test]
fn test_hex_literal_lua51() {
    let input = r#"local x = 0xFF"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "hex literal: {:?}", result);
}

#[test]
fn test_hex_float_lua53() {
    let input = r#"local x = 0x1.Fp10"#;
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "hex float: {:?}", result);
}

#[test]
fn test_binary_literal_luau() {
    let input = r#"local x = 0b1010"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "binary literal: {:?}", result);
}

#[test]
fn test_underscore_in_number_luau() {
    let input = r#"local x = 1_000_000"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "underscore number: {:?}", result);
}

#[test]
fn test_long_string_level0_lua51() {
    let input = r#"local x = [[hello]]"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "long string l0: {:?}", result);
}

#[test]
fn test_long_string_level1_lua51() {
    let input = r#"local x = [=[hello]=]"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "long string l1: {:?}", result);
}

#[test]
fn test_long_string_level2_lua51() {
    let input = r#"local x = [==[hello]==]"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "long string l2: {:?}", result);
}

#[test]
fn test_escape_sequences_lua53() {
    let input = r#"local x = "\a\b\f\n\r\t\v\\\"\'""#;
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "escape sequences: {:?}", result);
}

#[test]
fn test_hex_escape_lua53() {
    let input = r#"local x = "\x41""#;
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "hex escape: {:?}", result);
}

#[test]
fn test_unicode_escape_lua53() {
    let input = r#"local x = "\u{1F600}""#;
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "unicode escape: {:?}", result);
}

#[test]
fn test_interpolated_string_luau() {
    let input = "local x = `hello {name}`";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "interpolated string: {:?}", result);
}

#[test]
fn test_interpolated_string_nested_luau() {
    let input = "local x = `a {b} c {d}`";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "nested interpolated: {:?}", result);
}

#[test]
fn test_semicolons_lua51() {
    let input = r#"local x = 1; local y = 2;"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "semicolons: {:?}", result);
}

#[test]
fn test_empty_semicolons_lua52() {
    let input = r#";;;"#;
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "empty semicolons: {:?}", result);
}

#[test]
fn test_empty_do_block_lua51() {
    let input = r#"do end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "empty do: {:?}", result);
}

#[test]
fn test_nested_do_blocks_lua51() {
    let input = r#"do do do end end end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "nested do: {:?}", result);
}

#[test]
fn test_break_in_while_lua51() {
    let input = r#"while true do break end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "break in while: {:?}", result);
}

#[test]
fn test_continue_in_while_luau() {
    let input = r#"while true do continue end"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "continue in while: {:?}", result);
}

#[test]
fn test_continue_rejected_lua51() {
    let input = r#"while true do continue end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    // continue is NOT in standard Lua 5.1
    assert!(result.is_err(), "continue should fail on 5.1: {:?}", result);
}

#[test]
fn test_goto_lua52() {
    let input = r#"goto done ::done::"#;
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "goto 5.2: {:?}", result);
}

#[test]
fn test_goto_rejected_lua51() {
    let input = r#"goto done"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "goto should fail on 5.1: {:?}", result);
}

#[test]
fn test_label_rejected_lua51() {
    let input = r#"::done::"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "labels should fail on 5.1: {:?}", result);
}

#[test]
fn test_numeric_for_lua51() {
    let input = r#"for i = 1, 10 do end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "numeric for: {:?}", result);
}

#[test]
fn test_numeric_for_with_step_lua51() {
    let input = r#"for i = 1, 10, 2 do end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "numeric for with step: {:?}", result);
}

#[test]
fn test_generic_for_lua51() {
    let input = r#"for k, v in pairs(t) do end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "generic for: {:?}", result);
}

#[test]
fn test_generic_for_ipairs_lua51() {
    let input = r#"for i, v in ipairs(t) do end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "generic for ipairs: {:?}", result);
}

#[test]
fn test_multi_assign_lua51() {
    let input = r#"a, b, c = 1, 2, 3"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "multi assign: {:?}", result);
}

#[test]
fn test_multi_local_lua51() {
    let input = r#"local a, b, c = 1, 2, 3"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "multi local: {:?}", result);
}

#[test]
fn test_multi_return_lua51() {
    let input = r#"local function f() return 1, 2, 3 end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "multi return: {:?}", result);
}

#[test]
fn test_empty_table_lua51() {
    let input = r#"local t = {}"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "empty table: {:?}", result);
}

#[test]
fn test_table_list_lua51() {
    let input = r#"local t = {1, 2, 3}"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "table list: {:?}", result);
}

#[test]
fn test_table_record_lua51() {
    let input = r#"local t = {a = 1, b = 2}"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "table record: {:?}", result);
}

#[test]
fn test_table_mixed_lua51() {
    let input = r#"local t = {1, a = 2, [3] = 4}"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "table mixed: {:?}", result);
}

#[test]
fn test_table_trailing_comma_lua51() {
    let input = r#"local t = {1, 2, 3,}"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "table trailing comma: {:?}", result);
}

#[test]
fn test_table_trailing_semicolon_lua51() {
    let input = r#"local t = {1; 2; 3;}"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "table trailing semicolon: {:?}", result);
}

#[test]
fn test_table_nested_lua51() {
    let input = r#"local t = {{1, 2}, {3, 4}}"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "table nested: {:?}", result);
}

#[test]
fn test_table_function_value_lua51() {
    let input = r#"local t = {f = function() end}"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "table function value: {:?}", result);
}

#[test]
fn test_concat_right_assoc_lua51() {
    let input = r#"local x = "a" .. "b" .. "c""#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "concat right assoc: {:?}", result);
}

#[test]
fn test_power_right_assoc_lua51() {
    let input = r#"local x = 2 ^ 3 ^ 4"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "power right assoc: {:?}", result);
}

#[test]
fn test_not_operator_lua51() {
    let input = r#"local x = not true"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "not operator: {:?}", result);
}

#[test]
fn test_length_operator_lua51() {
    let input = r#"local x = #t"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "length operator: {:?}", result);
}

#[test]
fn test_unary_minus_lua51() {
    let input = r#"local x = -1"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "unary minus: {:?}", result);
}

#[test]
fn test_double_unary_lua51() {
    let input = r#"local x = - -1"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "double unary: {:?}", result);
}

#[test]
fn test_not_not_lua51() {
    let input = r#"local x = not not true"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "not not: {:?}", result);
}

#[test]
fn test_bitwise_ops_lua53() {
    let input = r#"local x = 1 & 2 | 3 ~ 4 << 5 >> 6"#;
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "bitwise ops 5.3: {:?}", result);
}

#[test]
fn test_bitwise_not_lua53() {
    let input = r#"local x = ~1"#;
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "bitwise not 5.3: {:?}", result);
}

#[test]
fn test_bitwise_rejected_lua51() {
    let input = r#"local x = 1 & 2"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "bitwise should fail on 5.1: {:?}", result);
}

#[test]
fn test_immediate_call_function_lua51() {
    let input = r#"local x = (function() return 1 end)()"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "immediate call: {:?}", result);
}

#[test]
fn test_function_no_args_lua51() {
    let input = r#"local f = function() end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "function no args: {:?}", result);
}

#[test]
fn test_function_varargs_lua51() {
    let input = r#"local f = function(...) return ... end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "function varargs: {:?}", result);
}

#[test]
fn test_function_mixed_args_varargs_lua51() {
    let input = r#"local f = function(a, b, ...) end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "function mixed varargs: {:?}", result);
}

#[test]
fn test_comment_after_statement_lua51() {
    let input = "local x = 1 -- comment\nlocal y = 2";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "comment after statement: {:?}", result);
}

#[test]
fn test_block_comment_multiline_lua51() {
    let input = "local x = 1 --[[\nmultiline\ncomment\n]] local y = 2";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "block comment multiline: {:?}", result);
}

#[test]
fn test_comment_in_table_lua51() {
    let input = "local t = {\n-- comment\n1,\n-- another\n2\n}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "comment in table: {:?}", result);
}

#[test]
fn test_block_comment_in_args_lua51() {
    let input = "foo(--[[ comment ]] 1, 2)";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "block comment in args: {:?}", result);
}

#[test]
fn test_comment_between_statements_lua51() {
    let input = "local x = 1\n--[[ block ]]\nlocal y = 2";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "comment between statements: {:?}", result);
}

#[test]
fn test_if_elseif_else_lua51() {
    let input = r#"if a then b() elseif c then d() else e() end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "if/elseif/else: {:?}", result);
}

#[test]
fn test_multiple_elseif_lua51() {
    let input = r#"if a then elseif b then elseif c then else end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "multiple elseif: {:?}", result);
}

#[test]
fn test_nested_if_lua51() {
    let input = r#"if a then if b then end end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "nested if: {:?}", result);
}

#[test]
fn test_repeat_until_lua51() {
    let input = r#"repeat x = x + 1 until x > 10"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "repeat until: {:?}", result);
}

#[test]
fn test_repeat_empty_lua51() {
    let input = r#"repeat until true"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "repeat empty: {:?}", result);
}

#[test]
fn test_method_call_lua51() {
    let input = r#"obj:method()"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "method call: {:?}", result);
}

#[test]
fn test_chained_method_calls_lua51() {
    let input = r#"obj:a():b():c()"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "chained methods: {:?}", result);
}

#[test]
fn test_method_definition_lua51() {
    let input = r#"function obj:method() end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "method definition: {:?}", result);
}

#[test]
fn test_string_call_lua51() {
    let input = r#"print "hello""#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "string call: {:?}", result);
}

#[test]
fn test_table_call_lua51() {
    let input = r#"foo {1, 2, 3}"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "table call: {:?}", result);
}

#[test]
fn test_string_method_call_lua51() {
    let input = r#"obj:method "hello""#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "string method call: {:?}", result);
}

#[test]
fn test_compound_add_luau() {
    let input = r#"x += 1"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "compound add: {:?}", result);
}

#[test]
fn test_compound_concat_luau() {
    let input = r#"x ..= "hello""#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "compound concat: {:?}", result);
}

#[test]
fn test_compound_rejected_lua51() {
    let input = r#"x += 1"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "compound should fail on 5.1: {:?}", result);
}

#[test]
fn test_if_expression_luau() {
    let input = r#"local x = if a then b else c"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "if expression: {:?}", result);
}

#[test]
fn test_if_expression_elseif_luau() {
    let input = r#"local x = if a then b elseif c then d else e"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "if expression elseif: {:?}", result);
}

#[test]
fn test_type_alias_luau() {
    let input = r#"type Point = {x: number, y: number}"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "type alias: {:?}", result);
}

#[test]
fn test_type_function_luau() {
    let input = r#"type Callback = (number, string) -> boolean"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "type function: {:?}", result);
}

#[test]
fn test_type_union_luau() {
    let input = r#"local x: string | number = 1"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "type union: {:?}", result);
}

#[test]
fn test_type_intersection_luau() {
    let input = r#"local x: A & B = {}"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "type intersection: {:?}", result);
}

#[test]
fn test_type_optional_luau() {
    let input = r#"local x: number? = nil"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "type optional: {:?}", result);
}

#[test]
fn test_export_type_luau() {
    let input = r#"export type Foo = number"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "export type: {:?}", result);
}

#[test]
fn test_type_cast_luau() {
    let input = r#"local x = y :: number"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "type cast: {:?}", result);
}

#[test]
fn test_type_rejected_lua51() {
    let input = r#"local x: number = 1"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "types should fail on 5.1: {:?}", result);
}

#[test]
fn test_paren_expr_lua51() {
    let input = r#"local x = (1 + 2) * 3"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "paren expr: {:?}", result);
}

#[test]
fn test_nested_parens_lua51() {
    let input = r#"local x = ((((1))))"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "nested parens: {:?}", result);
}

#[test]
fn test_complex_expr_lua51() {
    let input = r#"local x = a and b or c and d"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "complex and/or: {:?}", result);
}

#[test]
fn test_all_comparison_ops_lua51() {
    let input = r#"local a, b, c, d, e, f = 1<2, 1>2, 1<=2, 1>=2, 1==2, 1~=2"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "all comparisons: {:?}", result);
}

#[test]
fn test_varargs_in_expression_lua51() {
    let input = r#"local function f(...) local x = ... end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "varargs in expr: {:?}", result);
}

#[test]
fn test_varargs_in_call_lua51() {
    let input = r#"local function f(...) print(...) end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "varargs in call: {:?}", result);
}

#[test]
fn test_varargs_in_table_lua51() {
    let input = r#"local function f(...) local t = {...} end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "varargs in table: {:?}", result);
}

#[test]
fn test_multiline_function_call_lua51() {
    let input = "foo(\n1,\n2,\n3\n)";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "multiline call: {:?}", result);
}

#[test]
fn test_multiline_table_lua51() {
    let input = "local t = {\n1,\n2,\n3\n}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "multiline table: {:?}", result);
}

#[test]
fn test_multiline_chain_in_local_lua51() {
    let input = "local x = a\n.b\n.c\n.d";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "multiline chain in local: {:?}", result);
}

#[test]
fn test_oop_pattern_lua51() {
    let input = r#"
local Class = {}
Class.__index = Class

function Class.new(x, y)
    local self = setmetatable({}, Class)
    self.x = x
    self.y = y
    return self
end

function Class:getX()
    return self.x
end
"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "oop pattern: {:?}", result);
}

#[test]
fn test_module_pattern_lua51() {
    let input = r#"
local M = {}

function M.create()
    return {}
end

return M
"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "module pattern: {:?}", result);
}

#[test]
fn test_complex_luau_code() {
    let input = r#"
type Config = {
    name: string,
    value: number?,
    callback: ((string) -> boolean)?,
}

local function process<T>(items: {T}, filter: (T) -> boolean): {T}
    local result: {T} = {}
    for _, item in items do
        if filter(item) then
            table.insert(result, item)
        end
    end
    return result
end

export type Result<T> = {
    ok: boolean,
    value: T?,
    error: string?,
}
"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "complex luau: {:?}", result);
}

#[test]
fn test_integer_division_lua54() {
    let input = r#"local x = 10 // 3"#;
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "integer division 5.4: {:?}", result);
}

#[test]
fn test_attributes_lua54() {
    let input = r#"local x <const> = 1"#;
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "attributes 5.4: {:?}", result);
}

#[test]
fn test_close_attribute_lua54() {
    let input = r#"local f <close> = io.open("file")"#;
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "close attribute 5.4: {:?}", result);
}

#[test]
fn test_attributes_rejected_lua51() {
    let input = r#"local x <const> = 1"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "attributes should fail on 5.1: {:?}", result);
}

#[test]
fn test_empty_program_lua51() {
    let input = r#""#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "empty program: {:?}", result);
}

#[test]
fn test_only_comments_lua51() {
    let input = "-- just a comment\n-- another comment";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "only comments: {:?}", result);
}

#[test]
fn test_only_block_comment_lua51() {
    let input = "--[[\nblock comment\n]]";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "only block comment: {:?}", result);
}

#[test]
fn test_nested_field_assign_lua51() {
    let input = r#"a.b.c.d = 1"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "nested field assign: {:?}", result);
}

#[test]
fn test_mixed_access_assign_lua51() {
    let input = r#"a.b[1].c["d"] = 1"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "mixed access assign: {:?}", result);
}

#[test]
fn test_return_nothing_lua51() {
    let input = r#"return"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "return nothing: {:?}", result);
}

#[test]
fn test_return_nil_lua51() {
    let input = r#"return nil"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "return nil: {:?}", result);
}

#[test]
fn test_return_multiple_lua51() {
    let input = r#"return 1, 2, 3"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "return multiple: {:?}", result);
}

#[test]
fn test_nested_functions_lua51() {
    let input = r#"
local function outer()
    local function inner()
        return 1
    end
    return inner()
end
"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "nested functions: {:?}", result);
}

#[test]
fn test_deeply_nested_table_lua51() {
    let input = r#"local t = {a = {b = {c = {d = 1}}}}"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "deeply nested table: {:?}", result);
}

#[test]
fn test_long_field_chain_lua51() {
    let input = r#"local x = a.b.c.d.e.f.g"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "long field chain: {:?}", result);
}

#[test]
fn test_operator_precedence_lua51() {
    let input = r#"local x = 1 + 2 * 3 - 4 / 5 % 6 ^ 7"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "operator precedence: {:?}", result);
}

#[test]
fn test_for_in_no_pairs_luau() {
    let input = r#"for _, v in t do end"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "for-in without pairs luau: {:?}", result);
}

#[test]
fn test_multiple_statements_one_line_lua51() {
    let input = r#"local x = 1 local y = 2 local z = 3"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "multiple statements one line: {:?}", result);
}

#[test]
fn test_many_params_lua51() {
    let input = r#"local function f(a, b, c, d, e, f, g, h, i, j) end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "many params: {:?}", result);
}

#[test]
fn test_chained_and_or_lua51() {
    let input = r#"local x = a > 0 and a < 10 or a == -1"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "chained and/or: {:?}", result);
}

#[test]
fn test_type_generic_table_luau() {
    let input = r#"type Map<K, V> = {[K]: V}"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "generic table type: {:?}", result);
}

#[test]
fn test_nested_if_expression_luau() {
    let input = r#"local x = if a then if b then 1 else 2 else 3"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "nested if expression: {:?}", result);
}

#[test]
fn test_decimal_escape_lua51() {
    let input = r#"local x = "\97""#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "decimal escape: {:?}", result);
}

#[test]
fn test_return_trailing_call_lua51() {
    let input = r#"local function f() return 1, foo() end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "return trailing call: {:?}", result);
}

#[test]
fn test_while_complex_condition_lua51() {
    let input = r#"while x > 0 and y < 10 do x = x - 1 end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "while complex condition: {:?}", result);
}

#[test]
fn test_table_computed_key_lua51() {
    let input = r#"local t = {[1+2] = "three", [f()] = "result"}"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "table computed key: {:?}", result);
}

#[test]
fn test_integer_format_lua53() {
    let input = r#"local x = 0xDEADBEEF"#;
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "integer format 5.3: {:?}", result);
}
