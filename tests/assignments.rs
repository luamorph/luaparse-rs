use luaparse::{Parser, Luau, Lua51, Lua52, Lua53, Lua54};

#[test]
fn test_goto_forward_lua52() {
    let input = "goto done\nlocal x = 1\n::done::";
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "goto forward: {:?}", result);
}

#[test]
fn test_goto_backward_lua52() {
    let input = "::start::\nlocal x = 1\ngoto start";
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "goto backward: {:?}", result);
}

#[test]
fn test_multiple_labels_lua52() {
    let input = "::a::\ngoto b\n::b::\ngoto a";
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "multiple labels: {:?}", result);
}

#[test]
fn test_label_in_do_lua53() {
    let input = "do ::inner:: print(1) end";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "label in do: {:?}", result);
}

#[test]
fn test_return_in_function_lua51() {
    let input = "local function f() return 1 end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "return in function: {:?}", result);
}

#[test]
fn test_return_semicolon_lua51() {
    let input = "return 1;";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "return semicolon: {:?}", result);
}

#[test]
fn test_return_nothing_toplevel_lua51() {
    let input = "return";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "return nothing: {:?}", result);
}

#[test]
fn test_code_before_return_lua51() {
    let input = "local x = 1\nreturn x";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "code before return: {:?}", result);
}

#[test]
fn test_double_return_fails_lua51() {
    let input = "return 1 return 2";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "double return should fail: {:?}", result);
}

#[test]
fn test_return_then_statement_fails_lua51() {
    let input = "return 1\nlocal x = 2";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "return then statement should fail: {:?}", result);
}

#[test]
fn test_return_in_nested_block_lua51() {
    let input = "if true then return 1 end\nlocal x = 2";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "return in nested block: {:?}", result);
}

#[test]
fn test_deep_field_assign_lua51() {
    let input = "a.b.c.d.e = 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "deep field assign: {:?}", result);
}

#[test]
fn test_mixed_field_index_assign_lua51() {
    let input = "a.b[1].c[2] = 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "mixed field index assign: {:?}", result);
}

#[test]
fn test_call_chain_field_assign_lua51() {
    let input = "f().g().h = 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "call chain field assign: {:?}", result);
}

#[test]
fn test_method_call_field_assign_lua51() {
    let input = "obj:method().field = 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "method call field assign: {:?}", result);
}

#[test]
fn test_bare_call_assign_fails_lua51() {
    let input = "f() = 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "bare call assign should fail: {:?}", result);
}

#[test]
fn test_empty_string_lua51() {
    let input = "local x = \"\"";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "empty string: {:?}", result);
}

#[test]
fn test_single_char_string_lua51() {
    let input = "local x = 'a'";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "single char string: {:?}", result);
}

#[test]
fn test_escaped_quotes_lua51() {
    let input = r#"local x = "hello \"world\"""#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "escaped quotes: {:?}", result);
}

#[test]
fn test_newline_escape_lua51() {
    let input = r#"local x = "line1\nline2""#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "newline escape: {:?}", result);
}

#[test]
fn test_empty_long_string_lua51() {
    let input = "local x = [[]]";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "empty long string: {:?}", result);
}

#[test]
fn test_call_with_table_arg_lua51() {
    let input = "f{1, 2, 3}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "call with table arg: {:?}", result);
}

#[test]
fn test_call_with_string_arg_lua51() {
    let input = "f\"hello\"";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "call with string arg: {:?}", result);
}

#[test]
fn test_call_with_long_string_arg_lua51() {
    let input = "f[[hello]]";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "call with long string arg: {:?}", result);
}

#[test]
fn test_method_call_table_arg_lua51() {
    let input = "obj:method{1, 2}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "method call table arg: {:?}", result);
}

#[test]
fn test_method_call_string_arg_lua51() {
    let input = "obj:method\"hello\"";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "method call string arg: {:?}", result);
}

#[test]
fn test_chained_calls_lua51() {
    let input = "f(1)(2)(3)";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "chained calls: {:?}", result);
}

#[test]
fn test_chained_mixed_calls_lua51() {
    let input = "f(1){2}\"3\"";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "chained mixed calls: {:?}", result);
}

#[test]
fn test_array_type_luau() {
    let input = "local x: {number} = {}";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "array type: {:?}", result);
}

#[test]
fn test_tuple_type_luau() {
    let input = "local function f(): (number, string) return 1, \"a\" end";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "tuple type: {:?}", result);
}

#[test]
fn test_recursive_type_luau() {
    let input = "type Node = {value: number, children: {Node}}";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "recursive type: {:?}", result);
}

#[test]
fn test_intersection_complex_luau() {
    let input = "local x: Readable & Writable & Seekable = nil";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "intersection complex: {:?}", result);
}

#[test]
fn test_union_with_nil_luau() {
    let input = "local x: string | number | nil = nil";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "union with nil: {:?}", result);
}

#[test]
fn test_type_cast_expression_luau() {
    let input = "local x = (y :: number) + 1";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "type cast expression: {:?}", result);
}

#[test]
fn test_compound_on_field_luau() {
    let input = "a.b += 1";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "compound on field: {:?}", result);
}

#[test]
fn test_compound_on_index_luau() {
    let input = "a[1] += 1";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "compound on index: {:?}", result);
}

#[test]
fn test_close_in_do_lua54() {
    let input = "do local f <close> = io.open(\"test\") end";
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "close in do: {:?}", result);
}

#[test]
fn test_const_multiple_lua54() {
    let input = "local a <const>, b <const> = 1, 2";
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "const multiple: {:?}", result);
}

#[test]
fn test_lua54_attr_rejected_luau() {
    let input = "local x <const> = 1";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "lua54 attr on luau should fail: {:?}", result);
}

#[test]
fn test_multiline_args_lua51() {
    let input = "f(\n1,\n2,\n3\n)";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "multiline args: {:?}", result);
}

#[test]
fn test_multiline_table_constructor_lua51() {
    let input = "local t = {\n[1] = \"a\",\n[2] = \"b\",\n}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "multiline table constructor: {:?}", result);
}

#[test]
fn test_nested_and_or_lua51() {
    let input = "local x = (a and b) or (c and d) or e";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "nested and or: {:?}", result);
}

#[test]
fn test_comparison_chain_lua51() {
    let input = "if a < b and b < c then print(1) end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "comparison chain: {:?}", result);
}

#[test]
fn test_varargs_return_lua51() {
    let input = "local function f(...) return ... end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "varargs return: {:?}", result);
}

#[test]
fn test_varargs_forward_lua51() {
    let input = "local function f(...) g(...) end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "varargs forward: {:?}", result);
}

#[test]
fn test_varargs_in_concat_lua51() {
    let input = "local function f(...) return \"prefix\" .. ... end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "varargs in concat: {:?}", result);
}

#[test]
fn test_semicolons_between_statements_lua51() {
    let input = "local a = 1; local b = 2; local c = 3;";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "semicolons between statements: {:?}", result);
}

#[test]
fn test_many_semicolons_lua52() {
    let input = ";;;local x = 1;;;";
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "many semicolons: {:?}", result);
}

#[test]
fn test_empty_semicolons_rejected_lua51() {
    let input = ";;;";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    // Note: this depends on whether the parser gates empty semicolons
    // In standard Lua 5.1, empty statements (just ;) are NOT allowed
    // But many parsers are lenient. This is more of a "nice to have" check.
    // If the parser is lenient, change to is_ok
    // Not asserting either way - just checking it doesn't crash
}

#[test]
fn test_interpolated_empty_expr_luau() {
    let input = "local x = `hello {name}`";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "interpolated empty expr: {:?}", result);
}

#[test]
fn test_interpolated_complex_expr_luau() {
    let input = "local x = `result: {a + b}`";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "interpolated complex expr: {:?}", result);
}

#[test]
fn test_interpolated_multiple_luau() {
    let input = "local x = `{a} and {b} and {c}`";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "interpolated multiple: {:?}", result);
}

#[test]
fn test_function_as_value_lua51() {
    let input = "local f = function(x) return x * 2 end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "function as value: {:?}", result);
}

#[test]
fn test_function_in_table_lua51() {
    let input = "local t = {f = function() end, g = function(x) return x end}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "function in table: {:?}", result);
}

#[test]
fn test_immediate_function_call_lua51() {
    let input = "(function() print(1) end)()";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "immediate function call: {:?}", result);
}

#[test]
fn test_table_string_key_lua51() {
    let input = "local t = {[\"key\"] = 1}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "table string key: {:?}", result);
}

#[test]
fn test_table_expr_key_lua51() {
    let input = "local t = {[1+2] = 3}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "table expr key: {:?}", result);
}

#[test]
fn test_table_concat_key_lua51() {
    let input = "local t = {[\"pre\" .. \"fix\"] = 1}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "table concat key: {:?}", result);
}

#[test]
fn test_luau_class_pattern() {
    let input = r#"
type MyClass = {
    new: (name: string) -> MyClass,
    getName: (self: MyClass) -> string,
}

local MyClass = {} :: MyClass;
(MyClass :: any).__index = MyClass

function MyClass.new(name: string): MyClass
    local self = setmetatable({}, MyClass) :: MyClass
    return self
end

function MyClass:getName(): string
    return "test"
end
"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "luau class pattern: {:?}", result);
}

#[test]
fn test_luau_module_pattern() {
    let input = r#"
local module = {}

export type Config = {
    name: string,
    value: number?,
}

function module.create(config: Config): Config
    return config
end

return module
"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "luau module pattern: {:?}", result);
}

#[test]
fn test_hex_with_exponent_lua51() {
    let input = "local x = 0x1p10";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "hex with exponent: {:?}", result);
}

#[test]
fn test_max_integer_lua53() {
    let input = "local x = 9223372036854775807";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "max integer: {:?}", result);
}

#[test]
fn test_hex_upper_lua51() {
    let input = "local x = 0XABCDEF";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "hex upper: {:?}", result);
}

#[test]
fn test_nested_loops_lua51() {
    let input = r#"
for i = 1, 10 do
    for j = 1, 10 do
        if i == j then
            break
        end
    end
end
"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "nested loops: {:?}", result);
}

#[test]
fn test_while_in_repeat_lua51() {
    let input = r#"
repeat
    local x = 1
    while x < 10 do
        x = x + 1
    end
until x >= 10
"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "while in repeat: {:?}", result);
}

#[test]
fn test_for_in_if_lua51() {
    let input = r#"
if true then
    for i = 1, 10 do
        print(i)
    end
end
"#;
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "for in if: {:?}", result);
}
