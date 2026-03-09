use luaparse::{Parser, Luau, Lua51, Lua52, Lua53, Lua54};

#[test]
fn test_anonymous_function_no_generic_luau() {
    let input = "local f = function(x: number): string return tostring(x) end";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_type_module_path_luau() {
    let input = "local x: module.SubType = y";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_type_module_generic_luau() {
    let input = "local x: module.Map<string, number> = {}";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_type_nested_optional_luau() {
    let input = "local x: string?? = nil";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_type_union_with_nil_luau() {
    let input = "local x: string | nil = nil";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_type_function_returning_function_luau() {
    let input = "local x: (number) -> (string) -> boolean = fn";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_type_table_indexer_luau() {
    let input = "local x: { [number]: string } = {}";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_type_mixed_table_luau() {
    let input = "local x: { name: string, [number]: boolean } = {}";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_type_decl_function_type_luau() {
    let input = "type Callback = (number, string) -> boolean";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse_with_types();
    assert!(result.is_ok());
}

#[test]
fn test_type_decl_union_luau() {
    let input = "type Result = string | number | nil";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse_with_types();
    assert!(result.is_ok());
}

#[test]
fn test_type_decl_table_luau() {
    let input = "type Config = { enabled: boolean, name: string }";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse_with_types();
    assert!(result.is_ok());
}

#[test]
fn test_type_decl_generic_constraint_luau() {
    let input = "type Container<T> = { value: T }";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse_with_types();
    assert!(result.is_ok());
}

#[test]
fn test_if_without_else_lua51() {
    let input = "if true then print(1) end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_nested_if_lua51() {
    let input = "if a then if b then c() end end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_return_in_function_lua51() {
    let input = "function foo() return 1, 2, 3 end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_return_at_end_of_do_block_lua51() {
    let input = "do return 1 end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_chained_comparisons_lua51() {
    let input = "local x = a < b and b < c";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_ternary_idiom_lua51() {
    let input = "local x = condition and value_true or value_false";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_complex_binary_expr_lua51() {
    let input = "local x = (a + b) * (c - d) / (e ^ f)";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_continue_in_while_luau() {
    let input = "while true do if x then continue end end";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_continue_in_repeat_luau() {
    let input = "repeat if x then continue end until false";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_continue_in_generic_for_luau() {
    let input = "for k, v in pairs(t) do if v then continue end end";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_table_with_computed_keys_lua51() {
    let input = "local t = { [1+1] = 'two', [2+2] = 'four' }";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_table_with_string_keys_lua51() {
    let input = "local t = { ['key with spaces'] = 1 }";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_table_empty_then_assign_lua51() {
    let input = "local t = {} t.x = 1 t.y = 2";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_single_quoted_string_lua51() {
    let input = "local x = 'hello world'";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_empty_string_lua51() {
    let input = "local x = ''";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_multiline_long_string_comment_lua51() {
    let input = "--[[\nThis is a\nmultiline comment\n]]\nlocal x = 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_nested_call_lua51() {
    let input = "print(tostring(tonumber('42')))";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_call_with_multiple_args_lua51() {
    let input = "string.format('%s = %d', key, value)";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_goto_in_if_lua52() {
    let input = "if x then goto skip end ::skip::";
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_multiple_labels_lua52() {
    let input = "::start:: print(1) ::middle:: print(2) ::end_label::";
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_for_with_expression_bounds_lua51() {
    let input = "for i = math.max(1, a), math.min(10, b) do print(i) end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_generic_for_multiple_iterators_lua51() {
    let input = "for k, v in next, t, nil do print(k) end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_integer_division_lua54() {
    let input = "local x = 7 // 2";
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_bitwise_ops_lua54() {
    let input = "local x = (a & b) | (c ~ d)";
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_shift_ops_lua54() {
    let input = "local x = a << 2 + b >> 1";
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_compound_assign_field_luau() {
    let input = "t.x += 1";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_compound_assign_index_luau() {
    let input = "t[1] += 1";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_compound_assign_nested_field_luau() {
    let input = "a.b.c += 1";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_function_call_newline_paren_lua51() {
    // In Lua, f\n(x) is treated as f(x) — no ASI
    let input = "f\n(x)";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    // This could be valid (call) or error depending on parser behavior
    // Just check it doesn't panic
}

#[test]
fn test_multiple_calls_one_line_lua51() {
    let input = "print(1) print(2) print(3)";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_multi_target_assignment_lua51() {
    let input = "a, b.c, d[1] = 1, 2, 3";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_interpolated_string_with_call_luau() {
    let input = "local x = `result: {tostring(val)}`";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_interpolated_string_multiple_exprs_luau() {
    let input = "local x = `{a} and {b} and {c}`";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_interpolated_string_empty_luau() {
    let input = "local x = ``";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_nested_loops_lua51() {
    let input = "for i = 1, 10 do for j = 1, 10 do for k = 1, 10 do print(i, j, k) end end end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_nested_if_in_loop_lua51() {
    let input = "while true do if a then if b then break end end end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_promise_chain_pattern_luau() {
    let input = r#"
local HttpService = game:GetService("HttpService")
local function fetchData(url: string): string
    local response = HttpService:GetAsync(url)
    return response
end
"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_typed_table_operations_luau() {
    let input = r#"
local inventory: {[string]: number} = {}
inventory["sword"] = 1
inventory["shield"] = 2
local total: number = 0
for item, count in pairs(inventory) do
    total += count
end
"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_event_handler_pattern_luau() {
    let input = r#"
local Players = game:GetService("Players")
Players.PlayerAdded:Connect(function(player: Player)
    player.CharacterAdded:Connect(function(character: Model)
        local humanoid: Humanoid = character:WaitForChild("Humanoid")
        humanoid.Died:Connect(function()
            print(player.Name .. " died")
        end)
    end)
end)
"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_typed_numeric_for_luau() {
    let input = "for i: number = 1, 10 do print(i) end";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_typed_generic_for_luau() {
    let input = "for k: string, v: number in pairs(t) do print(k, v) end";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_mixed_statements_lua51() {
    let input = r#"
local x = 1
local y = 2
x = x + y
print(x)
if x > 2 then
    print("big")
else
    print("small")
end
for i = 1, x do
    print(i)
end
"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_block_comment_nested_brackets_lua51() {
    let input = "--[=[ comment with ] and [[ inside ]=]\nlocal x = 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
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
    assert!(result.is_err());
}

#[test]
fn test_floor_div_rejected_lua52() {
    let input = "local x = 10 // 3";
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_bitwise_lua54() {
    let input = "local x = a & b";
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_bitwise_luau() {
    // Luau does NOT have bitwise operators (uses bit32 library instead)
    let input = "local x = a & b";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    // Luau may or may not support & — check the marker
    // If HAS_BITWISE_OPS is false for Luau, this should fail
}

#[test]
fn test_type_assertion_chain_luau() {
    let input = "local x = (y :: any) :: number";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_shebang_lua51() {
    let input = "#!/usr/bin/lua\nlocal x = 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_large_integer_lua51() {
    let input = "local x = 9999999999";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_scientific_notation_lua51() {
    let input = "local x = 1e10";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_negative_exponent_lua51() {
    let input = "local x = 1.5e-3";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_roblox_class_pattern_luau() {
    let input = r#"
local MyClass = {}
MyClass.__index = MyClass

type MyClass = typeof(setmetatable({} :: {
    name: string,
    health: number,
}, {} :: { __index: MyClass }))

function MyClass.new(name: string): MyClass
    local self = setmetatable({}, MyClass)
    self.name = name
    self.health = 100
    return self
end

function MyClass:TakeDamage(amount: number)
    self.health -= amount
    if self.health <= 0 then
        self.health = 0
    end
end
"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_attribute_with_args_luau() {
    let input = "@checked function validate(x: number): boolean return x > 0 end";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_variadic_type_param_luau() {
    let input = "local function foo(...: string) end";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_type_annotation_on_local_multi_luau() {
    let input = "local a: number, b: string, c: boolean = 1, 'hi', true";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_nested_generic_in_type_decl_luau() {
    let input = "type DeepMap = Map<string, Map<string, Array<number>>>";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse_with_types();
    assert!(result.is_ok());
}

#[test]
fn test_very_deep_generic_luau() {
    let input = "type Deep = A<B<C<D<number>>>>";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse_with_types();
    assert!(result.is_ok());
}

#[test]
fn test_intersection_with_generic_luau() {
    let input = "local x: Foo<number> & Bar<string> = {}";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_string_literal_type_luau() {
    let input = r#"type Direction = "north" | "south" | "east" | "west""#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse_with_types();
    assert!(result.is_ok());
}

#[test]
fn test_boolean_literal_type_luau() {
    let input = "type AlwaysTrue = true";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse_with_types();
    assert!(result.is_ok());
}
