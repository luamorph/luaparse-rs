//! Expression nodes in the syntax tree.

use alloc::{boxed::Box, string::String, vec::Vec};

use crate::Span;
use super::common::{Identifier, Block, Parameter, TypeAnnotation};

/// A single parsed expression.
///
/// Every expression has a [`kind`](Self::kind) that tells you what it is,
/// and a [`span`](Self::span) pointing back into the source.
#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    /// What kind of expression this is.
    pub kind: ExprKind,
    /// Where it appears in the source.
    pub span: Span,
}

impl Expr {
    pub fn new(kind: ExprKind, span: Span) -> Self {
        Self { kind, span }
    }
    
    pub fn synthetic(kind: ExprKind) -> Self {
        Self { kind, span: 0..0 }
    }
}

/// All the different kinds of expression the parser can produce.
#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    /// The `nil` literal.
    Nil,
    /// `true` or `false`.
    Boolean(bool),
    /// A number like `42`, `3.14`, or `0xFF`.
    Number(NumberLiteral),
    /// A string like `"hello"` or `'world'`.
    String(StringLiteral),
    /// The `...` vararg expression.
    Vararg,

    /// A table constructor: `{1, 2, key = "val"}`.
    Table(TableConstructor),
    /// An anonymous function: `function(x) return x end`.
    Function(FunctionExpr),

    /// A variable reference: `foo`.
    Identifier(Identifier),
    /// A dot field access: `obj.field`.
    FieldAccess(FieldAccess),
    /// A bracket index: `tbl[key]`.
    IndexAccess(IndexAccess),

    /// A unary operation: `-x`, `not x`, `#t`, `~x`.
    Unary(UnaryExpr),
    /// A binary operation: `a + b`, `x and y`, `s .. t`.
    Binary(BinaryExpr),

    /// A function call: `foo(1, 2)`.
    Call(CallExpr),
    /// A method call: `obj:method(1, 2)`.
    MethodCall(MethodCallExpr),

    /// A Luau if expression: `if cond then a else b`.
    IfExpression(IfExpression),
    /// A Luau interpolated string: `` `hello {name}` ``.
    InterpolatedString(InterpolatedString),
    /// A Luau type assertion: `expr :: Type`.
    TypeAssertion(TypeAssertion),
    /// A Luau explicit type instantiation: `f<<number>>(5)`.
    TypeInstantiation(TypeInstantiation),

    /// A parenthesized expression: `(expr)`.
    Parenthesized(Box<Expr>),
}

/// A number literal, stored as the raw source text.
///
/// Use [`parse_f64`](Self::parse_f64) to get the numeric value.
#[derive(Debug, Clone, PartialEq)]
pub struct NumberLiteral {
    /// The raw text from the source (e.g. `"0xFF"`, `"3.14e2"`).
    pub raw: String,
    /// Where it appears in the source.
    pub span: Span,
}

impl NumberLiteral {
    pub fn new(raw: String, span: Span) -> Self {
        Self { raw, span }
    }
    
    pub fn parse_f64(&self) -> Option<f64> {
        let cleaned = self.raw.replace('_', "");
        
        if cleaned.starts_with("0x") || cleaned.starts_with("0X") {
            // TODO
            u64::from_str_radix(&cleaned[2..].split('.').next()?, 16)
                .ok()
                .map(|v| v as f64)
        } else if cleaned.starts_with("0b") || cleaned.starts_with("0B") {
            u64::from_str_radix(&cleaned[2..], 2)
                .ok()
                .map(|v| v as f64)
        } else {
            cleaned.parse().ok()
        }
    }
}

/// A string literal with escape sequences already resolved.
#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral {
    /// The string content after processing escapes.
    pub value: String,
    /// Where it appears in the source.
    pub span: Span,
}

impl StringLiteral {
    pub fn new(value: String, span: Span) -> Self {
        Self { value, span }
    }
}

/// A table constructor: `{1, 2, key = "val", [expr] = val}`.
#[derive(Debug, Clone, PartialEq)]
pub struct TableConstructor {
    /// The fields in the table, in order.
    pub fields: Vec<TableField>,
    /// Where it appears in the source.
    pub span: Span,
}

impl TableConstructor {
    pub fn new(fields: Vec<TableField>, span: Span) -> Self {
        Self { fields, span }
    }
}

/// A single entry in a table constructor.
#[derive(Debug, Clone, PartialEq)]
pub struct TableField {
    /// What kind of table entry this is.
    pub kind: TableFieldKind,
    /// Where it appears in the source.
    pub span: Span,
}

impl TableField {
    pub fn new(kind: TableFieldKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// The different ways to define a table entry.
#[derive(Debug, Clone, PartialEq)]
pub enum TableFieldKind {
    /// `[expr] = value`.
    Bracketed { key: Expr, value: Expr },
    /// `name = value`.
    Named { name: Identifier, value: Expr },
    /// A positional value (no key), like `{1, 2, 3}`.
    Positional(Expr),
}

/// An anonymous function expression: `function(x, y) return x + y end`.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionExpr {
    /// The parameter list.
    pub parameters: Vec<Parameter>,
    /// An optional return type annotation (Luau).
    pub return_type: Option<TypeAnnotation>,
    /// The function body.
    pub body: Block,
    /// Where it appears in the source.
    pub span: Span,
}

impl FunctionExpr {
    pub fn new(
        parameters: Vec<Parameter>,
        return_type: Option<TypeAnnotation>,
        body: Block,
        span: Span,
    ) -> Self {
        Self {
            parameters,
            return_type,
            body,
            span,
        }
    }
}

/// A dot field access: `obj.field`.
#[derive(Debug, Clone, PartialEq)]
pub struct FieldAccess {
    /// The expression being accessed.
    pub base: Box<Expr>,
    /// The field name after the dot.
    pub field: Identifier,
    /// Where it appears in the source.
    pub span: Span,
}

impl FieldAccess {
    pub fn new(base: Expr, field: Identifier, span: Span) -> Self {
        Self {
            base: Box::new(base),
            field,
            span,
        }
    }
}

/// A bracket index: `tbl[key]`.
#[derive(Debug, Clone, PartialEq)]
pub struct IndexAccess {
    /// The expression being indexed.
    pub base: Box<Expr>,
    /// The index expression inside the brackets.
    pub index: Box<Expr>,
    /// Where it appears in the source.
    pub span: Span,
}

impl IndexAccess {
    pub fn new(base: Expr, index: Expr, span: Span) -> Self {
        Self {
            base: Box::new(base),
            index: Box::new(index),
            span,
        }
    }
}

/// A unary operation like `-x` or `not cond`.
#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpr {
    /// Which operator.
    pub operator: UnaryOperator,
    /// The expression it applies to.
    pub operand: Box<Expr>,
    /// Where it appears in the source.
    pub span: Span,
}

impl UnaryExpr {
    pub fn new(operator: UnaryOperator, operand: Expr, span: Span) -> Self {
        Self {
            operator,
            operand: Box::new(operand),
            span,
        }
    }
}

/// The unary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    /// `-` (negation).
    Minus,
    /// `not` (logical negation).
    Not,
    /// `#` (length).
    Length,
    /// `~` (bitwise not, Lua 5.3+).
    BitwiseNot,
}

impl UnaryOperator {
    pub const fn binding_power(self) -> u8 {
        14
    }
}

/// A binary operation like `a + b` or `x and y`.
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    /// Which operator.
    pub operator: BinaryOperator,
    /// The left operand.
    pub left: Box<Expr>,
    /// The right operand.
    pub right: Box<Expr>,
    /// Where it appears in the source.
    pub span: Span,
}

impl BinaryExpr {
    pub fn new(operator: BinaryOperator, left: Expr, right: Expr, span: Span) -> Self {
        Self {
            operator,
            left: Box::new(left),
            right: Box::new(right),
            span,
        }
    }
}

/// All the binary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    /// `+`
    Add,
    /// Subtraction.
    Subtract,
    /// `*`
    Multiply,
    /// `/`
    Divide,
    /// `//` (Lua 5.3+, Luau).
    FloorDiv,
    /// `%`
    Modulo,
    /// `^`
    Power,

    /// `..` (string concatenation).
    Concat,

    /// `==`
    Equal,
    /// `~=`
    NotEqual,
    /// `<`
    Less,
    /// `<=`
    LessEqual,
    /// `>`
    Greater,
    /// `>=`
    GreaterEqual,

    /// `and`
    And,
    /// `or`
    Or,

    /// `&` (Lua 5.3+).
    BitwiseAnd,
    /// `|` (Lua 5.3+).
    BitwiseOr,
    /// `~` (bitwise xor, Lua 5.3+).
    BitwiseXor,
    /// `<<` (Lua 5.3+).
    LeftShift,
    /// `>>` (Lua 5.3+).
    RightShift,
}

impl BinaryOperator {
    pub const fn binding_power(self) -> (u8, u8) {
        match self {
            Self::Or => (1, 2),
            Self::And => (3, 4),
            Self::Equal | Self::NotEqual | Self::Less | Self::LessEqual
            | Self::Greater | Self::GreaterEqual => (5, 6),
            Self::BitwiseOr => (6, 7),
            Self::BitwiseXor => (7, 8),
            Self::BitwiseAnd => (8, 9),
            Self::LeftShift | Self::RightShift => (9, 10),
            Self::Concat => (10, 9),
            Self::Add | Self::Subtract => (11, 12),
            Self::Multiply | Self::Divide | Self::FloorDiv | Self::Modulo => (13, 14),
            Self::Power => (16, 15),
        }
    }
    
    pub const fn is_right_associative(self) -> bool {
        matches!(self, Self::Concat | Self::Power)
    }
}

/// A function call: `foo(1, 2)` or `foo "hello"` or `foo {1,2}`.
#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr {
    /// The expression being called.
    pub function: Box<Expr>,
    /// The arguments passed.
    pub arguments: Vec<Expr>,
    /// Where it appears in the source.
    pub span: Span,
}

impl CallExpr {
    pub fn new(function: Expr, arguments: Vec<Expr>, span: Span) -> Self {
        Self {
            function: Box::new(function),
            arguments,
            span,
        }
    }
}

/// A method call: `obj:method(1, 2)`.
#[derive(Debug, Clone, PartialEq)]
pub struct MethodCallExpr {
    /// The object being called on.
    pub base: Box<Expr>,
    /// The method name.
    pub method: Identifier,
    /// The arguments passed.
    pub arguments: Vec<Expr>,
    /// Where it appears in the source.
    pub span: Span,
}

impl MethodCallExpr {
    pub fn new(base: Expr, method: Identifier, arguments: Vec<Expr>, span: Span) -> Self {
        Self {
            base: Box::new(base),
            method,
            arguments,
            span,
        }
    }
}

/// A Luau if/then/else expression: `if cond then a elseif cond2 then b else c`.
#[derive(Debug, Clone, PartialEq)]
pub struct IfExpression {
    /// The condition after `if`.
    pub condition: Box<Expr>,
    /// The value when the condition is true.
    pub then_branch: Box<Expr>,
    /// Zero or more `elseif` branches.
    pub elseif_branches: Vec<ElseIfExprBranch>,
    /// The `else` value (always present in if expressions).
    pub else_branch: Box<Expr>,
    /// Where it appears in the source.
    pub span: Span,
}

impl IfExpression {
    pub fn new(
        condition: Expr,
        then_branch: Expr,
        elseif_branches: Vec<ElseIfExprBranch>,
        else_branch: Expr,
        span: Span,
    ) -> Self {
        Self {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            elseif_branches,
            else_branch: Box::new(else_branch),
            span,
        }
    }
}

/// One `elseif cond then value` branch inside an [`IfExpression`].
#[derive(Debug, Clone, PartialEq)]
pub struct ElseIfExprBranch {
    /// The condition after `elseif`.
    pub condition: Expr,
    /// The value when this condition is true.
    pub then_branch: Expr,
}

impl ElseIfExprBranch {
    pub fn new(condition: Expr, then_branch: Expr) -> Self {
        Self {
            condition,
            then_branch,
        }
    }
}

/// A Luau interpolated string: `` `hello {name}, you are {age}` ``.
#[derive(Debug, Clone, PartialEq)]
pub struct InterpolatedString {
    /// The alternating text and expression segments.
    pub segments: Vec<InterpolationSegment>,
    /// Where it appears in the source.
    pub span: Span,
}

impl InterpolatedString {
    pub fn new(segments: Vec<InterpolationSegment>, span: Span) -> Self {
        Self { segments, span }
    }
}

/// A piece of an interpolated string.
#[derive(Debug, Clone, PartialEq)]
pub enum InterpolationSegment {
    /// A literal text segment between expressions.
    Text(String),
    /// An embedded `{expression}`.
    Expression(Expr),
}

/// A Luau type assertion: `expr :: Type`.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeAssertion {
    /// The expression being asserted.
    pub expression: Box<Expr>,
    /// The type being asserted.
    pub type_annotation: TypeAnnotation,
    /// Where it appears in the source.
    pub span: Span,
}

impl TypeAssertion {
    pub fn new(expression: Expr, type_annotation: TypeAnnotation, span: Span) -> Self {
        Self {
            expression: Box::new(expression),
            type_annotation,
            span,
        }
    }
}

/// A Luau explicit type instantiation: `f<<number>>(5)`.
///
/// The `<<` and `>>` delimiters are split by the parser into individual
/// angle brackets so the type arguments can be parsed normally
#[derive(Debug, Clone, PartialEq)]
pub struct TypeInstantiation {
    /// The expression being instantiated (e.g. the function reference).
    pub expr: Box<Expr>,
    /// The span covering the type arguments between `<<` and `>>`.
    pub type_annotation: TypeAnnotation,
    /// Where it appears in the source.
    pub span: Span,
}

impl TypeInstantiation {
    pub fn new(expr: Expr, type_annotation: TypeAnnotation, span: Span) -> Self {
        Self {
            expr: Box::new(expr),
            type_annotation,
            span,
        }
    }
}
