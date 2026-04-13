//! Shared types used across the syntax tree: identifiers, blocks, comments, and parameters.

use alloc::{string::String, vec::Vec};

use crate::Span;
use super::stmt::Stmt;

/// A name in the source code, like a variable or function name.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    /// The actual text of the name.
    pub name: String,
    /// Where it appears in the source.
    pub span: Span,
}

impl Identifier {
    pub fn new(name: String, span: Span) -> Self {
        Self { name, span }
    }
}

/// A variable name that may have an attribute attached.
///
/// In Lua 5.4, local variables can have attributes like `<const>` or `<close>`.
/// For example: `local f <close> = io.open("file")`.
#[derive(Debug, Clone, PartialEq)]
pub struct VariableName {
    /// The variable's name.
    pub name: Identifier,
    /// An optional attribute (e.g. `const` or `close` in Lua 5.4).
    pub attribute: Option<Identifier>,
}

impl VariableName {
    pub fn new(name: Identifier, attribute: Option<Identifier>) -> Self {
        Self { name, attribute }
    }
    
    pub fn simple(name: Identifier) -> Self {
        Self { name, attribute: None }
    }
}

/// A sequence of statements, like the body of a function or loop.
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    /// The statements in this block, in order.
    pub statements: Vec<Stmt>,
    /// The span covering the entire block.
    pub span: Span,
}

impl Block {
    pub fn new(statements: Vec<Stmt>, span: Span) -> Self {
        Self { statements, span }
    }
    
    pub fn empty(span: Span) -> Self {
        Self {
            statements: Vec::new(),
            span,
        }
    }
}

/// A comment found in the source code.
///
/// Both line comments (`-- hello`) and block comments (`--[[ hello ]]`) are captured.
#[derive(Debug, Clone, PartialEq)]
pub struct Comment {
    /// The text of the comment (without the `--` prefix or block delimiters).
    pub content: String,
    /// `true` for block comments (`--[[ ]]`), `false` for line comments (`--`).
    pub is_block: bool,
    /// Where it appears in the source.
    pub span: Span,
}

impl Comment {
    pub fn new(content: String, is_block: bool, span: Span) -> Self {
        Self { content, is_block, span }
    }
}

/// A type annotation's source location.
///
/// This is a lightweight marker that just records where the annotation is
/// in the source. The actual type expression lives in the Luau type system;
/// see [`TypeExpr`](super::types::TypeExpr) for the full representation.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeAnnotation {
    /// The span covering the entire type annotation.
    pub span: Span,
}

impl TypeAnnotation {
    pub fn new(span: Span) -> Self {
        Self { span }
    }
}

/// A function parameter.
///
/// Can be a named parameter, a variadic (`...`), or (in Luau) a typed parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    /// The parameter name, if it has one. Varargs (`...`) have no name.
    pub name: Option<Identifier>,
    /// An optional attribute on the parameter.
    pub attribute: Option<Identifier>,
    /// An optional Luau type annotation (e.g. `: string`).
    pub type_annotation: Option<TypeAnnotation>,
    /// `true` if this is a `...` vararg parameter.
    pub is_vararg: bool,
    /// Where it appears in the source.
    pub span: Span,
}

impl Parameter {
    pub fn new(
        name: Option<Identifier>,
        attribute: Option<Identifier>,
        type_annotation: Option<TypeAnnotation>,
        is_vararg: bool,
        span: Span,
    ) -> Self {
        Self {
            name,
            attribute,
            type_annotation,
            is_vararg,
            span,
        }
    }
    
    pub fn vararg(span: Span) -> Self {
        Self {
            name: None,
            attribute: None,
            type_annotation: None,
            is_vararg: true,
            span,
        }
    }
    
    pub fn identifier(name: Identifier) -> Self {
        let span = name.span.clone();
        Self {
            name: Some(name),
            attribute: None,
            type_annotation: None,
            is_vararg: false,
            span,
        }
    }
}
