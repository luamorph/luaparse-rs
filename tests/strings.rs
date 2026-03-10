use luaparse_rs::{Parser, Luau, Lua51, Lua52, Lua53, Lua54};

#[test]
fn test_long_string_nested_brackets_lua51() {
    let input = "local x = [==[hello ]=] still going]==]";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "long string nested brackets: {:?}", result);
}

#[test]
fn test_long_comment_nested_brackets_lua51() {
    let input = "--[==[comment ]=] still comment]==]\nlocal x = 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "long comment nested brackets: {:?}", result);
}

#[test]
fn test_single_quote_string_lua51() {
    let input = "local x = 'hello world'";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "single quote string: {:?}", result);
}

#[test]
fn test_interpolation_with_method_call_luau() {
    let input = "local x = `{obj:method()}`";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "interpolation with method call: {:?}", result);
}

#[test]
fn test_interpolation_with_table_luau() {
    let input = "local x = `{t.field}`";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "interpolation with table: {:?}", result);
}

#[test]
fn test_interpolation_rejected_lua51() {
    let input = "local x = `{name}`";
    let parser = Parser::<Lua51>::new(input);
    // Should fail either at lex or parse time
    let result = match parser {
        Ok(p) => p.parse().is_err(),
        Err(_) => true,
    };
    assert!(result, "interpolation should fail on 5.1");
}

#[test]
fn test_function_long_dotted_name_lua51() {
    let input = "function a.b.c.d.e.f() end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "function long dotted name: {:?}", result);
}

#[test]
fn test_multiple_method_defs_lua51() {
    let input = "function obj:foo() end\nfunction obj:bar() end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "multiple method defs: {:?}", result);
}

#[test]
fn test_table_with_closures_lua51() {
    let input = "local t = {f = function(x) return x end, g = function(y) return y end}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "table with closures: {:?}", result);
}

#[test]
fn test_nested_computed_keys_lua51() {
    let input = "local t = {[1] = {[2] = {[3] = \"deep\"}}}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "nested computed keys: {:?}", result);
}

#[test]
fn test_table_only_positional_lua51() {
    let input = "local t = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "table only positional: {:?}", result);
}

#[test]
fn test_empty_table_in_call_lua51() {
    let input = "f({})";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "empty table in call: {:?}", result);
}

#[test]
fn test_deep_parens_lua51() {
    let input = "local x = ((((((1))))))";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "deep parens: {:?}", result);
}

#[test]
fn test_unary_minus_paren_lua51() {
    let input = "local x = -(a + b)";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "unary minus paren: {:?}", result);
}

#[test]
fn test_not_on_call_lua51() {
    let input = "local x = not f()";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "not on call: {:?}", result);
}

#[test]
fn test_length_of_field_lua51() {
    let input = "local x = #t.items";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "length of field: {:?}", result);
}

#[test]
fn test_export_generic_type_luau() {
    let input = "export type Result<T, E> = {ok: true, value: T} | {ok: false, error: E}";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "export generic type: {:?}", result);
}

#[test]
fn test_type_string_literal_luau() {
    let input = "type Direction = \"north\" | \"south\" | \"east\" | \"west\"";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "type string literal: {:?}", result);
}

#[test]
fn test_type_boolean_literal_luau() {
    let input = "type Success = {ok: true, value: string}";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "type boolean literal: {:?}", result);
}

#[test]
fn test_nested_function_types_luau() {
    let input = "type Transformer = (input: (string) -> number) -> (number) -> string";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "nested function types: {:?}", result);
}

#[test]
fn test_if_expr_assigned_to_field_luau() {
    let input = "t.x = if cond then 1 else 2";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "if expr assigned to field: {:?}", result);
}

#[test]
fn test_if_expr_in_return_luau() {
    let input = "local function f() return if x then 1 else 2 end";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "if expr in return: {:?}", result);
}

#[test]
fn test_if_expr_multiple_elseif_luau() {
    let input = "local x = if a then 1 elseif b then 2 elseif c then 3 else 4";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "if expr multiple elseif: {:?}", result);
}

#[test]
fn test_multi_assign_with_calls_lua51() {
    let input = "a.b, c().d, e[1] = 1, 2, 3";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "multi assign with calls: {:?}", result);
}

#[test]
fn test_assign_complex_rhs_lua51() {
    let input = "a = f(1) + g(2) * h(3)";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "assign complex rhs: {:?}", result);
}

#[test]
fn test_type_cast_in_assignment_luau() {
    let input = "local x = t :: {[string]: number}";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "type cast in assignment: {:?}", result);
}

#[test]
fn test_type_cast_on_call_result_luau() {
    let input = "local x = f() :: number";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "type cast on call result: {:?}", result);
}

#[test]
fn test_type_cast_rejected_lua51() {
    let input = "local x = y :: number";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "type cast should fail on 5.1: {:?}", result);
}

#[test]
fn test_continue_in_for_luau() {
    let input = "for i = 1, 10 do if i == 5 then continue end print(i) end";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "continue in for luau: {:?}", result);
}

#[test]
fn test_continue_rejected_lua52() {
    let input = "for i = 1, 10 do continue end";
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "continue should fail on 5.2: {:?}", result);
}

#[test]
fn test_continue_rejected_lua53() {
    let input = "for i = 1, 10 do continue end";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "continue should fail on 5.3: {:?}", result);
}

#[test]
fn test_goto_works_lua53() {
    let input = "goto done\n::done::";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "goto works 5.3: {:?}", result);
}

#[test]
fn test_goto_works_lua54() {
    let input = "goto done\n::done::";
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "goto works 5.4: {:?}", result);
}

#[test]
fn test_full_module_lua51() {
    let input = r#"
local M = {}
M.__index = M

function M.new(name)
    local self = setmetatable({}, M)
    self.name = name
    self.items = {}
    return self
end

function M:add(item)
    table.insert(self.items, item)
end

function M:get(index)
    return self.items[index]
end

function M:size()
    return #self.items
end

return M
"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "full module: {:?}", result);
}

#[test]
fn test_complex_luau_typed_code() {
    let input = r#"
type Option<T> = T | nil

local function map<T, U>(opt: Option<T>, f: (T) -> U): Option<U>
    return if opt ~= nil then f(opt) else nil
end

local function unwrap<T>(opt: Option<T>, default: T): T
    return if opt ~= nil then opt else default
end

local x: Option<number> = 42
local y: Option<string> = map(x, function(n: number): string
    return tostring(n)
end)
"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "complex luau typed: {:?}", result);
}

#[test]
fn test_mixed_floor_div_bitwise_lua53() {
    let input = "local x = (a // b) & 0xFF | (c // d) << 8";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "mixed floor div bitwise: {:?}", result);
}

#[test]
fn test_lua54_complex_code() {
    let input = r#"
local x <const> = 42
local y <const> = "hello"
local f <close> = io.open("test.txt")
if f then
    local content = f:read("*a")
    print(content)
end
"#;
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "lua54 complex: {:?}", result);
}

#[test]
fn test_unterminated_string_fails_lua51() {
    let input = "local x = \"hello";
    let parser = Parser::<Lua51>::new(input);
    let result = match parser {
        Ok(p) => p.parse().is_err(),
        Err(_) => true,
    };
    assert!(result, "unterminated string should fail");
}

#[test]
fn test_unterminated_table_fails_lua51() {
    let input = "local x = {1, 2, 3";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "unterminated table should fail: {:?}", result);
}

#[test]
fn test_missing_end_fails_lua51() {
    let input = "if true then print(1)";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "missing end should fail: {:?}", result);
}

#[test]
fn test_extra_end_fails_lua51() {
    let input = "if true then end end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "extra end should fail: {:?}", result);
}

#[test]
fn test_assign_to_method_call_fails_lua51() {
    let input = "f:method() = 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "assign to method call should fail: {:?}", result);
}

#[test]
fn test_do_end_only_lua51() {
    let input = "do end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "do end only: {:?}", result);
}

#[test]
fn test_deeply_nested_do_lua51() {
    let input = "do do do do end end end end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "deeply nested do: {:?}", result);
}

#[test]
fn test_call_no_args_lua51() {
    let input = "f()";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "call no args: {:?}", result);
}

#[test]
fn test_negative_number_lua51() {
    let input = "local x = -1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "negative number: {:?}", result);
}

#[test]
fn test_boolean_values_lua51() {
    let input = "local a, b = true, false";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "boolean values: {:?}", result);
}

#[test]
fn test_nil_value_lua51() {
    let input = "local x = nil";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "nil value: {:?}", result);
}

#[test]
fn test_multi_return_function_lua51() {
    let input = "local function f() return 1, 2, 3 end\nlocal a, b, c = f()";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "multi return function: {:?}", result);
}

#[test]
fn test_while_break_lua51() {
    let input = "while true do if x then break end end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "while break: {:?}", result);
}

#[test]
fn test_repeat_complex_condition_lua51() {
    let input = "repeat x = x + 1 until x >= 10 and y < 20";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "repeat complex condition: {:?}", result);
}

#[test]
fn test_for_with_call_iterator_lua51() {
    let input = "for k, v in pairs(t) do print(k, v) end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "for with call iterator: {:?}", result);
}
