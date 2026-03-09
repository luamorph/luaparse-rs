#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use alloc::{string::String, vec::Vec};
use thiserror::Error;

pub mod ast;
pub mod parser;
pub mod lexer;
pub mod marker;

pub use ast::{Ast, AstWithTypes};
pub use lexer::InterpolationPart;

pub use marker::{Lua51, Lua52, Lua53, Lua54, LuaVersion, Luau};
pub use parser::Parser;

pub type Span = core::ops::Range<usize>;

#[derive(Debug, Error, Clone)]
pub enum ParseError {
    #[error("unexpected token: expected {expected:?}, found {found}")]
    UnexpectedToken {
        expected: Vec<String>,
        found: String,
        span: Span,
    },
    #[error("unexpected end of file: expected {expected:?}")]
    UnexpectedEof {
        expected: Vec<String>,
        span: Span,
    },
    #[error("{message}")]
    InvalidSyntax {
        message: String,
        span: Span,
        help: Option<String>,
    },
    #[error("feature '{feature}' not supported in {version}")]
    UnsupportedFeature {
        feature: String,
        version: String,
        span: Span,
    },
}

#[derive(Debug, Error, Clone)]
pub enum LexError {
    #[error("unterminated string literal")]
    UnterminatedString { span: Span },
    #[error("invalid number literal")]
    InvalidNumber { span: Span },
    #[error("unterminated comment")]
    UnterminatedComment { span: Span },
    #[error("invalid escape sequence: \\{escape}")]
    InvalidEscape { escape: char, span: Span },
}

#[cfg(feature = "wasm")]
pub mod wasm;
