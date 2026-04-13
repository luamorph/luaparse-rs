#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use thiserror::Error;

/// The syntax tree types: statements, expressions, types, and the visitor.
pub mod ast;
/// The parser that turns source code into an [`Ast`].
pub mod parser;
/// The tokenizer that breaks source text into tokens.
pub mod lexer;
/// Lua version markers ([`Luau`], [`Lua51`], [`Lua52`], [`Lua53`], [`Lua54`]).
pub mod marker;

pub use ast::{Ast, AstWithTypes};
pub use lexer::InterpolationPart;

pub use marker::{Lua51, Lua52, Lua53, Lua54, LuaVersion, Luau};
pub use parser::Parser;

/// A byte range in the source code (`start..end`).
///
/// Every node in the syntax tree carries a span so you can trace it back to
/// a position in the original source string.
pub type Span = core::ops::Range<usize>;

/// Something went wrong while parsing.
///
/// Each variant includes a [`Span`] pointing to where the problem is in the source.
#[derive(Debug, Error, Clone)]
pub enum ParseError {
    /// Found a token the parser wasn't expecting.
    #[error("unexpected token: expected {expected:?}, found {found}")]
    UnexpectedToken {
        /// What the parser was looking for.
        expected: Vec<String>,
        /// What it actually found.
        found: String,
        /// Where in the source.
        span: Span,
    },
    /// The source ended before the parser finished.
    #[error("unexpected end of file: expected {expected:?}")]
    UnexpectedEof {
        /// What the parser still needed.
        expected: Vec<String>,
        /// Points to the end of the source.
        span: Span,
    },
    /// The code is structurally wrong.
    #[error("{message}")]
    InvalidSyntax {
        /// What went wrong.
        message: String,
        /// Where in the source.
        span: Span,
        /// An optional suggestion for how to fix it.
        help: Option<String>,
    },
    /// A feature was used that doesn't exist in the chosen Lua version.
    ///
    /// For example, using `continue` when parsing as Lua 5.1.
    #[error("feature '{feature}' not supported in {version}")]
    UnsupportedFeature {
        /// Name of the unsupported feature.
        feature: String,
        /// Which Lua version was selected.
        version: String,
        /// Where in the source.
        span: Span,
    },
}

/// Something went wrong during lexing (tokenization).
///
/// These errors happen before parsing even starts. The source text couldn't
/// be broken into valid tokens.
#[derive(Debug, Error, Clone)]
pub enum LexError {
    /// A string literal was opened but never closed.
    #[error("unterminated string literal")]
    UnterminatedString { span: Span },
    /// A number literal couldn't be understood.
    #[error("invalid number literal")]
    InvalidNumber { span: Span },
    /// A block comment (`--[[ ... ]]`) was opened but never closed.
    #[error("unterminated comment")]
    UnterminatedComment { span: Span },
    /// A string contains a backslash sequence that isn't valid.
    #[error("invalid escape sequence: \\{escape}")]
    InvalidEscape { escape: char, span: Span },
}

#[cfg(feature = "wasm")]
pub mod wasm;
