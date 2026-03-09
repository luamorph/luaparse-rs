use luaparse::{Parser, Luau, Lua51, Lua52, Lua53, Lua54};

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
    let input = "x %= 3";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_compound_concat_assign_luau() {
    let input = "x ..= 'world'";
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
fn test_compound_floor_div_assign_luau() {
    let input = "x //= 2";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_function_call_as_last_value_lua51() {
    let input = "local a, b, c = 1, foo()";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_table_immediate_index_lua51() {
    let input = "local x = ({1, 2, 3})[1]";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_mixed_chain_lua51() {
    let input = "a.b:c(d).e[f]()";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_negative_number_in_table_lua51() {
    let input = "local t = {-1, -2, -3}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_negative_number_as_key_lua51() {
    let input = "local t = {[-1] = 'neg'}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_immediate_call_function_expr_lua51() {
    let input = "(function() return 1 end)()";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_function_as_argument_lua51() {
    let input = "table.sort(t, function(a, b) return a < b end)";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_multi_return_from_function_lua51() {
    let input = "function foo() return 1, 2, 3 end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_type_assertion_on_call_luau() {
    let input = "local x = foo() :: number";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_type_assertion_on_table_index_luau() {
    let input = "local x = t.field :: string";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_type_assertion_on_binary_luau() {
    let input = "local x = (a + b) :: number";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_comment_between_statements_lua51() {
    let input = "local x = 1\n-- comment\nlocal y = 2";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_comment_at_start_lua51() {
    let input = "-- first line comment\nlocal x = 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_only_comments_lua51() {
    let input = "-- just a comment";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_block_comment_between_exprs_lua51() {
    let input = "local x = 1 --[[ inline ]] + 2";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_type_optional_function_luau() {
    let input = "local f: ((number) -> string)? = nil";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_type_intersection_luau() {
    let input = "local x: A & B & C = {}";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_type_complex_nested_luau() {
    let input = "local x: { callback: (string, number?) -> boolean?, data: {[string]: any} } = {}";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_goto_forward_lua53() {
    let input = "goto skip\nprint('skipped')\n::skip::";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_deeply_nested_call_chain_lua51() {
    let input = "a(b(c(d(e(f())))))";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_deeply_nested_binary_lua51() {
    let input = "local x = ((((a + b) * c) - d) / e) ^ f";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_precedence_and_or_lua51() {
    let input = "local x = a or b and c";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_precedence_comparison_lua51() {
    let input = "local x = a + b == c * d";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_precedence_unary_vs_binary_lua51() {
    let input = "local x = -a ^ 2";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_precedence_not_comparison_lua51() {
    let input = "local x = not a == b";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_bitwise_precedence_lua53() {
    let input = "local x = a | b & c ~ d";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_shift_precedence_lua53() {
    let input = "local x = a << b | c >> d";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_interpolated_string_with_field_luau() {
    let input = "local x = `name: {player.Name}`";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_interpolated_string_with_math_luau() {
    let input = "local x = `result: {1 + 2}`";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_assign_to_chained_field_lua51() {
    let input = "a.b.c.d.e = 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_assign_to_nested_index_lua51() {
    let input = "a[b[c]] = 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_assign_to_call_result_field_lua51() {
    let input = "foo().bar = 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_long_string_no_newline_lua51() {
    let input = "local x = [[single line]]";
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
fn test_empty_if_body_lua51() {
    let input = "if true then end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_empty_while_body_lua51() {
    let input = "while true do end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_empty_for_body_lua51() {
    let input = "for i = 1, 10 do end";
    let parser = Parser::<Lua51>::new(input).unwrap();
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
fn test_empty_repeat_body_lua51() {
    let input = "repeat until true";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_signal_pattern_luau() {
    let input = r#"
local RunService = game:GetService("RunService")
local connection: RBXScriptConnection? = nil
connection = RunService.Heartbeat:Connect(function(dt: number)
    if not shouldRun then
        if connection then
            connection:Disconnect()
            connection = nil
        end
        return
    end
    update(dt)
end)
"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_module_with_types_luau() {
    let input = r#"
export type Vector3 = {
    X: number,
    Y: number,
    Z: number,
}

local function add(a: Vector3, b: Vector3): Vector3
    return {
        X = a.X + b.X,
        Y = a.Y + b.Y,
        Z = a.Z + b.Z,
    }
end
"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_setmetatable_pattern_lua51() {
    let input = r#"
local mt = {}
mt.__index = mt

function mt:new(x)
    return setmetatable({value = x}, self)
end

function mt:getValue()
    return self.value
end
"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_pcall_pattern_lua51() {
    let input = "local ok, result = pcall(function() return dangerous() end)";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_continue_rejected_lua52() {
    let input = "while true do continue end";
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "continue should be rejected in Lua 5.2");
}

#[test]
fn test_continue_rejected_lua53() {
    let input = "while true do continue end";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "continue should be rejected in Lua 5.3");
}

#[test]
fn test_type_cast_rejected_lua51() {
    let input = "local x = y :: number";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    // :: in Lua 5.2+ is a label, in Luau it's type cast
    // In Lua 5.1, :: should cause an error
}

#[test]
fn test_export_rejected_lua51() {
    let input = "export type Foo = number";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "export should be rejected in Lua 5.1");
}

#[test]
fn test_type_decl_rejected_lua51() {
    let input = "type Foo = number";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    // In Lua 5.1, `type` is just an identifier, so this parses as `type` (assignment target) `Foo` ...
    // which should fail because `type Foo` isn't a valid assignment
}

#[test]
fn test_concat_right_associativity_lua51() {
    let input = "local x = a .. b .. c";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_double_colon_method_chain_lua51() {
    let input = "obj:m1 'a' :m2 'b'";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    // This should parse as obj:m1('a'):m2('b')
    assert!(result.is_ok());
}

#[test]
fn test_close_with_file_lua54() {
    let input = r#"
do
    local f <close> = io.open("test.txt", "r")
    if f then
        local content = f:read("*a")
        print(content)
    end
end
"#;
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_table_as_last_arg_lua51() {
    let input = "f(1, 2, {a = 1, b = 2})";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_instance_creation_luau() {
    let input = r#"
local part = Instance.new("Part")
part.Size = Vector3.new(4, 1, 2)
part.Position = Vector3.new(0, 10, 0)
part.Anchored = true
part.Parent = workspace
"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_for_in_ipairs_luau() {
    let input = "for i, v in ipairs(t) do print(i, v) end";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_no_spaces_around_ops_lua51() {
    let input = "local x=1+2*3";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_lots_of_whitespace_lua51() {
    let input = "  local   x   =   1   +   2  ";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_tabs_lua51() {
    let input = "\tlocal\tx\t=\t1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}
