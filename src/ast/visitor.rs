//! Walk through a parsed syntax tree, visiting each node.
//!
//! This module gives you two traits for traversing an [`Ast`]:
//!
//! * [`Visitor`] for read only traversal (counting things, collecting info)
//! * [`VisitorMut`] for changing the tree in place (renaming variables, rewriting code)
//!
//! Each trait has a set of `visit_*` methods with sensible defaults that recurse
//! into child nodes automatically. Override just the ones you care about.
//!
//! If you just need something quick, [`Ast`] also has convenience methods:
//! [`for_each_stmt`](Ast::for_each_stmt), [`for_each_expr`](Ast::for_each_expr),
//! and [`for_each_identifier`](Ast::for_each_identifier) that take a closure.

use super::common::{Block, Comment, Identifier, Parameter};
use super::expr::*;
use super::stmt::*;
use super::types::*;
use super::{Ast, AstWithTypes};

/// Read only traversal of an [`Ast`].
///
/// Implement this trait and override the `visit_*` methods you care about.
/// The default implementation of each method calls the corresponding `walk_*`
/// function, which recurses into child nodes. If you override a method and
/// still want recursion, call the walk function yourself at the end.
///
/// # Example
///
/// ```rust
/// use luaparse_rs::ast::visitor::{Visitor, walk_expr};
/// use luaparse_rs::ast::{Expr, ExprKind};
///
/// struct NumberCounter(usize);
///
/// impl Visitor for NumberCounter {
///     fn visit_expr(&mut self, expr: &Expr) {
///         if matches!(expr.kind, ExprKind::Number(_)) {
///             self.0 += 1;
///         }
///         walk_expr(self, expr);
///     }
/// }
/// ```
pub trait Visitor {
    /// Visit the entire AST. Walks statements then comments.
    fn visit_ast(&mut self, ast: &Ast) {
        walk_ast(self, ast);
    }

    /// Visit an AST with type declarations (Luau).
    fn visit_ast_with_types(&mut self, ast: &AstWithTypes) {
        walk_ast_with_types(self, ast);
    }

    /// Visit a block of statements.
    fn visit_block(&mut self, block: &Block) {
        walk_block(self, block);
    }

    /// Visit a single statement.
    fn visit_stmt(&mut self, stmt: &Stmt) {
        walk_stmt(self, stmt);
    }

    /// Visit a single expression.
    fn visit_expr(&mut self, expr: &Expr) {
        walk_expr(self, expr);
    }

    /// Visit an identifier. This is a leaf node; so no default recursion.
    fn visit_identifier(&mut self, _ident: &Identifier) {}

    /// Visit an assignment target (`x`, `obj.field`, `tbl[key]`).
    fn visit_assignment_target(&mut self, target: &AssignmentTarget) {
        walk_assignment_target(self, target);
    }

    /// Visit a Luau type expression.
    fn visit_type_expr(&mut self, type_expr: &TypeExpr) {
        walk_type_expr(self, type_expr);
    }

    /// Visit a comment. This is a leaf node; so no default recursion.
    fn visit_comment(&mut self, _comment: &Comment) {}

    /// Visit a function parameter.
    fn visit_parameter(&mut self, param: &Parameter) {
        walk_parameter(self, param);
    }

    /// Visit a function name (dotted path, possibly with a method).
    fn visit_function_name(&mut self, name: &FunctionName) {
        walk_function_name(self, name);
    }

    /// Visit a table field entry.
    fn visit_table_field(&mut self, field: &TableField) {
        walk_table_field(self, field);
    }

    /// Visit a full type declaration (Luau).
    fn visit_type_declaration_full(&mut self, decl: &TypeDeclarationFull) {
        walk_type_declaration_full(self, decl);
    }
}

/// Walk all children of an [`Ast`]: the block, then the comments.
pub fn walk_ast<V: Visitor + ?Sized>(v: &mut V, ast: &Ast) {
    v.visit_block(&ast.block);
    for comment in &ast.comments {
        v.visit_comment(comment);
    }
}

/// Walk an [`AstWithTypes`]: the inner AST, then the type declarations.
pub fn walk_ast_with_types<V: Visitor + ?Sized>(v: &mut V, ast: &AstWithTypes) {
    v.visit_ast(&ast.ast);
    for decl in &ast.type_declarations {
        v.visit_type_declaration_full(decl);
    }
}

/// Walk all statements in a block.
pub fn walk_block<V: Visitor + ?Sized>(v: &mut V, block: &Block) {
    for stmt in &block.statements {
        v.visit_stmt(stmt);
    }
}

/// Walk into a statement's children based on its kind.
pub fn walk_stmt<V: Visitor + ?Sized>(v: &mut V, stmt: &Stmt) {
    match &stmt.kind {
        StmtKind::LocalDeclaration(decl) => {
            for var in &decl.names {
                v.visit_identifier(&var.name);
            }
            if let Some(values) = &decl.values {
                for val in values {
                    v.visit_expr(val);
                }
            }
        }
        StmtKind::FunctionDeclaration(decl) => {
            v.visit_function_name(&decl.name);
            for param in &decl.parameters {
                v.visit_parameter(param);
            }
            v.visit_block(&decl.body);
        }
        StmtKind::LocalFunctionDeclaration(decl) => {
            v.visit_identifier(&decl.name);
            for param in &decl.parameters {
                v.visit_parameter(param);
            }
            v.visit_block(&decl.body);
        }
        StmtKind::Assignment(assign) => {
            for target in &assign.targets {
                v.visit_assignment_target(target);
            }
            for val in &assign.values {
                v.visit_expr(val);
            }
        }
        StmtKind::CompoundAssignment(assign) => {
            v.visit_assignment_target(&assign.target);
            v.visit_expr(&assign.value);
        }
        StmtKind::IfStatement(if_stmt) => {
            v.visit_expr(&if_stmt.condition);
            v.visit_block(&if_stmt.then_block);
            for branch in &if_stmt.elseif_branches {
                v.visit_expr(&branch.condition);
                v.visit_block(&branch.then_block);
            }
            if let Some(else_block) = &if_stmt.else_block {
                v.visit_block(else_block);
            }
        }
        StmtKind::WhileLoop(while_loop) => {
            v.visit_expr(&while_loop.condition);
            v.visit_block(&while_loop.body);
        }
        StmtKind::RepeatLoop(repeat) => {
            v.visit_block(&repeat.body);
            v.visit_expr(&repeat.condition);
        }
        StmtKind::NumericForLoop(for_loop) => {
            v.visit_identifier(&for_loop.variable);
            v.visit_expr(&for_loop.start);
            v.visit_expr(&for_loop.end);
            if let Some(step) = &for_loop.step {
                v.visit_expr(step);
            }
            v.visit_block(&for_loop.body);
        }
        StmtKind::GenericForLoop(for_loop) => {
            for var in &for_loop.variables {
                v.visit_identifier(var);
            }
            for expr in &for_loop.expressions {
                v.visit_expr(expr);
            }
            v.visit_block(&for_loop.body);
        }
        StmtKind::DoBlock(block) => {
            v.visit_block(block);
        }
        StmtKind::ReturnStatement(ret) => {
            for val in &ret.values {
                v.visit_expr(val);
            }
        }
        StmtKind::BreakStatement | StmtKind::ContinueStatement => {}
        StmtKind::CallStatement(expr) => {
            v.visit_expr(expr);
        }
        StmtKind::TypeDeclaration(decl) => {
            v.visit_identifier(&decl.name);
        }
        StmtKind::ExportStatement(inner) => {
            v.visit_stmt(inner);
        }
        StmtKind::GotoStatement(goto) => {
            v.visit_identifier(&goto.label);
        }
        StmtKind::LabelStatement(label) => {
            v.visit_identifier(&label.name);
        }
    }
}

/// Walk into an expression's children based on its kind.
pub fn walk_expr<V: Visitor + ?Sized>(v: &mut V, expr: &Expr) {
    match &expr.kind {
        ExprKind::Nil | ExprKind::Boolean(_) | ExprKind::Number(_)
        | ExprKind::String(_) | ExprKind::Vararg => {}

        ExprKind::Table(table) => {
            for field in &table.fields {
                v.visit_table_field(field);
            }
        }
        ExprKind::Function(func) => {
            for param in &func.parameters {
                v.visit_parameter(param);
            }
            v.visit_block(&func.body);
        }
        ExprKind::Identifier(ident) => {
            v.visit_identifier(ident);
        }
        ExprKind::FieldAccess(access) => {
            v.visit_expr(&access.base);
            v.visit_identifier(&access.field);
        }
        ExprKind::IndexAccess(access) => {
            v.visit_expr(&access.base);
            v.visit_expr(&access.index);
        }
        ExprKind::Unary(unary) => {
            v.visit_expr(&unary.operand);
        }
        ExprKind::Binary(binary) => {
            v.visit_expr(&binary.left);
            v.visit_expr(&binary.right);
        }
        ExprKind::Call(call) => {
            v.visit_expr(&call.function);
            for arg in &call.arguments {
                v.visit_expr(arg);
            }
        }
        ExprKind::MethodCall(call) => {
            v.visit_expr(&call.base);
            v.visit_identifier(&call.method);
            for arg in &call.arguments {
                v.visit_expr(arg);
            }
        }
        ExprKind::IfExpression(if_expr) => {
            v.visit_expr(&if_expr.condition);
            v.visit_expr(&if_expr.then_branch);
            for branch in &if_expr.elseif_branches {
                v.visit_expr(&branch.condition);
                v.visit_expr(&branch.then_branch);
            }
            v.visit_expr(&if_expr.else_branch);
        }
        ExprKind::InterpolatedString(interp) => {
            for segment in &interp.segments {
                if let InterpolationSegment::Expression(expr) = segment {
                    v.visit_expr(expr);
                }
            }
        }
        ExprKind::TypeAssertion(assertion) => {
            v.visit_expr(&assertion.expression);
            // TypeAnnotation is span-only; so there's no TypeExpr to visit here.
        }
        ExprKind::TypeInstantiation(inst) => {
            v.visit_expr(&inst.expr);
        }
        ExprKind::Parenthesized(inner) => {
            v.visit_expr(inner);
        }
    }
}

/// Walk into an assignment target's children.
pub fn walk_assignment_target<V: Visitor + ?Sized>(v: &mut V, target: &AssignmentTarget) {
    match target {
        AssignmentTarget::Identifier(ident) => {
            v.visit_identifier(ident);
        }
        AssignmentTarget::FieldAccess { base, field, .. } => {
            v.visit_expr(base);
            v.visit_identifier(field);
        }
        AssignmentTarget::IndexAccess { base, index, .. } => {
            v.visit_expr(base);
            v.visit_expr(index);
        }
    }
}

/// Walk a parameter (visits its name identifier, if it has one).
pub fn walk_parameter<V: Visitor + ?Sized>(v: &mut V, param: &Parameter) {
    if let Some(name) = &param.name {
        v.visit_identifier(name);
    }
}

/// Walk a function name's segments and optional method.
pub fn walk_function_name<V: Visitor + ?Sized>(v: &mut V, name: &FunctionName) {
    for segment in &name.segments {
        v.visit_identifier(segment);
    }
    if let Some(method) = &name.method {
        v.visit_identifier(method);
    }
}

/// Walk a table field's key and value.
pub fn walk_table_field<V: Visitor + ?Sized>(v: &mut V, field: &TableField) {
    match &field.kind {
        TableFieldKind::Bracketed { key, value } => {
            v.visit_expr(key);
            v.visit_expr(value);
        }
        TableFieldKind::Named { name, value } => {
            v.visit_identifier(name);
            v.visit_expr(value);
        }
        TableFieldKind::Positional(expr) => {
            v.visit_expr(expr);
        }
    }
}

/// Walk into a type expression's children.
pub fn walk_type_expr<V: Visitor + ?Sized>(v: &mut V, type_expr: &TypeExpr) {
    match &type_expr.kind {
        TypeExprKind::Nil | TypeExprKind::Boolean(_) | TypeExprKind::String(_)
        | TypeExprKind::Number(_) => {}

        TypeExprKind::Named { path, generics } => {
            for ident in path {
                v.visit_identifier(ident);
            }
            if let Some(generics) = generics {
                for generic in generics {
                    v.visit_type_expr(generic);
                }
            }
        }
        TypeExprKind::Table(table) => {
            for prop in &table.properties {
                v.visit_identifier(&prop.name);
                v.visit_type_expr(&prop.type_expr);
            }
            if let Some(indexer) = &table.indexer {
                v.visit_type_expr(&indexer.key_type);
                v.visit_type_expr(&indexer.value_type);
            }
        }
        TypeExprKind::Function(func) => {
            for param in &func.parameters {
                if let Some(name) = &param.name {
                    v.visit_identifier(name);
                }
                v.visit_type_expr(&param.type_expr);
            }
            v.visit_type_expr(&func.return_type);
        }
        TypeExprKind::Union(types) | TypeExprKind::Intersection(types) => {
            for ty in types {
                v.visit_type_expr(ty);
            }
        }
        TypeExprKind::Optional(inner) => {
            v.visit_type_expr(inner);
        }
        TypeExprKind::Typeof(expr) => {
            v.visit_expr(expr);
        }
        TypeExprKind::GenericPack(ident) => {
            v.visit_identifier(ident);
        }
        TypeExprKind::VariadicPack(inner) => {
            v.visit_type_expr(inner);
        }
        TypeExprKind::Parenthesized(inner) => {
            v.visit_type_expr(inner);
        }
    }
}

/// Walk a full type declaration (name and type expression).
pub fn walk_type_declaration_full<V: Visitor + ?Sized>(v: &mut V, decl: &TypeDeclarationFull) {
    v.visit_identifier(&decl.name);
    v.visit_type_expr(&decl.type_expr);
}

/// Mutable traversal of an [`Ast`], for transforming the tree in place.
///
/// Same pattern as [`Visitor`], but every reference is `&mut`. Use this when
/// you need to rename variables, rewrite expressions, or make any changes to
/// the tree.
///
/// # Example
///
/// ```rust
/// use luaparse_rs::ast::visitor::{VisitorMut, walk_expr_mut};
/// use luaparse_rs::ast::{Expr, ExprKind, Identifier};
///
/// struct Renamer;
///
/// impl VisitorMut for Renamer {
///     fn visit_identifier(&mut self, ident: &mut Identifier) {
///         if ident.name == "old_name" {
///             ident.name = "new_name".into();
///         }
///     }
/// }
/// ```
pub trait VisitorMut {
    /// Visit the entire AST (mutable).
    fn visit_ast(&mut self, ast: &mut Ast) {
        walk_ast_mut(self, ast);
    }

    /// Visit an AST with type declarations (mutable).
    fn visit_ast_with_types(&mut self, ast: &mut AstWithTypes) {
        walk_ast_with_types_mut(self, ast);
    }

    /// Visit a block of statements (mutable).
    fn visit_block(&mut self, block: &mut Block) {
        walk_block_mut(self, block);
    }

    /// Visit a single statement (mutable).
    fn visit_stmt(&mut self, stmt: &mut Stmt) {
        walk_stmt_mut(self, stmt);
    }

    /// Visit a single expression (mutable).
    fn visit_expr(&mut self, expr: &mut Expr) {
        walk_expr_mut(self, expr);
    }

    /// Visit an identifier (mutable). Leaf node; so no default recursion.
    fn visit_identifier(&mut self, _ident: &mut Identifier) {}

    /// Visit an assignment target (mutable).
    fn visit_assignment_target(&mut self, target: &mut AssignmentTarget) {
        walk_assignment_target_mut(self, target);
    }

    /// Visit a Luau type expression (mutable).
    fn visit_type_expr(&mut self, type_expr: &mut TypeExpr) {
        walk_type_expr_mut(self, type_expr);
    }

    /// Visit a comment (mutable). Leaf node; so no default recursion.
    fn visit_comment(&mut self, _comment: &mut Comment) {}

    /// Visit a function parameter (mutable).
    fn visit_parameter(&mut self, param: &mut Parameter) {
        walk_parameter_mut(self, param);
    }

    /// Visit a function name (mutable).
    fn visit_function_name(&mut self, name: &mut FunctionName) {
        walk_function_name_mut(self, name);
    }

    /// Visit a table field entry (mutable).
    fn visit_table_field(&mut self, field: &mut TableField) {
        walk_table_field_mut(self, field);
    }

    /// Visit a full type declaration (mutable).
    fn visit_type_declaration_full(&mut self, decl: &mut TypeDeclarationFull) {
        walk_type_declaration_full_mut(self, decl);
    }
}

/// Mutable version of [`walk_ast`].
pub fn walk_ast_mut<V: VisitorMut + ?Sized>(v: &mut V, ast: &mut Ast) {
    v.visit_block(&mut ast.block);
    for comment in &mut ast.comments {
        v.visit_comment(comment);
    }
}

/// Mutable version of [`walk_ast_with_types`].
pub fn walk_ast_with_types_mut<V: VisitorMut + ?Sized>(v: &mut V, ast: &mut AstWithTypes) {
    v.visit_ast(&mut ast.ast);
    for decl in &mut ast.type_declarations {
        v.visit_type_declaration_full(decl);
    }
}

/// Mutable version of [`walk_block`].
pub fn walk_block_mut<V: VisitorMut + ?Sized>(v: &mut V, block: &mut Block) {
    for stmt in &mut block.statements {
        v.visit_stmt(stmt);
    }
}

/// Mutable version of [`walk_stmt`].
pub fn walk_stmt_mut<V: VisitorMut + ?Sized>(v: &mut V, stmt: &mut Stmt) {
    match &mut stmt.kind {
        StmtKind::LocalDeclaration(decl) => {
            for var in &mut decl.names {
                v.visit_identifier(&mut var.name);
            }
            if let Some(values) = &mut decl.values {
                for val in values {
                    v.visit_expr(val);
                }
            }
        }
        StmtKind::FunctionDeclaration(decl) => {
            v.visit_function_name(&mut decl.name);
            for param in &mut decl.parameters {
                v.visit_parameter(param);
            }
            v.visit_block(&mut decl.body);
        }
        StmtKind::LocalFunctionDeclaration(decl) => {
            v.visit_identifier(&mut decl.name);
            for param in &mut decl.parameters {
                v.visit_parameter(param);
            }
            v.visit_block(&mut decl.body);
        }
        StmtKind::Assignment(assign) => {
            for target in &mut assign.targets {
                v.visit_assignment_target(target);
            }
            for val in &mut assign.values {
                v.visit_expr(val);
            }
        }
        StmtKind::CompoundAssignment(assign) => {
            v.visit_assignment_target(&mut assign.target);
            v.visit_expr(&mut assign.value);
        }
        StmtKind::IfStatement(if_stmt) => {
            v.visit_expr(&mut if_stmt.condition);
            v.visit_block(&mut if_stmt.then_block);
            for branch in &mut if_stmt.elseif_branches {
                v.visit_expr(&mut branch.condition);
                v.visit_block(&mut branch.then_block);
            }
            if let Some(else_block) = &mut if_stmt.else_block {
                v.visit_block(else_block);
            }
        }
        StmtKind::WhileLoop(while_loop) => {
            v.visit_expr(&mut while_loop.condition);
            v.visit_block(&mut while_loop.body);
        }
        StmtKind::RepeatLoop(repeat) => {
            v.visit_block(&mut repeat.body);
            v.visit_expr(&mut repeat.condition);
        }
        StmtKind::NumericForLoop(for_loop) => {
            v.visit_identifier(&mut for_loop.variable);
            v.visit_expr(&mut for_loop.start);
            v.visit_expr(&mut for_loop.end);
            if let Some(step) = &mut for_loop.step {
                v.visit_expr(step);
            }
            v.visit_block(&mut for_loop.body);
        }
        StmtKind::GenericForLoop(for_loop) => {
            for var in &mut for_loop.variables {
                v.visit_identifier(var);
            }
            for expr in &mut for_loop.expressions {
                v.visit_expr(expr);
            }
            v.visit_block(&mut for_loop.body);
        }
        StmtKind::DoBlock(block) => {
            v.visit_block(block);
        }
        StmtKind::ReturnStatement(ret) => {
            for val in &mut ret.values {
                v.visit_expr(val);
            }
        }
        StmtKind::BreakStatement | StmtKind::ContinueStatement => {}
        StmtKind::CallStatement(expr) => {
            v.visit_expr(expr);
        }
        StmtKind::TypeDeclaration(decl) => {
            v.visit_identifier(&mut decl.name);
        }
        StmtKind::ExportStatement(inner) => {
            v.visit_stmt(inner);
        }
        StmtKind::GotoStatement(goto) => {
            v.visit_identifier(&mut goto.label);
        }
        StmtKind::LabelStatement(label) => {
            v.visit_identifier(&mut label.name);
        }
    }
}

/// Mutable version of [`walk_expr`].
pub fn walk_expr_mut<V: VisitorMut + ?Sized>(v: &mut V, expr: &mut Expr) {
    match &mut expr.kind {
        ExprKind::Nil | ExprKind::Boolean(_) | ExprKind::Number(_)
        | ExprKind::String(_) | ExprKind::Vararg => {}

        ExprKind::Table(table) => {
            for field in &mut table.fields {
                v.visit_table_field(field);
            }
        }
        ExprKind::Function(func) => {
            for param in &mut func.parameters {
                v.visit_parameter(param);
            }
            v.visit_block(&mut func.body);
        }
        ExprKind::Identifier(ident) => {
            v.visit_identifier(ident);
        }
        ExprKind::FieldAccess(access) => {
            v.visit_expr(&mut access.base);
            v.visit_identifier(&mut access.field);
        }
        ExprKind::IndexAccess(access) => {
            v.visit_expr(&mut access.base);
            v.visit_expr(&mut access.index);
        }
        ExprKind::Unary(unary) => {
            v.visit_expr(&mut unary.operand);
        }
        ExprKind::Binary(binary) => {
            v.visit_expr(&mut binary.left);
            v.visit_expr(&mut binary.right);
        }
        ExprKind::Call(call) => {
            v.visit_expr(&mut call.function);
            for arg in &mut call.arguments {
                v.visit_expr(arg);
            }
        }
        ExprKind::MethodCall(call) => {
            v.visit_expr(&mut call.base);
            v.visit_identifier(&mut call.method);
            for arg in &mut call.arguments {
                v.visit_expr(arg);
            }
        }
        ExprKind::IfExpression(if_expr) => {
            v.visit_expr(&mut if_expr.condition);
            v.visit_expr(&mut if_expr.then_branch);
            for branch in &mut if_expr.elseif_branches {
                v.visit_expr(&mut branch.condition);
                v.visit_expr(&mut branch.then_branch);
            }
            v.visit_expr(&mut if_expr.else_branch);
        }
        ExprKind::InterpolatedString(interp) => {
            for segment in &mut interp.segments {
                if let InterpolationSegment::Expression(expr) = segment {
                    v.visit_expr(expr);
                }
            }
        }
        ExprKind::TypeAssertion(assertion) => {
            v.visit_expr(&mut assertion.expression);
        }
        ExprKind::TypeInstantiation(inst) => {
            v.visit_expr(&mut inst.expr);
        }
        ExprKind::Parenthesized(inner) => {
            v.visit_expr(inner);
        }
    }
}

/// Mutable version of [`walk_assignment_target`].
pub fn walk_assignment_target_mut<V: VisitorMut + ?Sized>(
    v: &mut V,
    target: &mut AssignmentTarget,
) {
    match target {
        AssignmentTarget::Identifier(ident) => {
            v.visit_identifier(ident);
        }
        AssignmentTarget::FieldAccess { base, field, .. } => {
            v.visit_expr(base);
            v.visit_identifier(field);
        }
        AssignmentTarget::IndexAccess { base, index, .. } => {
            v.visit_expr(base);
            v.visit_expr(index);
        }
    }
}

/// Mutable version of [`walk_parameter`].
pub fn walk_parameter_mut<V: VisitorMut + ?Sized>(v: &mut V, param: &mut Parameter) {
    if let Some(name) = &mut param.name {
        v.visit_identifier(name);
    }
}

/// Mutable version of [`walk_function_name`].
pub fn walk_function_name_mut<V: VisitorMut + ?Sized>(v: &mut V, name: &mut FunctionName) {
    for segment in &mut name.segments {
        v.visit_identifier(segment);
    }
    if let Some(method) = &mut name.method {
        v.visit_identifier(method);
    }
}

/// Mutable version of [`walk_table_field`].
pub fn walk_table_field_mut<V: VisitorMut + ?Sized>(v: &mut V, field: &mut TableField) {
    match &mut field.kind {
        TableFieldKind::Bracketed { key, value } => {
            v.visit_expr(key);
            v.visit_expr(value);
        }
        TableFieldKind::Named { name, value } => {
            v.visit_identifier(name);
            v.visit_expr(value);
        }
        TableFieldKind::Positional(expr) => {
            v.visit_expr(expr);
        }
    }
}

/// Mutable version of [`walk_type_expr`].
pub fn walk_type_expr_mut<V: VisitorMut + ?Sized>(v: &mut V, type_expr: &mut TypeExpr) {
    match &mut type_expr.kind {
        TypeExprKind::Nil | TypeExprKind::Boolean(_) | TypeExprKind::String(_)
        | TypeExprKind::Number(_) => {}

        TypeExprKind::Named { path, generics } => {
            for ident in path {
                v.visit_identifier(ident);
            }
            if let Some(generics) = generics {
                for generic in generics {
                    v.visit_type_expr(generic);
                }
            }
        }
        TypeExprKind::Table(table) => {
            for prop in &mut table.properties {
                v.visit_identifier(&mut prop.name);
                v.visit_type_expr(&mut prop.type_expr);
            }
            if let Some(indexer) = &mut table.indexer {
                v.visit_type_expr(&mut indexer.key_type);
                v.visit_type_expr(&mut indexer.value_type);
            }
        }
        TypeExprKind::Function(func) => {
            for param in &mut func.parameters {
                if let Some(name) = &mut param.name {
                    v.visit_identifier(name);
                }
                v.visit_type_expr(&mut param.type_expr);
            }
            v.visit_type_expr(&mut func.return_type);
        }
        TypeExprKind::Union(types) | TypeExprKind::Intersection(types) => {
            for ty in types {
                v.visit_type_expr(ty);
            }
        }
        TypeExprKind::Optional(inner) => {
            v.visit_type_expr(inner);
        }
        TypeExprKind::Typeof(expr) => {
            v.visit_expr(expr);
        }
        TypeExprKind::GenericPack(ident) => {
            v.visit_identifier(ident);
        }
        TypeExprKind::VariadicPack(inner) => {
            v.visit_type_expr(inner);
        }
        TypeExprKind::Parenthesized(inner) => {
            v.visit_type_expr(inner);
        }
    }
}

/// Mutable version of [`walk_type_declaration_full`].
pub fn walk_type_declaration_full_mut<V: VisitorMut + ?Sized>(
    v: &mut V,
    decl: &mut TypeDeclarationFull,
) {
    v.visit_identifier(&mut decl.name);
    v.visit_type_expr(&mut decl.type_expr);
}

impl Ast {
    /// Calls `f` on every statement in the tree, recursing into nested blocks.
    ///
    /// This is a shortcut for implementing [`Visitor`] when all you need is
    /// to look at statements.
    pub fn for_each_stmt(&self, f: impl FnMut(&Stmt)) {
        struct StmtWalker<F>(F);
        impl<F: FnMut(&Stmt)> Visitor for StmtWalker<F> {
            fn visit_stmt(&mut self, stmt: &Stmt) {
                (self.0)(stmt);
                walk_stmt(self, stmt);
            }
        }
        let mut walker = StmtWalker(f);
        walker.visit_ast(self);
    }

    /// Calls `f` on every expression in the tree, recursing into sub expressions.
    ///
    /// This is a shortcut for implementing [`Visitor`] when all you need is
    /// to look at expressions.
    pub fn for_each_expr(&self, f: impl FnMut(&Expr)) {
        struct ExprWalker<F>(F);
        impl<F: FnMut(&Expr)> Visitor for ExprWalker<F> {
            fn visit_expr(&mut self, expr: &Expr) {
                (self.0)(expr);
                walk_expr(self, expr);
            }
        }
        let mut walker = ExprWalker(f);
        walker.visit_ast(self);
    }

    /// Calls `f` on every identifier in the tree.
    ///
    /// Useful for collecting all names used in a program, building symbol
    /// tables, or doing simple renaming.
    pub fn for_each_identifier(&self, f: impl FnMut(&Identifier)) {
        struct IdentWalker<F>(F);
        impl<F: FnMut(&Identifier)> Visitor for IdentWalker<F> {
            fn visit_identifier(&mut self, ident: &Identifier) {
                (self.0)(ident);
            }
        }
        let mut walker = IdentWalker(f);
        walker.visit_ast(self);
    }
}
