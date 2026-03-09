use alloc::{boxed::Box, string::String, vec::Vec};

use crate::Span;
use super::common::{Identifier, Block, Parameter, TypeAnnotation};

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
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

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Nil,
    Boolean(bool),
    Number(NumberLiteral),
    String(StringLiteral),
    Vararg,
    
    Table(TableConstructor),
    Function(FunctionExpr),
    
    Identifier(Identifier),
    FieldAccess(FieldAccess),
    IndexAccess(IndexAccess),
    
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    
    Call(CallExpr),
    MethodCall(MethodCallExpr),
    
    IfExpression(IfExpression),
    InterpolatedString(InterpolatedString),
    TypeAssertion(TypeAssertion),
    
    Parenthesized(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct NumberLiteral {
    pub raw: String,
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

#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral {
    pub value: String,
    pub span: Span,
}

impl StringLiteral {
    pub fn new(value: String, span: Span) -> Self {
        Self { value, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableConstructor {
    pub fields: Vec<TableField>,
    pub span: Span,
}

impl TableConstructor {
    pub fn new(fields: Vec<TableField>, span: Span) -> Self {
        Self { fields, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableField {
    pub kind: TableFieldKind,
    pub span: Span,
}

impl TableField {
    pub fn new(kind: TableFieldKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TableFieldKind {
    Bracketed { key: Expr, value: Expr },
    Named { name: Identifier, value: Expr },
    Positional(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionExpr {
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeAnnotation>,
    pub body: Block,
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

#[derive(Debug, Clone, PartialEq)]
pub struct FieldAccess {
    pub base: Box<Expr>,
    pub field: Identifier,
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

#[derive(Debug, Clone, PartialEq)]
pub struct IndexAccess {
    pub base: Box<Expr>,
    pub index: Box<Expr>,
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

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpr {
    pub operator: UnaryOperator,
    pub operand: Box<Expr>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Minus,
    Not,
    Length,
    BitwiseNot,
}

impl UnaryOperator {
    pub const fn binding_power(self) -> u8 {
        14
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    pub operator: BinaryOperator,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    FloorDiv,
    Modulo,
    Power,
    
    Concat,
    
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    
    And,
    Or,

    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
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

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr {
    pub function: Box<Expr>,
    pub arguments: Vec<Expr>,
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

#[derive(Debug, Clone, PartialEq)]
pub struct MethodCallExpr {
    pub base: Box<Expr>,
    pub method: Identifier,
    pub arguments: Vec<Expr>,
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

#[derive(Debug, Clone, PartialEq)]
pub struct IfExpression {
    pub condition: Box<Expr>,
    pub then_branch: Box<Expr>,
    pub elseif_branches: Vec<ElseIfExprBranch>,
    pub else_branch: Box<Expr>,
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

#[derive(Debug, Clone, PartialEq)]
pub struct ElseIfExprBranch {
    pub condition: Expr,
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

#[derive(Debug, Clone, PartialEq)]
pub struct InterpolatedString {
    pub segments: Vec<InterpolationSegment>,
    pub span: Span,
}

impl InterpolatedString {
    pub fn new(segments: Vec<InterpolationSegment>, span: Span) -> Self {
        Self { segments, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InterpolationSegment {
    Text(String),
    Expression(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAssertion {
    pub expression: Box<Expr>,
    pub type_annotation: TypeAnnotation,
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