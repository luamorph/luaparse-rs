//! Luau type annotation nodes.
//!
//! These types represent the Luau type system's syntax: named types,
//! unions, intersections, function types, table types, and more.
//! They only appear when parsing with [`Luau`](crate::Luau) and using
//! [`parse_with_types`](crate::Parser::parse_with_types).

use alloc::{boxed::Box, string::String, vec::Vec};

use crate::Span;
use super::common::Identifier;

/// A single Luau type expression.
///
/// Works the same way as [`Expr`](super::expr::Expr): a [`kind`](Self::kind)
/// telling you what it is, and a [`span`](Self::span) pointing into the source.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeExpr {
    /// What kind of type expression this is.
    pub kind: TypeExprKind,
    /// Where it appears in the source.
    pub span: Span,
}

impl TypeExpr {
    pub fn new(kind: TypeExprKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// All the different kinds of Luau type expression.
#[derive(Debug, Clone, PartialEq)]
pub enum TypeExprKind {
    /// The `nil` type.
    Nil,
    /// A singleton boolean type (`true` or `false`).
    Boolean(bool),
    /// A singleton string type (e.g. `"success"`).
    String(String),
    /// A singleton number type.
    Number(String),

    /// A named type, possibly with a module path and generics.
    ///
    /// Examples: `string`, `React.Element<Props>`, `Module.Type`.
    Named {
        /// The dotted path (e.g. `[React, Element]`).
        path: Vec<Identifier>,
        /// Optional generic type arguments.
        generics: Option<Vec<TypeExpr>>,
    },

    /// A table type: `{x: number, y: number}`.
    Table(Box<TableType>),
    /// A function type: `(number, string) -> boolean`.
    Function(Box<FunctionType>),

    /// A union type: `string | number`.
    Union(Vec<TypeExpr>),
    /// An intersection type: `Readable & Writable`.
    Intersection(Vec<TypeExpr>),

    /// An optional type: `string?` (shorthand for `string | nil`).
    Optional(Box<TypeExpr>),

    /// A `typeof(expr)` type.
    Typeof(Box<super::expr::Expr>),

    /// A generic type pack: `T...`.
    GenericPack(Identifier),
    /// A variadic type pack: `...number`.
    VariadicPack(Box<TypeExpr>),

    /// A parenthesized type: `(string)`.
    Parenthesized(Box<TypeExpr>),
}

/// A Luau table type: `{x: number, y: number, [string]: any}`.
#[derive(Debug, Clone, PartialEq)]
pub struct TableType {
    /// Named properties (e.g. `x: number`).
    pub properties: Vec<TableProperty>,
    /// An optional indexer (e.g. `[string]: any`).
    pub indexer: Option<Box<TableIndexer>>,
    /// Where it appears in the source.
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

/// A single property in a table type: `name: Type`.
#[derive(Debug, Clone, PartialEq)]
pub struct TableProperty {
    /// The property name.
    pub name: Identifier,
    /// The property's type.
    pub type_expr: TypeExpr,
    /// Optional `read` or `write` modifier.
    pub read_write: Option<ReadWrite>,
    /// Where it appears in the source.
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

/// A read/write modifier on a table property.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadWrite {
    /// `read` access only.
    Read,
    /// `write` access only.
    Write,
}

/// An indexer in a table type: `[KeyType]: ValueType`.
#[derive(Debug, Clone, PartialEq)]
pub struct TableIndexer {
    /// The key type (e.g. `string` in `[string]: any`).
    pub key_type: Box<TypeExpr>,
    /// The value type.
    pub value_type: Box<TypeExpr>,
    /// Optional `read` or `write` modifier.
    pub read_write: Option<ReadWrite>,
    /// Where it appears in the source.
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

/// A function type: `<T>(x: T, y: number) -> T`.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionType {
    /// Generic type parameters (e.g. `<T, U>`).
    pub generics: Vec<GenericParameter>,
    /// The parameter types.
    pub parameters: Vec<FunctionTypeParameter>,
    /// The return type.
    pub return_type: Box<TypeExpr>,
    /// Where it appears in the source.
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

/// A single parameter in a function type.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionTypeParameter {
    /// An optional parameter name (e.g. `x` in `x: number`).
    pub name: Option<Identifier>,
    /// The parameter's type.
    pub type_expr: TypeExpr,
    /// Where it appears in the source.
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

/// A generic type parameter in a declaration or function type.
///
/// Examples: `T`, `T...` (type pack), `T = string` (with default).
#[derive(Debug, Clone, PartialEq)]
pub struct GenericParameter {
    /// The parameter name.
    pub name: Identifier,
    /// An optional constraint type.
    pub constraint: Option<TypeExpr>,
    /// An optional default type.
    pub default: Option<TypeExpr>,
    /// `true` if this is a type pack parameter (`T...`).
    pub is_pack: bool,
    /// Where it appears in the source.
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

/// A fully resolved `type` or `export type` declaration.
///
/// Unlike [`TypeDeclaration`](super::stmt::TypeDeclaration), which only stores
/// spans, this variant includes the actual parsed type expression. You get
/// these from [`AstWithTypes::type_declarations`](super::AstWithTypes::type_declarations).
#[derive(Debug, Clone, PartialEq)]
pub struct TypeDeclarationFull {
    /// Whether this was declared with `export`.
    pub exported: bool,
    /// The name of the type alias.
    pub name: Identifier,
    /// The generic parameters.
    pub generics: Vec<GenericParameter>,
    /// The right side type expression.
    pub type_expr: TypeExpr,
    /// Where it appears in the source.
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
