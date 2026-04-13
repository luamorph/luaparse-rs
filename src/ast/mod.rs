//! All the types that make up a parsed Lua syntax tree.
//!
//! The main entry point is [`Ast`], which holds the top level block of statements
//! and any comments found in the source. For Luau code with type annotations,
//! use [`AstWithTypes`] instead.

use alloc::vec::Vec;

/// Shared building blocks: identifiers, blocks, comments, parameters.
pub mod common;
/// Expression nodes (`1 + 2`, `foo()`, `if/then/else`, etc.).
pub mod expr;
/// Statement nodes (`local`, `if`, `while`, `return`, etc.).
pub mod stmt;
/// Luau type annotation nodes (`string`, `number?`, `{x: number}`, etc.).
pub mod types;
/// Tree traversal with the [`Visitor`](visitor::Visitor) and [`VisitorMut`](visitor::VisitorMut) traits.
pub mod visitor;

pub use common::*;
pub use expr::*;
pub use stmt::*;
pub use types::*;

/// A complete parsed Lua program.
///
/// Contains the top level block of statements and every comment found in the
/// source. You get this from [`Parser::parse`](crate::Parser::parse).
#[derive(Debug, Clone, PartialEq)]
pub struct Ast {
    /// The top level block containing all statements in the program.
    pub block: Block,
    /// Every comment found in the source, in order of appearance.
    pub comments: Vec<Comment>,
}

impl Ast {
    pub fn new(block: Block, comments: Vec<Comment>) -> Self {
        Self { block, comments }
    }
}

/// A parsed Luau program with type declarations pulled out separately.
///
/// You get this from [`Parser::parse_with_types`](crate::Parser::parse_with_types).
/// The `type Foo = ...` and `export type Bar = ...` declarations live in
/// `type_declarations` instead of being mixed into the statement list.
#[derive(Debug, Clone, PartialEq)]
pub struct AstWithTypes {
    /// The syntax tree (statements and comments).
    pub ast: Ast,
    /// All `type` and `export type` declarations, with their full type expressions resolved.
    pub type_declarations: Vec<TypeDeclarationFull>,
}

impl AstWithTypes {
    pub fn new(ast: Ast, type_declarations: Vec<TypeDeclarationFull>) -> Self {
        Self { ast, type_declarations }
    }
}
