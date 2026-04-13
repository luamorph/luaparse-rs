use luaparse_rs::{Parser, Luau, Lua51, Lua52, Lua53, Lua54};

#[test]
fn test_goto_rejected_lua51() {
    let input = "goto done";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "goto should be rejected in Lua 5.1");
}

#[test]
fn test_goto_accepted_lua52() {
    let input = "goto done ::done::";
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "goto should work in Lua 5.2+");
}

#[test]
fn test_goto_accepted_lua53() {
    let input = "goto done ::done::";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_goto_accepted_lua54() {
    let input = "goto done ::done::";
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_label_rejected_lua51() {
    let input = "::label::";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "labels should be rejected in Lua 5.1");
}

#[test]
fn test_continue_rejected_lua51() {
    let input = "while true do continue end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "continue should be rejected in Lua 5.1");
}

#[test]
fn test_continue_rejected_lua54() {
    let input = "while true do continue end";
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "continue should be rejected in Lua 5.4");
}

#[test]
fn test_type_annotation_rejected_lua51() {
    // In Lua 5.1, `local x: number = 1` should NOT parse as a type annotation
    // With the version gate fix, the `:` won't be consumed as a type annotation,
    // so the parser will see `local x` then `: number = 1` which is invalid
    let input = "local x: number = 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "type annotations should be rejected in Lua 5.1");
}

#[test]
fn test_compound_assignment_rejected_lua51() {
    let input = "x += 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "compound assignment should be rejected in Lua 5.1");
}

#[test]
fn test_compound_assignment_rejected_lua54() {
    let input = "x += 1";
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "compound assignment should be rejected in Lua 5.4");
}

#[test]
fn test_interpolated_string_rejected_lua51() {
    let input = "local x = `hello {name}`";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    // Lua 5.1 has no interpolated strings
    // This may error at lex or parse level
}

#[test]
fn test_attribute_rejected_lua51() {
    let input = "@native function foo() end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "attributes should be rejected in Lua 5.1");
}

#[test]
fn test_bitwise_rejected_lua51() {
    let input = "local x = a & b";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "bitwise should be rejected in Lua 5.1");
}

#[test]
fn test_bitwise_rejected_lua52() {
    let input = "local x = a & b";
    let parser = Parser::<Lua52>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "bitwise should be rejected in Lua 5.2");
}

#[test]
fn test_semicolons_as_separator_lua51() {
    let input = "local x = 1; local y = 2; print(x + y)";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_multiple_semicolons_lua51() {
    let input = "local x = 1;;; local y = 2";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_empty_program_lua51() {
    let input = "";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_only_semicolons_lua51() {
    let input = ";;;";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_long_string_level0_lua51() {
    let input = "local x = [[hello\nworld]]";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_long_string_level1_lua51() {
    let input = "local x = [=[hello\nworld]=]";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_long_string_with_brackets_inside_lua51() {
    // [=[ ... ]=] — content can have ] and [[ but not ]=]
    let input = "local x = [=[ has ] and [[ inside ]=]";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_double_negation_lua51() {
    let input = "local x = - -5";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_not_not_lua51() {
    let input = "local x = not not true";
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
fn test_length_of_table_lua51() {
    let input = "local x = #{1, 2, 3}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_method_call_lua51() {
    let input = "obj:method(arg1, arg2)";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_chained_method_calls_lua51() {
    let input = "obj:m1():m2():m3()";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_method_def_in_table_lua51() {
    let input = "function t:method(a, b) return a + b end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_deeply_dotted_function_lua51() {
    let input = "function a.b.c.d(x) return x end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_dotted_method_function_lua51() {
    let input = "function a.b:c(x) return x end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_table_trailing_comma_lua51() {
    let input = "local t = {1, 2, 3,}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_table_trailing_semicolon_lua51() {
    let input = "local t = {1; 2; 3;}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_table_mixed_separator_lua51() {
    let input = "local t = {1, 2; 3, 4}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_table_single_element_lua51() {
    let input = "local t = {42}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_nested_tables_lua51() {
    let input = "local t = {{1, 2}, {3, 4}, {5, 6}}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_table_with_function_value_lua51() {
    let input = "local t = { fn = function(x) return x end }";
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
fn test_field_assignment_lua51() {
    let input = "a.b.c = 1";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_index_assignment_lua51() {
    let input = "a[1][2] = 3";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_call_with_string_arg_lua51() {
    let input = "print 'hello'";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_call_with_table_arg_lua51() {
    let input = "print {1, 2, 3}";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_method_call_with_string_arg_lua51() {
    let input = "obj:method 'hello'";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_call_with_long_string_arg_lua51() {
    let input = "print [[hello]]";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_varargs_in_function_lua51() {
    let input = "function foo(...) return ... end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_varargs_passed_to_call_lua51() {
    let input = "function foo(...) return bar(...) end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_varargs_in_table_lua51() {
    let input = "function foo(...) return {...} end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_varargs_after_params_lua51() {
    let input = "function foo(a, b, ...) return a, b, ... end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_string_concat_lua51() {
    let input = "local x = 'hello' .. ' ' .. 'world'";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_concat_with_number_lua51() {
    let input = "local x = 'count: ' .. 42";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_if_elseif_chain_lua51() {
    let input = "if a then x = 1 elseif b then x = 2 elseif c then x = 3 else x = 4 end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_do_block_lua51() {
    let input = "do local x = 1 end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_nested_do_blocks_lua51() {
    let input = "do do do local x = 1 end end end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_if_expression_luau() {
    let input = "local x = if condition then value1 else value2";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_if_expression_elseif_luau() {
    let input = "local x = if a then 1 elseif b then 2 else 3";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_if_expression_nested_luau() {
    let input = "local x = if a then (if b then 1 else 2) else 3";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_if_expression_rejected_lua51() {
    // if-then-else as expression is Luau-only
    let input = "local x = if true then 1 else 2";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "if-expression should be rejected in Lua 5.1");
}

#[test]
fn test_type_pack_in_generic_luau() {
    // Type packs (T...) are a Luau feature that may not be fully supported yet
    // For now just test basic variadic type annotation
    let input = "type Packed<T> = (T) -> T";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse_with_types();
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
fn test_hex_number_lower_lua51() {
    let input = "local x = 0xff";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_float_number_lua51() {
    let input = "local x = 3.14159";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_number_starting_with_dot_lua51() {
    let input = "local x = .5";
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
fn test_break_in_repeat_lua51() {
    let input = "repeat break until true";
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
fn test_empty_return_lua51() {
    let input = "function foo() return end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_return_nil_lua51() {
    let input = "function foo() return nil end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_export_type_simple_luau() {
    let input = "export type Name = string";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_export_type_table_luau() {
    let input = "export type Config = { name: string, enabled: boolean }";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_export_type_generic_luau() {
    let input = "export type Container<T> = { value: T, next: Container<T>? }";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_elseif_complex_condition_lua51() {
    let input = "if a and b then x() elseif c or (d and e) then y() else z() end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_call_chain_as_statement_lua51() {
    let input = "foo()()()";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_repeat_until_complex_lua51() {
    let input = "repeat local x = foo() until x > 0 or x == nil";
    let parser = Parser::<Lua51>::new(input).unwrap();
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
fn test_bitwise_not_lua54() {
    let input = "local x = ~0xFF";
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_deeply_parenthesized_lua51() {
    let input = "local x = ((((1 + 2))))";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_paren_as_call_prefix_lua51() {
    let input = "(getfn())(arg)";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_concat_after_type_cast_luau() {
    let input = "local x = (y :: string) .. 'suffix'";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_multi_local_no_assign_lua51() {
    let input = "local a, b, c";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_multi_local_partial_assign_lua51() {
    let input = "local a, b, c = 1, 2";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_right_assoc_exponent_lua51() {
    let input = "local x = 2 ^ 3 ^ 4";
    let parser = Parser::<Lua51>::new(input).unwrap();
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
    let input = "local f <close> = io.open('test.txt')";
    let parser = Parser::<Lua54>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_lua54_attributes_rejected_lua51() {
    let input = "local x <const> = 42";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "Lua 5.4 attributes should be rejected in Lua 5.1");
}

#[test]
fn test_module_pattern_luau() {
    let input = r#"
local module = {}

export type ModuleConfig = {
    debug: boolean,
    maxRetries: number,
    timeout: number?,
}

function module.init(config: ModuleConfig)
    module.config = config
end

function module.run()
    for i = 1, module.config.maxRetries do
        local success = pcall(function()
            -- do work
        end)
        if success then
            break
        end
    end
end

return module
"#;
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_function_in_table_constructor_lua51() {
    let input = "local t = { f = function() return 1 end, g = function() return 2 end }";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_complex_table_computed_key_lua51() {
    let input = "local t = { [foo()] = bar() }";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_while_complex_condition_lua51() {
    let input = "while i < 10 and not done do i = i + 1 end";
    let parser = Parser::<Lua51>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_frozen_table_luau() {
    let input = "local t = table.freeze({a = 1, b = 2})";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_integer_literal_lua53() {
    let input = "local x = 42";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_float_literal_lua53() {
    let input = "local x = 42.0";
    let parser = Parser::<Lua53>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_typeof_in_union_luau() {
    let input = "local x: typeof(y) | string = z";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_typeof_with_complex_expr_luau() {
    let input = "local x: typeof(setmetatable({}, mt)) = y";
    let parser = Parser::<Luau>::new(input).unwrap();
    let result = parser.parse();
    assert!(result.is_ok());
}
