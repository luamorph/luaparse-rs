use alloc::{string::String, vec::Vec};

use crate::Span;
use super::stmt::Stmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub name: String,
    pub span: Span,
}

impl Identifier {
    pub fn new(name: String, span: Span) -> Self {
        Self { name, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableName {
    pub name: Identifier,
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

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Stmt>,
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

#[derive(Debug, Clone, PartialEq)]
pub struct Comment {
    pub content: String,
    pub is_block: bool,
    pub span: Span,
}

impl Comment {
    pub fn new(content: String, is_block: bool, span: Span) -> Self {
        Self { content, is_block, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAnnotation {
    pub span: Span,
}

impl TypeAnnotation {
    pub fn new(span: Span) -> Self {
        Self { span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: Option<Identifier>,
    pub attribute: Option<Identifier>,
    pub type_annotation: Option<TypeAnnotation>,
    pub is_vararg: bool,
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