use alloc::vec::Vec;

pub mod common;
pub mod expr;
pub mod stmt;
pub mod types;
pub mod visitor;

pub use common::*;
pub use expr::*;
pub use stmt::*;
pub use types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Ast {
    pub block: Block,
    pub comments: Vec<Comment>,
}

impl Ast {
    pub fn new(block: Block, comments: Vec<Comment>) -> Self {
        Self { block, comments }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstWithTypes {
    pub ast: Ast,
    pub type_declarations: Vec<TypeDeclarationFull>,
}

impl AstWithTypes {
    pub fn new(ast: Ast, type_declarations: Vec<TypeDeclarationFull>) -> Self {
        Self { ast, type_declarations }
    }
}