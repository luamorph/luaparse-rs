use luaparse_rs::{Parser, Luau, Lua51, Lua52, Lua53, Lua54};

#[test]
fn test_triple_nested_generic_luau() {
    let input = "local x: A<B<C<number>>> = {}";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_generic_followed_by_eq_luau() {
    // This is a type annotation context, not assignment
    let input = "type Foo = Bar<number>";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse_with_types();
    assert!(result.is_ok());
}

#[test]
fn test_string_interpolation_basic_luau() {
    let input = "local x = `hello {name}`";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_string_interpolation_expr_luau() {
    let input = "local x = `{1 + 2} items`";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_string_interpolation_nested_luau() {
    let input = "local x = `hello {`world`}`";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_string_interpolation_empty_luau() {
    let input = "local x = `hello`";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_if_expression_in_assignment_luau() {
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
fn test_if_expression_nested_luau() {
    let input = "local x = if a then (if b then c else d) else e";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_continue_in_for_luau() {
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
    assert!(result.is_err());
}

#[test]
fn test_compound_add_assign_luau() {
    let input = "x += 1";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_compound_sub_assign_luau() {
    let input = "x -= 1";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_compound_mul_assign_luau() {
    let input = "x *= 2";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_compound_div_assign_luau() {
    let input = "x /= 2";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_compound_mod_assign_luau() {
    let input = "x %= 2";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_compound_pow_assign_luau() {
    let input = "x ^= 2";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_compound_concat_assign_luau() {
    let input = "x ..= 'hello'";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_compound_floor_div_assign_luau() {
    let input = "x //= 2";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_compound_assign_rejected_lua51() {
    let input = "x += 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_compound_assign_rejected_lua54() {
    let input = "x += 1";
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_method_chain_luau() {
    let input = "x:foo():bar():baz()";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_method_with_string_arg_luau() {
    let input = "x:foo 'hello'";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_method_with_table_arg_luau() {
    let input = "x:foo { a = 1 }";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_multiline_function_call_luau() {
    let input = "foo(\n  1,\n  2,\n  3\n)";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_chained_index_and_call_luau() {
    let input = "a.b.c[1](2).d:e(3)";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_union_type_annotation_luau() {
    let input = "local x: string | number | nil = nil";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_intersection_type_annotation_luau() {
    let input = "local x: A & B & C = {}";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_function_type_annotation_luau() {
    let input = "local x: (number, string) -> boolean = fn";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_optional_type_annotation_luau() {
    let input = "local x: string? = nil";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_table_type_annotation_luau() {
    let input = "local x: { name: string, age: number } = {}";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_array_type_annotation_luau() {
    let input = "local x: {number} = {}";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_generic_function_type_luau() {
    let input = "local x: <T>(T) -> T = fn";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_typeof_in_type_annotation_luau() {
    let input = "local x: typeof(someValue) = y";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_function_return_type_luau() {
    let input = "local function foo(): number return 1 end";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_function_multi_return_type_luau() {
    let input = "local function foo(): (number, string) return 1, 'a' end";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_function_generic_params_luau() {
    let input = "local function foo<T, U>(x: T, y: U): T return x end";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_type_alias_luau() {
    let input = "type Foo = number";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse_with_types();
    assert!(result.is_ok());
}

#[test]
fn test_type_generic_declaration_luau() {
    let input = "type Map<K, V> = { [K]: V }";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse_with_types();
    assert!(result.is_ok());
}

#[test]
fn test_export_type_luau() {
    let input = "export type Foo = number";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse_with_types();
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
fn test_repeat_until_complex_condition() {
    let input = "repeat x = x + 1 until x > 10 and y < 5 or z == 0";
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_numeric_for_with_step_lua51() {
    let input = "for i = 1, 10, 2 do print(i) end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_generic_for_pairs_lua51() {
    let input = "for k, v in pairs(t) do print(k, v) end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_generic_for_ipairs_lua51() {
    let input = "for i, v in ipairs(t) do print(i, v) end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_generic_for_next_lua51() {
    let input = "for k, v in next, t do print(k, v) end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_vararg_in_function_body() {
    let input = "function foo(...) return ... end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_vararg_in_table() {
    let input = "function foo(...) local t = {...} end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_vararg_in_call() {
    let input = "function foo(...) print(...) end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_multiple_assignment_lua51() {
    let input = "a, b, c = 1, 2, 3";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_swap_assignment_lua51() {
    let input = "a, b = b, a";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_table_with_function_value_lua51() {
    let input = "local t = { foo = function(x) return x end }";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_table_mixed_keys_lua51() {
    let input = "local t = { [1] = 'a', b = 'c', 'd' }";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_table_nested_lua51() {
    let input = "local t = { a = { b = { c = 1 } } }";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_long_string_level0_lua51() {
    let input = "local x = [[hello world]]";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_long_string_level1_lua51() {
    let input = "local x = [=[hello]=]";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_long_string_with_newlines_lua51() {
    let input = "local x = [[\nhello\nworld\n]]";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_function_as_argument_lua51() {
    let input = "pcall(function() error('test') end)";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_label_goto_lua52() {
    let input = "goto done\n::done::";
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_label_goto_lua53() {
    let input = "::start:: goto start";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_goto_rejected_lua51() {
    let input = "goto done";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err());
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
fn test_bitwise_not_lua53() {
    let input = "local x = ~a";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_left_shift_lua53() {
    let input = "local x = a << b";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_right_shift_lua53() {
    let input = "local x = a >> b";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_bitwise_rejected_lua51() {
    let input = "local x = a & b";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_bitwise_rejected_lua52() {
    let input = "local x = a & b";
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_floor_div_in_complex_expr_lua53() {
    let input = "local x = (a + b) // c * d";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_multiple_attributes_function_luau() {
    let input = "@native @inline function foo() end";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_type_cast_luau() {
    let input = "local x = y :: number";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_type_cast_in_expression_luau() {
    let input = "local x = (y :: number) + 1";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_oop_pattern_lua51() {
    let input = r#"
local MyClass = {}
MyClass.__index = MyClass
function MyClass.new(name)
    local self = setmetatable({}, MyClass)
    self.name = name
    return self
end
function MyClass:getName()
    return self.name
end
"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_module_pattern_lua51() {
    let input = r#"
local M = {}
function M.foo() return 1 end
function M.bar() return M.foo() + 1 end
return M
"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_complex_roblox_pattern_luau() {
    let input = r#"
local Players = game:GetService("Players")
local function onPlayerAdded(player: Player)
    local leaderstats = Instance.new("Folder")
    leaderstats.Name = "leaderstats"
    leaderstats.Parent = player
    local coins: IntValue = Instance.new("IntValue")
    coins.Name = "Coins"
    coins.Value = 0
    coins.Parent = leaderstats
end
Players.PlayerAdded:Connect(onPlayerAdded)
"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_empty_do_block_lua51() {
    let input = "do end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_empty_while_block_lua51() {
    let input = "while true do end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_empty_function_body_lua51() {
    let input = "function foo() end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_semicolons_between_statements_lua51() {
    let input = "local x = 1; local y = 2; local z = 3;";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_empty_statement_semicolons_lua52() {
    let input = ";;;";
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_unary_minus_number_lua51() {
    let input = "local x = -1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_unary_minus_variable_lua51() {
    let input = "local x = -y";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_unary_minus_call_lua51() {
    let input = "local x = -foo()";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_double_unary_minus_lua51() {
    let input = "local x = - -1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_precedence_add_mul_lua51() {
    let input = "local x = 1 + 2 * 3";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_precedence_and_or_lua51() {
    let input = "local x = a and b or c";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_precedence_not_and_lua51() {
    let input = "local x = not a and b";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_precedence_concat_lua51() {
    let input = "local x = a .. b .. c";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_precedence_pow_lua51() {
    let input = "local x = 2 ^ 3 ^ 2";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_nested_function_def_lua51() {
    let input = "function a.b.c() end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_method_function_def_lua51() {
    let input = "function a:b() end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_nested_method_function_def_lua51() {
    let input = "function a.b:c() end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_const_attribute_lua54() {
    let input = "local x <const> = 5";
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_close_attribute_lua54() {
    let input = "local f <close> = io.open('file')";
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_attribute_rejected_lua51() {
    let input = "local x <const> = 5";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_attribute_rejected_lua52() {
    let input = "local x <const> = 5";
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_attribute_rejected_lua53() {
    let input = "local x <const> = 5";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_return_nothing_lua51() {
    let input = "return";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_return_multiple_lua51() {
    let input = "return 1, 2, 3";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_return_function_call_lua51() {
    let input = "return foo()";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_less_than_lua51() {
    let input = "local x = a < b";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_greater_than_lua51() {
    let input = "local x = a > b";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_less_equal_lua51() {
    let input = "local x = a <= b";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_greater_equal_lua51() {
    let input = "local x = a >= b";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_equal_lua51() {
    let input = "local x = a == b";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_not_equal_lua51() {
    let input = "local x = a ~= b";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_deeply_nested_parens_lua51() {
    let input = "local x = ((((((1))))))";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_deeply_nested_tables_lua51() {
    let input = "local x = {{{{{1}}}}}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_multiline_string_in_call_lua51() {
    let input = "print [[\nhello\nworld\n]]";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_empty_input_lua51() {
    let input = "";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_whitespace_only_lua51() {
    let input = "   \n\n\t  ";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_comment_only_lua51() {
    let input = "-- just a comment";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_typed_variadic_param_luau() {
    let input = "local function foo(...: number) return ... end";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_frozen_table_call_luau() {
    let input = "local t = table.freeze({1, 2, 3})";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_hex_number_lua51() {
    let input = "local x = 0xFF";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_hex_float_lua53() {
    let input = "local x = 0x1.0p10";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_binary_number_luau() {
    let input = "local x = 0b1010";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_underscore_in_number_luau() {
    let input = "local x = 1_000_000";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_immediate_invoke_lua51() {
    let input = "(function() return 1 end)()";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_index_on_call_result_lua51() {
    // foo().bar is not a valid statement (only calls can be statements)
    // but it IS a valid expression, so we test it in an assignment context
    let input = "local x = foo().bar";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_call_on_string_lua51() {
    let input = "('hello'):upper()";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_trailing_comma_in_table_lua51() {
    let input = "local t = {1, 2, 3,}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_trailing_semicolon_in_table_lua51() {
    let input = "local t = {1; 2; 3;}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_call_as_statement_lua51() {
    let input = "print(1)";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_string_escapes_lua51() {
    let input = r#"local x = "hello\nworld\t\"\\""#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_string_hex_escape_lua52() {
    let input = r#"local x = "\x41\x42""#;
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_string_unicode_escape_lua53() {
    let input = r#"local x = "\u{1F600}""#;
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_long_elseif_chain_lua51() {
    let input = "if a then b() elseif c then d() elseif e then f() elseif g then h() else i() end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_local_function_many_params_lua51() {
    let input = "local function foo(a, b, c, d, e, ...) end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_assign_to_table_field_lua51() {
    let input = "t.x = 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_assign_to_table_index_lua51() {
    let input = "t[1] = 'hello'";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_assign_to_nested_field_lua51() {
    let input = "a.b.c.d = 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_multiple_local_with_types_luau() {
    let input = "local a: number, b: string = 1, 'hello'";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_type_cast_generic_luau() {
    let input = "local x = y :: Array<number>";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_type_cast_union_luau() {
    let input = "local x = y :: (string | number)";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_do_block_with_statements_lua51() {
    let input = "do local x = 1 local y = 2 end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_boolean_true_lua51() {
    let input = "local x = true";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_boolean_false_lua51() {
    let input = "local x = false";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_nil_literal_lua51() {
    let input = "local x = nil";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_length_operator_lua51() {
    let input = "local x = #t";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_length_of_string_lua51() {
    let input = "local x = #'hello'";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_break_in_while_lua51() {
    let input = "while true do break end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_break_in_for_lua51() {
    let input = "for i = 1, 10 do break end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_break_in_repeat_lua51() {
    let input = "repeat break until true";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}
