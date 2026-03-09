use sealed::sealed;

#[sealed]
pub trait LuaVersion: 'static {
    const NAME: &'static str;
    const HAS_FLOOR_DIV: bool;
    const HAS_COMPOUND_ASSIGN: bool;
    const HAS_CONTINUE: bool;
    const HAS_IF_EXPR: bool;
    const HAS_STRING_INTERP: bool;
    const HAS_TYPE_ANNOTATIONS: bool;
    const HAS_ATTRIBUTES: bool;
    const HAS_GENERICS: bool;
    const HAS_EXPORT: bool;
    const HAS_GOTO: bool;
    const HAS_BITWISE_OPS: bool;
    const HAS_VARIABLE_ATTRIBUTES: bool; // Lua 5.4 <const>/<close> on local declarations
    const HAS_CONST: bool; // Luau `const x = 5` immutable bindings
}

// #[cfg(feature = "luau")]
#[derive(Debug, Clone, Copy)]
pub struct Luau;

// #[cfg(feature = "luau")]
#[sealed]
impl LuaVersion for Luau {
    const NAME: &'static str = "Luau";
    const HAS_FLOOR_DIV: bool = true;
    const HAS_COMPOUND_ASSIGN: bool = true;
    const HAS_CONTINUE: bool = true;
    const HAS_IF_EXPR: bool = true;
    const HAS_STRING_INTERP: bool = true;
    const HAS_TYPE_ANNOTATIONS: bool = true;
    const HAS_ATTRIBUTES: bool = true;
    const HAS_GENERICS: bool = true;
    const HAS_EXPORT: bool = true;
    const HAS_GOTO: bool = false;
    const HAS_BITWISE_OPS: bool = false;
    const HAS_VARIABLE_ATTRIBUTES: bool = false;
    const HAS_CONST: bool = true;
}

// #[cfg(feature = "lua51")]
#[derive(Debug, Clone, Copy)]
pub struct Lua51;

// #[cfg(feature = "lua51")]
#[sealed]
impl LuaVersion for Lua51 {
    const NAME: &'static str = "Lua 5.1";
    const HAS_FLOOR_DIV: bool = false;
    const HAS_COMPOUND_ASSIGN: bool = false;
    const HAS_CONTINUE: bool = false;
    const HAS_IF_EXPR: bool = false;
    const HAS_STRING_INTERP: bool = false;
    const HAS_TYPE_ANNOTATIONS: bool = false;
    const HAS_ATTRIBUTES: bool = false;
    const HAS_GENERICS: bool = false;
    const HAS_EXPORT: bool = false;
    const HAS_GOTO: bool = false;
    const HAS_BITWISE_OPS: bool = false;
    const HAS_VARIABLE_ATTRIBUTES: bool = false;
    const HAS_CONST: bool = false;
}

// #[cfg(feature = "lua52")]
#[derive(Debug, Clone, Copy)]
pub struct Lua52;

// #[cfg(feature = "lua52")]
#[sealed]
impl LuaVersion for Lua52 {
    const NAME: &'static str = "Lua 5.2";
    const HAS_FLOOR_DIV: bool = false;
    const HAS_COMPOUND_ASSIGN: bool = false;
    const HAS_CONTINUE: bool = false;
    const HAS_IF_EXPR: bool = false;
    const HAS_STRING_INTERP: bool = false;
    const HAS_TYPE_ANNOTATIONS: bool = false;
    const HAS_ATTRIBUTES: bool = false;
    const HAS_GENERICS: bool = false;
    const HAS_EXPORT: bool = false;
    const HAS_GOTO: bool = true;
    const HAS_BITWISE_OPS: bool = false;
    const HAS_VARIABLE_ATTRIBUTES: bool = false;
    const HAS_CONST: bool = false;
}

// #[cfg(feature = "lua53")]
#[derive(Debug, Clone, Copy)]
pub struct Lua53;

// #[cfg(feature = "lua53")]
#[sealed]
impl LuaVersion for Lua53 {
    const NAME: &'static str = "Lua 5.3"; // gross
    const HAS_FLOOR_DIV: bool = true;
    const HAS_COMPOUND_ASSIGN: bool = false;
    const HAS_CONTINUE: bool = false;
    const HAS_IF_EXPR: bool = false;
    const HAS_STRING_INTERP: bool = false;
    const HAS_TYPE_ANNOTATIONS: bool = false;
    const HAS_ATTRIBUTES: bool = false;
    const HAS_GENERICS: bool = false;
    const HAS_EXPORT: bool = false;
    const HAS_GOTO: bool = true;
    const HAS_BITWISE_OPS: bool = true;
    const HAS_VARIABLE_ATTRIBUTES: bool = false;
    const HAS_CONST: bool = false;
}

#[derive(Debug, Clone, Copy)]
pub struct Lua54;

#[sealed]
impl LuaVersion for Lua54 {
    const NAME: &'static str = "Lua 5.4";
    const HAS_FLOOR_DIV: bool = true;
    const HAS_COMPOUND_ASSIGN: bool = false;
    const HAS_CONTINUE: bool = false;
    const HAS_IF_EXPR: bool = false;
    const HAS_STRING_INTERP: bool = false;
    const HAS_TYPE_ANNOTATIONS: bool = false;
    const HAS_ATTRIBUTES: bool = true;
    const HAS_GENERICS: bool = false;
    const HAS_EXPORT: bool = false;
    const HAS_GOTO: bool = true;
    const HAS_BITWISE_OPS: bool = true;
    const HAS_VARIABLE_ATTRIBUTES: bool = true;
    const HAS_CONST: bool = false;
}
