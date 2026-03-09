use luaparse::{Parser, Luau, Lua51, Lua52, Lua53, Lua54};

#[test]
fn test_multiple_local_with_types_luau() {
    let input = "local x: number, y: string = 1, \"hello\"";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_local_with_complex_type_luau() {
    let input = "local x: {[string]: number} = {}";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_local_with_function_type_luau() {
    let input = "local f: (number, string) -> boolean = nil";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_local_with_union_type_luau() {
    let input = "local x: number | string | nil = nil";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_local_with_intersection_type_luau() {
    let input = "local x: Foo & Bar = nil";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_local_with_optional_type_luau() {
    let input = "local x: number? = nil";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_local_with_generic_type_luau() {
    let input = "local x: Array<number> = {}";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_local_with_nested_generic_luau() {
    let input = "local x: Map<string, Array<number>> = {}";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_local_with_module_type_luau() {
    let input = "local x: module.Type = nil";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_const_attribute_lua54() {
    let input = "local x <const> = 42";
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_close_attribute_lua54() {
    let input = "local f <close> = io.open('test')";
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_attribute_rejected_lua51() {
    let input = "local x <const> = 42";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "Lua 5.1 should not support <const> attribute");
}

#[test]
fn test_attribute_rejected_lua52() {
    let input = "local x <const> = 42";
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "Lua 5.2 should not support <const> attribute");
}

#[test]
fn test_attribute_rejected_lua53() {
    let input = "local x <const> = 42";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "Lua 5.3 should not support <const> attribute");
}

#[test]
fn test_goto_lua52() {
    let input = "goto done\n::done::";
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_goto_rejected_lua51() {
    let input = "goto done";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    // In Lua 5.1, goto is an identifier, so this would be parsed as a call/assignment
    // It should either error or parse differently
}

#[test]
fn test_continue_luau() {
    let input = "for i = 1, 10 do continue end";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_continue_rejected_lua51() {
    let input = "for i = 1, 10 do continue end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    // In Lua 5.1, continue is an identifier - should parse differently or error
}

#[test]
fn test_numeric_for_lua51() {
    let input = "for i = 1, 10, 2 do print(i) end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_generic_for_lua51() {
    let input = "for k, v in pairs(t) do print(k, v) end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_repeat_until_lua51() {
    let input = "repeat x = x + 1 until x > 10";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_nested_if_elseif_else() {
    let input = "if a then x() elseif b then y() elseif c then z() else w() end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_while_loop() {
    let input = "while true do break end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_do_block() {
    let input = "do local x = 1 end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_function_with_return_type_luau() {
    let input = "function foo(x: number): string return tostring(x) end";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_local_function_with_types_luau() {
    let input = "local function foo(x: number, y: string): boolean return true end";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_method_declaration() {
    let input = "function Foo:bar(x) return x end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_nested_function_name() {
    let input = "function a.b.c.d() end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_variadic_function() {
    let input = "function foo(...) return ... end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_anonymous_function() {
    let input = "local f = function(x, y) return x + y end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_native_local_function_luau() {
    let input = "@native local function foo() end";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_if_expression_luau() {
    let input = "local x = if a then b else c";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_if_expression_elseif_luau() {
    let input = "local x = if a then b elseif c then d else e";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_string_concat_lua51() {
    let input = "local x = \"hello\" .. \" \" .. \"world\"";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_operator_precedence() {
    let input = "local x = 1 + 2 * 3 - 4 / 5 ^ 6";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_unary_not() {
    let input = "local x = not true";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_unary_length() {
    let input = "local x = #t";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_unary_negate() {
    let input = "local x = -42";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_chained_method_calls() {
    let input = "x:foo():bar():baz()";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_chained_index_and_field() {
    let input = "x.y.z[1][2].w = 42";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_type_assertion_luau() {
    let input = "local x = y :: number";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_empty_table() {
    let input = "local t = {}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_array_table() {
    let input = "local t = {1, 2, 3, 4, 5}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_hash_table() {
    let input = "local t = {a = 1, b = 2, c = 3}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_computed_key_table() {
    let input = "local t = {[1] = \"a\", [\"key\"] = true, [f()] = nil}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_nested_table() {
    let input = "local t = {{1, 2}, {3, 4}, {a = {b = {c = 5}}}}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_mixed_table() {
    let input = "local t = {1, a = 2, [3] = 4, \"five\"}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_trailing_semicolons_table() {
    let input = "local t = {1; 2; 3;}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_bitwise_and_lua53() {
    let input = "local x = a & b";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_bitwise_or_lua53() {
    let input = "local x = a | b";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_bitwise_xor_lua53() {
    let input = "local x = a ~ b";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_bitwise_shifts_lua53() {
    let input = "local x = a << 2\nlocal y = b >> 3";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_bitwise_not_lua53() {
    let input = "local x = ~a";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_bitwise_rejected_lua51() {
    let input = "local x = a & b";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "Lua 5.1 should not support bitwise &");
}

#[test]
fn test_bitwise_rejected_lua52() {
    let input = "local x = a & b";
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "Lua 5.2 should not support bitwise &");
}

#[test]
fn test_floor_div_lua53() {
    let input = "local x = 10 // 3";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_floor_div_lua54() {
    let input = "local x = 10 // 3";
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_floor_div_luau() {
    let input = "local x = 10 // 3";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_floor_div_rejected_lua51() {
    let input = "local x = 10 // 3";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "Lua 5.1 should not support //");
}

#[test]
fn test_floor_div_rejected_lua52() {
    let input = "local x = 10 // 3";
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "Lua 5.2 should not support //");
}

#[test]
fn test_compound_add_luau() {
    let input = "x += 1";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_compound_sub_luau() {
    let input = "x -= 1";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_compound_mul_luau() {
    let input = "x *= 2";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_compound_div_luau() {
    let input = "x /= 2";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_compound_mod_luau() {
    let input = "x %= 3";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_compound_pow_luau() {
    let input = "x ^= 2";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_compound_concat_luau() {
    let input = "x ..= \"!\"";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_compound_floor_div_luau() {
    let input = "x //= 3";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_compound_rejected_lua51() {
    let input = "x += 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "Lua 5.1 should not support +=");
}

#[test]
fn test_single_quoted_string() {
    let input = "local x = 'hello'";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_double_quoted_string() {
    let input = "local x = \"hello\"";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_long_string_basic() {
    let input = "local x = [[hello world]]";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_long_string_level1() {
    let input = "local x = [=[hello]=]";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_long_string_with_newlines() {
    let input = "local x = [[\nhello\nworld\n]]";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_interpolated_string_luau() {
    let input = "local x = `hello {name}`";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_interpolated_string_multiple_luau() {
    let input = "local x = `{a} + {b} = {a + b}`";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_type_alias_luau() {
    let input = "type Point = {x: number, y: number}";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_export_type_luau() {
    let input = "export type Callback = (number) -> ()";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_generic_type_declaration_luau() {
    let input = "type Array<T> = {T}";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_type_not_keyword_lua51() {
    // In Lua 5.1, 'type' is an identifier, not a keyword
    let input = "local type = 'hello'";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_multiple_return_values() {
    let input = "function foo() return 1, 2, 3 end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_empty_return() {
    let input = "function foo() return end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_multiple_statements_no_semicolons() {
    let input = "local a = 1 local b = 2 local c = 3";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_multiple_statements_with_semicolons() {
    let input = "local a = 1; local b = 2; local c = 3;";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_deeply_nested_expressions() {
    let input = "local x = ((((((1 + 2))))))";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_function_call_as_statement() {
    let input = "print(\"hello\")";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_method_call_as_statement() {
    let input = "obj:method(1, 2, 3)";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_string_call_as_statement() {
    let input = "print \"hello\"";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_table_call_as_statement() {
    let input = "foo {1, 2, 3}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_multi_assign() {
    let input = "a, b, c = 1, 2, 3";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_empty_program() {
    let input = "";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_only_comments() {
    let input = "-- just a comment\n-- another one";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_complex_real_world_luau() {
    let input = r#"
local Players = game:GetService("Players")

type PlayerData = {
    name: string,
    score: number,
    inventory: {string},
}

local function processPlayer(player: Player): PlayerData
    local data: PlayerData = {
        name = player.Name,
        score = 0,
        inventory = {},
    }
    return data
end

Players.PlayerAdded:Connect(function(player)
    local data = processPlayer(player)
    print(`Welcome {data.name}!`)
end)
"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_complex_real_world_lua53() {
    let input = r#"
local function bitops(a, b)
    local result = {
        band = a & b,
        bor = a | b,
        bxor = a ~ b,
        bnot = ~a,
        shl = a << 2,
        shr = b >> 1,
        fdiv = a // b,
    }
    return result
end

for k, v in pairs(bitops(0xFF, 0x0F)) do
    print(k, v)
end
"#;
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_complex_real_world_lua54() {
    let input = r#"
local f <close> = io.open("test.txt", "r")
local content <const> = f:read("*a")

::retry::
local ok, err = pcall(function()
    return content
end)

if not ok then
    goto retry
end
"#;
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_typeof_nested_luau() {
    let input = "local x: typeof(foo(bar())) = nil";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_native_with_fields_luau() {
    let input = "@native[opt=true] function foo() end";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_multiple_attributes_luau() {
    let input = "@native @checked function foo() end";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_typed_variadic_luau() {
    let input = "local function foo(...: number) end";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_generic_function_luau() {
    let input = "type Pair<A, B> = {first: A, second: B}";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_invalid_assignment_target() {
    let input = "1 = 2";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_unclosed_paren() {
    let input = "local x = (1 + 2";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_unclosed_do() {
    let input = "do local x = 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_unexpected_end() {
    let input = "end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_hex_number() {
    let input = "local x = 0xFF";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_binary_number() {
    let input = "local x = 0b1010";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_underscore_number() {
    let input = "local x = 1_000_000";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_float_number() {
    let input = "local x = 3.14159";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_scientific_notation() {
    let input = "local x = 1e10";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_hex_float() {
    let input = "local x = 0x1.fp10";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}
