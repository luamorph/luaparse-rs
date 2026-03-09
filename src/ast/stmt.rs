use alloc::{boxed::Box, string::String, vec::Vec};

use crate::Span;
use super::common::{Identifier, Block, Parameter, TypeAnnotation};
use super::expr::Expr;
use crate::ast::VariableName;

#[derive(Debug, Clone, PartialEq)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

impl Stmt {
    pub fn new(kind: StmtKind, span: Span) -> Self {
        Self { kind, span }
    }
    
    pub fn synthetic(kind: StmtKind) -> Self {
        Self { kind, span: 0..0 }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StmtKind {
    LocalDeclaration(LocalDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    LocalFunctionDeclaration(LocalFunctionDeclaration),
    
    Assignment(Assignment),
    CompoundAssignment(CompoundAssignment),
    
    IfStatement(IfStatement),
    WhileLoop(WhileLoop),
    RepeatLoop(RepeatLoop),
    NumericForLoop(NumericForLoop),
    GenericForLoop(GenericForLoop),
    DoBlock(Block),
    
    ReturnStatement(ReturnStatement),
    BreakStatement,
    ContinueStatement,
    
    CallStatement(Expr),
    
    TypeDeclaration(TypeDeclaration),
    ExportStatement(Box<Stmt>),

    GotoStatement(GotoStatement),
    LabelStatement(LabelStatement),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalDeclaration {
    pub names: Vec<VariableName>,
    pub values: Option<Vec<Expr>>,
    pub is_const: bool,
    pub span: Span,
}

impl LocalDeclaration {
    pub fn new(names: Vec<VariableName>, values: Option<Vec<Expr>>, span: Span) -> Self {
        Self { names, values, is_const: false, span }
    }
    
    pub fn new_const(names: Vec<VariableName>, values: Vec<Expr>, span: Span) -> Self {
        Self { names, values: Some(values), is_const: true, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub attributes: Vec<Attribute>,
    pub name: FunctionName,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeAnnotation>,
    pub body: Block,
    pub span: Span,
}

impl FunctionDeclaration {
    pub fn new(
        attributes: Vec<Attribute>,
        name: FunctionName,
        parameters: Vec<Parameter>,
        return_type: Option<TypeAnnotation>,
        body: Block,
        span: Span,
    ) -> Self {
        Self {
            attributes,
            name,
            parameters,
            return_type,
            body,
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalFunctionDeclaration {
    pub attributes: Vec<Attribute>,
    pub name: Identifier,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeAnnotation>,
    pub body: Block,
    pub span: Span,
}

impl LocalFunctionDeclaration {
    pub fn new(
        attributes: Vec<Attribute>,
        name: Identifier,
        parameters: Vec<Parameter>,
        return_type: Option<TypeAnnotation>,
        body: Block,
        span: Span,
    ) -> Self {
        Self {
            attributes,
            name,
            parameters,
            return_type,
            body,
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionName {
    pub segments: Vec<Identifier>,
    pub method: Option<Identifier>,
}

impl FunctionName {
    pub fn new(segments: Vec<Identifier>, method: Option<Identifier>) -> Self {
        Self { segments, method }
    }
    
    pub fn simple(name: Identifier) -> Self {
        Self {
            segments: vec![name],
            method: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub targets: Vec<AssignmentTarget>,
    pub values: Vec<Expr>,
    pub span: Span,
}

impl Assignment {
    pub fn new(targets: Vec<AssignmentTarget>, values: Vec<Expr>, span: Span) -> Self {
        Self {
            targets,
            values,
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompoundAssignment {
    pub target: AssignmentTarget,
    pub operator: CompoundOperator,
    pub value: Expr,
    pub span: Span,
}

impl CompoundAssignment {
    pub fn new(
        target: AssignmentTarget,
        operator: CompoundOperator,
        value: Expr,
        span: Span,
    ) -> Self {
        Self {
            target,
            operator,
            value,
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignmentTarget {
    Identifier(Identifier),
    FieldAccess {
        base: Box<Expr>,
        field: Identifier,
        span: Span,
    },
    IndexAccess {
        base: Box<Expr>,
        index: Box<Expr>,
        span: Span,
    },
}

impl AssignmentTarget {
    pub fn span(&self) -> &Span {
        match self {
            AssignmentTarget::Identifier(id) => &id.span,
            AssignmentTarget::FieldAccess { span, .. } => span,
            AssignmentTarget::IndexAccess { span, .. } => span,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompoundOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    FloorDiv,
    Modulo,
    Power,
    Concat,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    pub condition: Expr,
    pub then_block: Block,
    pub elseif_branches: Vec<ElseIfBranch>,
    pub else_block: Option<Block>,
    pub span: Span,
}

impl IfStatement {
    pub fn new(
        condition: Expr,
        then_block: Block,
        elseif_branches: Vec<ElseIfBranch>,
        else_block: Option<Block>,
        span: Span,
    ) -> Self {
        Self {
            condition,
            then_block,
            elseif_branches,
            else_block,
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElseIfBranch {
    pub condition: Expr,
    pub then_block: Block,
}

impl ElseIfBranch {
    pub fn new(condition: Expr, then_block: Block) -> Self {
        Self {
            condition,
            then_block,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileLoop {
    pub condition: Expr,
    pub body: Block,
    pub span: Span,
}

impl WhileLoop {
    pub fn new(condition: Expr, body: Block, span: Span) -> Self {
        Self {
            condition,
            body,
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RepeatLoop {
    pub body: Block,
    pub condition: Expr,
    pub span: Span,
}

impl RepeatLoop {
    pub fn new(body: Block, condition: Expr, span: Span) -> Self {
        Self {
            body,
            condition,
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NumericForLoop {
    pub variable: Identifier,
    pub start: Expr,
    pub end: Expr,
    pub step: Option<Expr>,
    pub body: Block,
    pub span: Span,
}

impl NumericForLoop {
    pub fn new(
        variable: Identifier,
        start: Expr,
        end: Expr,
        step: Option<Expr>,
        body: Block,
        span: Span,
    ) -> Self {
        Self {
            variable,
            start,
            end,
            step,
            body,
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericForLoop {
    pub variables: Vec<Identifier>,
    pub expressions: Vec<Expr>,
    pub body: Block,
    pub span: Span,
}

impl GenericForLoop {
    pub fn new(
        variables: Vec<Identifier>,
        expressions: Vec<Expr>,
        body: Block,
        span: Span,
    ) -> Self {
        Self {
            variables,
            expressions,
            body,
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    pub values: Vec<Expr>,
    pub span: Span,
}

impl ReturnStatement {
    pub fn new(values: Vec<Expr>, span: Span) -> Self {
        Self { values, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub name: Identifier,
    pub fields: Option<Vec<AttributeField>>,
    pub span: Span,
}

impl Attribute {
    pub fn new(name: Identifier, fields: Option<Vec<AttributeField>>, span: Span) -> Self {
        Self { name, fields, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeField {
    pub key: Option<Identifier>,
    pub value: AttributeValue,
}

impl AttributeField {
    pub fn new(key: Option<Identifier>, value: AttributeValue) -> Self {
        Self { key, value }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttributeValue {
    String(String),
    Number(String),
    Boolean(bool),
    Identifier(Identifier),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDeclaration {
    pub exported: bool,
    pub name: Identifier,
    pub generics_span: Option<Span>,
    pub type_span: Span,
    pub span: Span,
}

impl TypeDeclaration {
    pub fn new(
        exported: bool,
        name: Identifier,
        generics_span: Option<Span>,
        type_span: Span,
        span: Span,
    ) -> Self {
        Self {
            exported,
            name,
            generics_span,
            type_span,
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GotoStatement {
    pub label: Identifier,
    pub span: Span,
}

impl GotoStatement {
    pub fn new(label: Identifier, span: Span) -> Self {
        Self { label, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LabelStatement {
    pub name: Identifier,
    pub span: Span,
}

impl LabelStatement {
    pub fn new(name: Identifier, span: Span) -> Self {
        Self { name, span }
    }
}
