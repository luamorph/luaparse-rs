use alloc::{boxed::Box, string::String, vec::Vec};

use crate::Span;
use super::common::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeExpr {
    pub kind: TypeExprKind,
    pub span: Span,
}

impl TypeExpr {
    pub fn new(kind: TypeExprKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeExprKind {
    Nil,
    Boolean(bool),
    String(String),
    Number(String),
    
    Named {
        path: Vec<Identifier>,
        generics: Option<Vec<TypeExpr>>,
    },
    
    Table(Box<TableType>),  // Box here
    Function(Box<FunctionType>),  // Box here
    
    Union(Vec<TypeExpr>),
    Intersection(Vec<TypeExpr>),
    
    Optional(Box<TypeExpr>),
    
    Typeof(Box<super::expr::Expr>),
    
    GenericPack(Identifier),
    VariadicPack(Box<TypeExpr>),
    
    Parenthesized(Box<TypeExpr>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableType {
    pub properties: Vec<TableProperty>,
    pub indexer: Option<Box<TableIndexer>>,
    pub span: Span,
}

impl TableType {
    pub fn new(properties: Vec<TableProperty>, indexer: Option<Box<TableIndexer>>, span: Span) -> Self {
        Self {
            properties,
            indexer,
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableProperty {
    pub name: Identifier,
    pub type_expr: TypeExpr,
    pub read_write: Option<ReadWrite>,
    pub span: Span,
}

impl TableProperty {
    pub fn new(
        name: Identifier,
        type_expr: TypeExpr,
        read_write: Option<ReadWrite>,
        span: Span,
    ) -> Self {
        Self {
            name,
            type_expr,
            read_write,
            span,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadWrite {
    Read,
    Write,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableIndexer {
    pub key_type: Box<TypeExpr>,
    pub value_type: Box<TypeExpr>,
    pub read_write: Option<ReadWrite>,
    pub span: Span,
}

impl TableIndexer {
    pub fn new(
        key_type: TypeExpr,
        value_type: TypeExpr,
        read_write: Option<ReadWrite>,
        span: Span,
    ) -> Self {
        Self {
            key_type: Box::new(key_type),
            value_type: Box::new(value_type),
            read_write,
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionType {
    pub generics: Vec<GenericParameter>,
    pub parameters: Vec<FunctionTypeParameter>,
    pub return_type: Box<TypeExpr>,
    pub span: Span,
}

impl FunctionType {
    pub fn new(
        generics: Vec<GenericParameter>,
        parameters: Vec<FunctionTypeParameter>,
        return_type: TypeExpr,
        span: Span,
    ) -> Self {
        Self {
            generics,
            parameters,
            return_type: Box::new(return_type),
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionTypeParameter {
    pub name: Option<Identifier>,
    pub type_expr: TypeExpr,
    pub span: Span,
}

impl FunctionTypeParameter {
    pub fn new(name: Option<Identifier>, type_expr: TypeExpr, span: Span) -> Self {
        Self {
            name,
            type_expr,
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericParameter {
    pub name: Identifier,
    pub constraint: Option<TypeExpr>,
    pub default: Option<TypeExpr>,
    pub is_pack: bool,
    pub span: Span,
}

impl GenericParameter {
    pub fn new(
        name: Identifier,
        constraint: Option<TypeExpr>,
        default: Option<TypeExpr>,
        is_pack: bool,
        span: Span,
    ) -> Self {
        Self {
            name,
            constraint,
            default,
            is_pack,
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDeclarationFull {
    pub exported: bool,
    pub name: Identifier,
    pub generics: Vec<GenericParameter>,
    pub type_expr: TypeExpr,
    pub span: Span,
}

impl TypeDeclarationFull {
    pub fn new(
        exported: bool,
        name: Identifier,
        generics: Vec<GenericParameter>,
        type_expr: TypeExpr,
        span: Span,
    ) -> Self {
        Self {
            exported,
            name,
            generics,
            type_expr,
            span,
        }
    }
}