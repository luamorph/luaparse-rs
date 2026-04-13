//! Lua version markers that control which syntax features the parser accepts.
//!
//! Pass one of these as the type parameter to [`Parser`](crate::Parser) to
//! select which grammar rules apply.

use sealed::sealed;

/// Describes which syntax features a Lua version supports.
///
/// This is a sealed trait; you cannot implement it for your own types.
/// Pick one of the provided versions: [`Luau`], [`Lua51`], [`Lua52`],
/// [`Lua53`], or [`Lua54`].
#[sealed]
pub trait LuaVersion: 'static {
    /// A display name for this version (e.g. `"Lua 5.4"`, `"Luau"`).
    const NAME: &'static str;
    /// `//` integer division operator.
    const HAS_FLOOR_DIV: bool;
    /// Compound assignment operators (`+=`, `-=`, etc.).
    const HAS_COMPOUND_ASSIGN: bool;
    /// The `continue` keyword inside loops.
    const HAS_CONTINUE: bool;
    /// Inline `if cond then a else b` expressions.
    const HAS_IF_EXPR: bool;
    /// Backtick interpolated strings (`` `hello {name}` ``).
    const HAS_STRING_INTERP: bool;
    /// Type annotations (`: Type`, `-> ReturnType`).
    const HAS_TYPE_ANNOTATIONS: bool;
    /// `@native` and similar function attributes.
    const HAS_ATTRIBUTES: bool;
    /// Generic type parameters (`<T>`).
    const HAS_GENERICS: bool;
    /// `export type` declarations.
    const HAS_EXPORT: bool;
    /// `goto` and `::label::` statements.
    const HAS_GOTO: bool;
    /// Bitwise operators (`&`, `|`, `~`, `<<`, `>>`).
    const HAS_BITWISE_OPS: bool;
    /// `<const>` and `<close>` variable attributes (Lua 5.4).
    const HAS_VARIABLE_ATTRIBUTES: bool;
    /// `local const x = 5` immutable bindings (Luau).
    const HAS_CONST: bool;
}

/// Roblox's Luau dialect.
///
/// Supports type annotations, if expressions, string interpolation,
/// compound assignment, `continue`, `@native` attributes, generics,
/// `export type`, and `const` bindings. Does not have `goto` or bitwise operators.
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

/// Standard Lua 5.1.
///
/// The baseline version. No floor division, no compound assignment,
/// no `goto`, no bitwise operators, no type annotations.
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

/// Standard Lua 5.2.
///
/// Adds `goto` and `::label::` over Lua 5.1. No floor division,
/// no compound assignment, no bitwise operators.
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

/// Standard Lua 5.3.
///
/// Adds `//` floor division, bitwise operators (`&`, `|`, `~`, `<<`, `>>`),
/// and `goto` over Lua 5.2.
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

/// Standard Lua 5.4.
///
/// Adds `<const>` and `<close>` variable attributes and `@` function
/// attributes over Lua 5.3. Keeps floor division, bitwise operators, and `goto`.
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
