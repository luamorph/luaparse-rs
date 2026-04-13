//! Statement nodes in the syntax tree.

use alloc::{boxed::Box, string::String, vec::Vec};

use crate::Span;
use super::common::{Identifier, Block, Parameter, TypeAnnotation};
use super::expr::Expr;
use crate::ast::VariableName;

/// A single parsed statement.
///
/// Every statement has a [`kind`](Self::kind) that tells you what it is,
/// and a [`span`](Self::span) pointing back into the source.
#[derive(Debug, Clone, PartialEq)]
pub struct Stmt {
    /// What kind of statement this is.
    pub kind: StmtKind,
    /// Where it appears in the source.
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

/// All the different kinds of statement the parser can produce.
#[derive(Debug, Clone, PartialEq)]
pub enum StmtKind {
    /// `local x, y = 1, 2` or `local const z = 3` (Luau).
    LocalDeclaration(LocalDeclaration),
    /// `function foo.bar:baz() end`.
    FunctionDeclaration(FunctionDeclaration),
    /// `local function foo() end`.
    LocalFunctionDeclaration(LocalFunctionDeclaration),

    /// `x, y = 1, 2`.
    Assignment(Assignment),
    /// `x += 1` (Luau and some versions).
    CompoundAssignment(CompoundAssignment),

    /// `if cond then ... elseif ... else ... end`.
    IfStatement(IfStatement),
    /// `while cond do ... end`.
    WhileLoop(WhileLoop),
    /// `repeat ... until cond`.
    RepeatLoop(RepeatLoop),
    /// `for i = 1, 10, 2 do ... end`.
    NumericForLoop(NumericForLoop),
    /// `for k, v in pairs(t) do ... end`.
    GenericForLoop(GenericForLoop),
    /// `do ... end`.
    DoBlock(Block),

    /// `return expr, expr`.
    ReturnStatement(ReturnStatement),
    /// `break`.
    BreakStatement,
    /// `continue` (Luau only).
    ContinueStatement,

    /// A bare function or method call used as a statement (e.g. `print("hi")`).
    CallStatement(Expr),

    /// `type Foo = ...` (Luau).
    TypeDeclaration(TypeDeclaration),
    /// `export type Foo = ...` (Luau). Wraps the inner statement.
    ExportStatement(Box<Stmt>),

    /// `goto label` (Lua 5.2+).
    GotoStatement(GotoStatement),
    /// `::label::` (Lua 5.2+).
    LabelStatement(LabelStatement),
}

/// A `local` variable declaration.
///
/// Covers `local x = 1`, `local x, y`, and Luau's `local const z = 3`.
#[derive(Debug, Clone, PartialEq)]
pub struct LocalDeclaration {
    /// The names being declared, each possibly with an attribute.
    pub names: Vec<VariableName>,
    /// The values being assigned, if any.
    pub values: Option<Vec<Expr>>,
    /// `true` when declared with `const` (Luau immutable binding).
    pub is_const: bool,
    /// Where it appears in the source.
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

/// A named function declaration like `function foo() end`.
///
/// Includes dotted names (`function a.b.c() end`) and method syntax
/// (`function a:b() end`).
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    /// Attributes like `@native` (Luau).
    pub attributes: Vec<Attribute>,
    /// The function's name, possibly dotted or with a method component.
    pub name: FunctionName,
    /// The parameter list.
    pub parameters: Vec<Parameter>,
    /// An optional return type annotation (Luau).
    pub return_type: Option<TypeAnnotation>,
    /// The function body.
    pub body: Block,
    /// Where it appears in the source.
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

/// A `local function foo() end` declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct LocalFunctionDeclaration {
    /// Attributes like `@native` (Luau).
    pub attributes: Vec<Attribute>,
    /// The function name (always a single identifier for local functions).
    pub name: Identifier,
    /// The parameter list.
    pub parameters: Vec<Parameter>,
    /// An optional return type annotation (Luau).
    pub return_type: Option<TypeAnnotation>,
    /// The function body.
    pub body: Block,
    /// Where it appears in the source.
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

/// The name part of a function declaration.
///
/// A name like `a.b.c:d` has segments `[a, b, c]` and method `d`.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionName {
    /// The dotted path segments (at least one).
    pub segments: Vec<Identifier>,
    /// The method name after `:`, if any.
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

/// An assignment like `x, y = 1, 2`.
#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    /// What's being assigned to.
    pub targets: Vec<AssignmentTarget>,
    /// The values on the right side.
    pub values: Vec<Expr>,
    /// Where it appears in the source.
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

/// A compound assignment like `x += 1` or `s ..= "!"` (Luau).
#[derive(Debug, Clone, PartialEq)]
pub struct CompoundAssignment {
    /// What's being assigned to.
    pub target: AssignmentTarget,
    /// Which compound operator (`+=`, `-=`, etc.).
    pub operator: CompoundOperator,
    /// The right side value.
    pub value: Expr,
    /// Where it appears in the source.
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

/// Something that can appear on the left side of `=`.
#[derive(Debug, Clone, PartialEq)]
pub enum AssignmentTarget {
    /// A plain variable name, like `x`.
    Identifier(Identifier),
    /// A dot field, like `obj.field`.
    FieldAccess {
        base: Box<Expr>,
        field: Identifier,
        span: Span,
    },
    /// A bracket index, like `tbl[key]`.
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

/// The operator in a compound assignment (`+=`, `-=`, `..=`, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompoundOperator {
    /// `+=`
    Add,
    /// `-=`
    Subtract,
    /// `*=`
    Multiply,
    /// `/=`
    Divide,
    /// `//=`
    FloorDiv,
    /// `%=`
    Modulo,
    /// `^=`
    Power,
    /// `..=`
    Concat,
}

/// An `if/elseif/else/end` statement.
#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    /// The condition after `if`.
    pub condition: Expr,
    /// The `then` block.
    pub then_block: Block,
    /// Zero or more `elseif` branches.
    pub elseif_branches: Vec<ElseIfBranch>,
    /// The `else` block, if present.
    pub else_block: Option<Block>,
    /// Where it appears in the source.
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

/// One `elseif cond then ...` branch inside an [`IfStatement`].
#[derive(Debug, Clone, PartialEq)]
pub struct ElseIfBranch {
    /// The condition after `elseif`.
    pub condition: Expr,
    /// The block to run if the condition is true.
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

/// A `while cond do ... end` loop.
#[derive(Debug, Clone, PartialEq)]
pub struct WhileLoop {
    /// The loop condition.
    pub condition: Expr,
    /// The loop body.
    pub body: Block,
    /// Where it appears in the source.
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

/// A `repeat ... until cond` loop.
#[derive(Debug, Clone, PartialEq)]
pub struct RepeatLoop {
    /// The loop body (runs at least once).
    pub body: Block,
    /// The condition checked after each iteration.
    pub condition: Expr,
    /// Where it appears in the source.
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

/// A numeric `for` loop: `for i = start, end, step do ... end`.
#[derive(Debug, Clone, PartialEq)]
pub struct NumericForLoop {
    /// The loop variable.
    pub variable: Identifier,
    /// The starting value.
    pub start: Expr,
    /// The ending value.
    pub end: Expr,
    /// The step value (defaults to 1 if omitted).
    pub step: Option<Expr>,
    /// The loop body.
    pub body: Block,
    /// Where it appears in the source.
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

/// A generic `for` loop: `for k, v in pairs(t) do ... end`.
#[derive(Debug, Clone, PartialEq)]
pub struct GenericForLoop {
    /// The loop variables (e.g. `k, v`).
    pub variables: Vec<Identifier>,
    /// The iterator expressions (e.g. `pairs(t)`).
    pub expressions: Vec<Expr>,
    /// The loop body.
    pub body: Block,
    /// Where it appears in the source.
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

/// A `return` statement with zero or more values.
#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    /// The returned values (can be empty for a bare `return`).
    pub values: Vec<Expr>,
    /// Where it appears in the source.
    pub span: Span,
}

impl ReturnStatement {
    pub fn new(values: Vec<Expr>, span: Span) -> Self {
        Self { values, span }
    }
}

/// An attribute like `@native` or `@checked` (Luau).
#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    /// The attribute name (e.g. `native`).
    pub name: Identifier,
    /// Optional fields inside the attribute.
    pub fields: Option<Vec<AttributeField>>,
    /// Where it appears in the source.
    pub span: Span,
}

impl Attribute {
    pub fn new(name: Identifier, fields: Option<Vec<AttributeField>>, span: Span) -> Self {
        Self { name, fields, span }
    }
}

/// A single field inside an [`Attribute`].
#[derive(Debug, Clone, PartialEq)]
pub struct AttributeField {
    /// The field name, if it's a key/value pair.
    pub key: Option<Identifier>,
    /// The field value.
    pub value: AttributeValue,
}

impl AttributeField {
    pub fn new(key: Option<Identifier>, value: AttributeValue) -> Self {
        Self { key, value }
    }
}

/// A value inside an [`AttributeField`].
#[derive(Debug, Clone, PartialEq)]
pub enum AttributeValue {
    /// A string literal.
    String(String),
    /// A number literal (stored as the raw text).
    Number(String),
    /// `true` or `false`.
    Boolean(bool),
    /// A plain identifier.
    Identifier(Identifier),
}

/// A `type Foo = ...` declaration (Luau).
///
/// This is the lightweight version stored in the statement list.
/// For the fully resolved type expression, see [`TypeDeclarationFull`](super::types::TypeDeclarationFull).
#[derive(Debug, Clone, PartialEq)]
pub struct TypeDeclaration {
    /// Whether this was declared with `export`.
    pub exported: bool,
    /// The name of the type alias.
    pub name: Identifier,
    /// The span of the generic parameters, if any.
    pub generics_span: Option<Span>,
    /// The span of the type expression on the right side.
    pub type_span: Span,
    /// Where the whole declaration appears in the source.
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

/// A `goto label` statement (Lua 5.2+).
#[derive(Debug, Clone, PartialEq)]
pub struct GotoStatement {
    /// The label to jump to.
    pub label: Identifier,
    /// Where it appears in the source.
    pub span: Span,
}

impl GotoStatement {
    pub fn new(label: Identifier, span: Span) -> Self {
        Self { label, span }
    }
}

/// A `::label::` label definition (Lua 5.2+).
#[derive(Debug, Clone, PartialEq)]
pub struct LabelStatement {
    /// The label name.
    pub name: Identifier,
    /// Where it appears in the source.
    pub span: Span,
}

impl LabelStatement {
    pub fn new(name: Identifier, span: Span) -> Self {
        Self { name, span }
    }
}
