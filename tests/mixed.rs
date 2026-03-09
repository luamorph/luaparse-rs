use luaparse::{Parser, Luau, Lua51, Lua52, Lua53, Lua54};

#[test]
fn test_paren_call_lua51() {
    let input = r#"(f)(x)"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "paren call: {:?}", result);
}

#[test]
fn test_paren_field_assign_lua51() {
    let input = r#"(f).x = 1"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "paren field assign: {:?}", result);
}

#[test]
fn test_paren_index_assign_lua51() {
    let input = r#"(f)[1] = 1"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "paren index assign: {:?}", result);
}

#[test]
fn test_multi_target_mixed_assign_lua51() {
    let input = r#"a, b.c, d[1] = 1, 2, 3"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "multi target mixed: {:?}", result);
}

#[test]
fn test_deep_function_name_lua51() {
    let input = r#"function a.b.c.d() end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "deep function name: {:?}", result);
}

#[test]
fn test_nested_method_definition_lua51() {
    let input = r#"function a.b:c() end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "nested method definition: {:?}", result);
}

#[test]
fn test_recursive_local_function_lua51() {
    let input = r#"local function fib(n) if n < 2 then return n end return fib(n-1) + fib(n-2) end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "recursive local function: {:?}", result);
}

#[test]
fn test_leading_dot_number_lua51() {
    let input = r#"local x = .5"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "leading dot number: {:?}", result);
}

#[test]
fn test_scientific_notation_lua51() {
    let input = r#"local x = 1e10"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "scientific notation: {:?}", result);
}

#[test]
fn test_negative_exponent_lua51() {
    let input = r#"local x = 1e-5"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "negative exponent: {:?}", result);
}

#[test]
fn test_capital_e_exponent_lua51() {
    let input = r#"local x = 1E10"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "capital E exponent: {:?}", result);
}

#[test]
fn test_empty_while_body_lua51() {
    let input = r#"while true do end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "empty while body: {:?}", result);
}

#[test]
fn test_empty_for_body_lua51() {
    let input = r#"for i = 1, 10 do end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "empty for body: {:?}", result);
}

#[test]
fn test_empty_function_body_lua51() {
    let input = r#"function f() end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "empty function body: {:?}", result);
}

#[test]
fn test_empty_if_body_lua51() {
    let input = r#"if true then end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "empty if body: {:?}", result);
}

#[test]
fn test_empty_repeat_body_lua51() {
    let input = r#"repeat until true"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "empty repeat body: {:?}", result);
}

#[test]
fn test_ternary_pattern_lua51() {
    let input = r#"local x = cond and value1 or value2"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "ternary pattern: {:?}", result);
}

#[test]
fn test_unary_in_binary_lua51() {
    let input = r#"local x = -a + -b * -c"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "unary in binary: {:?}", result);
}

#[test]
fn test_not_comparison_lua51() {
    let input = r#"local x = not (a == b)"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "not comparison: {:?}", result);
}

#[test]
fn test_length_of_table_lua51() {
    let input = r#"local x = #{1, 2, 3}"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "length of table: {:?}", result);
}

#[test]
fn test_concat_call_lua51() {
    let input = r#"local x = tostring(1) .. tostring(2)"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "concat call: {:?}", result);
}

#[test]
fn test_typeof_in_union_luau() {
    let input = r#"local x: typeof(a) | string = nil"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "typeof in union: {:?}", result);
}

#[test]
fn test_deep_generic_type_luau() {
    let input = r#"local x: Array<Map<string, Array<number>>> = {}"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "deep generic type: {:?}", result);
}

#[test]
fn test_optional_function_type_luau() {
    let input = r#"local x: ((number) -> string)? = nil"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "optional function type: {:?}", result);
}

#[test]
fn test_type_assertion_chain_luau() {
    let input = r#"local x = (y :: any) :: number"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "type assertion chain: {:?}", result);
}

#[test]
fn test_generic_function_return_type_luau() {
    let input = r#"local function identity<T>(x: T): T return x end"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "generic function return type: {:?}", result);
}

#[test]
fn test_multi_generic_type_decl_luau() {
    let input = r#"type Either<A, B> = {tag: "left", value: A} | {tag: "right", value: B}"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "multi generic type decl: {:?}", result);
}

#[test]
fn test_frozen_table_type_luau() {
    let input = r#"local x: {name: string, age: number} = {name = "test", age = 1}"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "frozen table type: {:?}", result);
}

#[test]
fn test_floor_div_rejected_lua52() {
    let input = r#"local x = 10 // 3"#;
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "floor div should fail on 5.2: {:?}", result);
}

#[test]
fn test_floor_div_lua53() {
    let input = r#"local x = 10 // 3"#;
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "floor div 5.3: {:?}", result);
}

#[test]
fn test_floor_div_luau() {
    let input = r#"local x = 10 // 3"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "floor div luau: {:?}", result);
}

#[test]
fn test_bitwise_rejected_lua52() {
    let input = r#"local x = 1 & 2"#;
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "bitwise should fail on 5.2: {:?}", result);
}

#[test]
fn test_bitwise_lua54() {
    let input = r#"local x = 1 & 2 | 3 ~ 4"#;
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "bitwise 5.4: {:?}", result);
}

#[test]
fn test_type_rejected_lua52() {
    let input = r#"local x: number = 1"#;
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "types should fail on 5.2: {:?}", result);
}

#[test]
fn test_type_rejected_lua53() {
    let input = r#"local x: number = 1"#;
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "types should fail on 5.3: {:?}", result);
}

#[test]
fn test_type_rejected_lua54() {
    let input = r#"local x: number = 1"#;
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "types should fail on 5.4: {:?}", result);
}

#[test]
fn test_compound_rejected_lua52() {
    let input = r#"x += 1"#;
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "compound should fail on 5.2: {:?}", result);
}

#[test]
fn test_compound_rejected_lua53() {
    let input = r#"x += 1"#;
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "compound should fail on 5.3: {:?}", result);
}

#[test]
fn test_compound_rejected_lua54() {
    let input = r#"x += 1"#;
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "compound should fail on 5.4: {:?}", result);
}

#[test]
fn test_roblox_service_pattern_luau() {
    let input = r#"
local Players = game:GetService("Players")
local RunService = game:GetService("RunService")

local function onPlayerAdded(player: Player)
    print(player.Name)
end

Players.PlayerAdded:Connect(onPlayerAdded)
"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "roblox service pattern: {:?}", result);
}

#[test]
fn test_promise_pattern_luau() {
    let input = r#"
local function fetchData(): string
    return "data"
end

local result: string = fetchData()
"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "promise pattern: {:?}", result);
}

#[test]
fn test_complex_table_with_methods_luau() {
    let input = r#"
type Class = {
    new: (string, number) -> Class,
    getName: (self: Class) -> string,
    setName: (self: Class, name: string) -> (),
}
"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "complex table with methods: {:?}", result);
}

#[test]
fn test_multiline_expression_lua51() {
    let input = "local x = 1\n+\n2\n*\n3";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "multiline expression: {:?}", result);
}

#[test]
fn test_table_mixed_separators_lua51() {
    let input = r#"local t = {1, 2; 3, 4; 5}"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "table mixed separators: {:?}", result);
}

#[test]
fn test_bare_number_rejected_lua51() {
    let input = r#"42"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "bare number should fail: {:?}", result);
}

#[test]
fn test_bare_string_rejected_lua51() {
    let input = r#""hello""#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "bare string should fail: {:?}", result);
}

#[test]
fn test_assign_to_literal_rejected_lua51() {
    let input = r#"1 = 2"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "assign to literal should fail: {:?}", result);
}

#[test]
fn test_double_return_rejected_lua51() {
    let input = r#"return 1 return 2"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "double return should fail: {:?}", result);
}

#[test]
fn test_compound_sub_luau() {
    let input = r#"x -= 1"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "compound sub: {:?}", result);
}

#[test]
fn test_compound_mul_luau() {
    let input = r#"x *= 2"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "compound mul: {:?}", result);
}

#[test]
fn test_compound_div_luau() {
    let input = r#"x /= 2"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "compound div: {:?}", result);
}

#[test]
fn test_compound_mod_luau() {
    let input = r#"x %= 2"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "compound mod: {:?}", result);
}

#[test]
fn test_compound_pow_luau() {
    let input = r#"x ^= 2"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "compound pow: {:?}", result);
}

#[test]
fn test_compound_floor_div_luau() {
    let input = r#"x //= 2"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "compound floor div: {:?}", result);
}

#[test]
fn test_native_local_function_luau() {
    let input = r#"@native local function fast() end"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "native local function: {:?}", result);
}

#[test]
fn test_native_function_statement_luau() {
    let input = r#"@native function fast() end"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "native function statement: {:?}", result);
}

#[test]
fn test_lua54_const_attribute() {
    let input = r#"local x <const> = 42"#;
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "lua54 const: {:?}", result);
}

#[test]
fn test_block_comment_level1_lua51() {
    let input = "--[=[ block level 1 ]=]\nlocal x = 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "block comment level 1: {:?}", result);
}

#[test]
fn test_block_comment_level2_lua51() {
    let input = "--[==[ block level 2 ]==]\nlocal x = 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "block comment level 2: {:?}", result);
}

#[test]
fn test_if_expr_in_call_luau() {
    let input = r#"print(if x then "yes" else "no")"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "if expr in call: {:?}", result);
}

#[test]
fn test_if_expr_in_table_luau() {
    let input = r#"local t = {if x then 1 else 2, if y then 3 else 4}"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "if expr in table: {:?}", result);
}

#[test]
fn test_if_expr_in_binary_luau() {
    let input = r#"local x = (if a then 1 else 2) + 3"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "if expr in binary: {:?}", result);
}

#[test]
fn test_iterator_pattern_lua51() {
    let input = r#"
for line in io.lines("file.txt") do
    print(line)
end
"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "iterator pattern: {:?}", result);
}

#[test]
fn test_coroutine_pattern_lua51() {
    let input = r#"
local co = coroutine.create(function()
    local x = 1
    coroutine.yield(x)
    x = x + 1
    return x
end)
"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "coroutine pattern: {:?}", result);
}

#[test]
fn test_metatable_pattern_lua51() {
    let input = r#"
local mt = {}
mt.__index = mt
mt.__add = function(a, b)
    return setmetatable({value = a.value + b.value}, mt)
end
"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "metatable pattern: {:?}", result);
}

#[test]
fn test_pcall_pattern_lua51() {
    let input = r#"
local ok, err = pcall(function()
    error("something went wrong")
end)
if not ok then
    print(err)
end
"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "pcall pattern: {:?}", result);
}

#[test]
fn test_string_method_chain_lua51() {
    let input = r#"local x = s:gsub("a", "b"):lower():sub(1, 5)"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "string method chain: {:?}", result);
}

#[test]
fn test_table_operations_lua51() {
    let input = r#"
local t = {}
table.insert(t, 1)
table.insert(t, 1, "first")
table.remove(t, 1)
table.sort(t, function(a, b) return a < b end)
"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "table operations: {:?}", result);
}

#[test]
fn test_nested_closures_lua51() {
    let input = r#"
local function make_counter()
    local count = 0
    return function()
        count = count + 1
        return count
    end
end
"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "nested closures: {:?}", result);
}

#[test]
fn test_multiline_string_in_table_lua51() {
    let input = r#"local t = {msg = [[
hello
world
]]}"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "multiline string in table: {:?}", result);
}

#[test]
fn test_for_in_next_lua51() {
    let input = r#"for k, v in next, t, nil do print(k, v) end"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "for-in next: {:?}", result);
}

#[test]
fn test_select_pattern_lua51() {
    let input = "local function count(...) return select(\"#\", ...) end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "select pattern: {:?}", result);
}
