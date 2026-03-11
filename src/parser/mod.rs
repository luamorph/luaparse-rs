use alloc::vec::Vec;

use crate::{
    ast::*, lexer::Token, marker::LuaVersion,
    LexError,
    ParseError,
    Span,
};
use std::marker::PhantomData;

pub mod helpers;
pub mod types;
pub mod stmt;
pub mod expr;

/// The main parser. Feed it Lua source code and get back an [`Ast`].
///
/// The type parameter `V` selects which Lua version's grammar to use.
/// Pick one of [`Luau`](crate::Luau), [`Lua51`](crate::Lua51),
/// [`Lua52`](crate::Lua52), [`Lua53`](crate::Lua53), or [`Lua54`](crate::Lua54).
///
/// # Example
///
/// ```rust
/// use luaparse_rs::{Parser, Luau};
///
/// let parser = Parser::<Luau>::new("local x = 1").unwrap();
/// let ast = parser.parse().unwrap();
/// ```
pub struct Parser<'src, V: LuaVersion> {
    tokens: Vec<(Token, Span)>,
    position: usize,
    source: &'src str,
    comments: Vec<Comment>,
    _version: PhantomData<V>,
}

impl<'src, V: LuaVersion> Parser<'src, V> {
    /// Creates a new parser from a source string.
    ///
    /// This tokenizes the input immediately. If the source contains invalid
    /// tokens (like an unterminated string), you'll get a [`LexError`] here.
    pub fn new(source: &'src str) -> Result<Self, LexError> {
        let tokens = crate::lexer::lex_for_version::<V>(source)?;
        Ok(Self {
            tokens,
            position: 0,
            source,
            comments: Vec::new(),
            _version: PhantomData,
        })
    }
    
    /// Parses the source and returns the full syntax tree.
    ///
    /// This consumes the parser. If you need type declarations (Luau),
    /// use [`parse_with_types`](Self::parse_with_types) instead.
    pub fn parse(mut self) -> Result<Ast, ParseError> {
        let start = 0;
        let mut statements = Vec::new();
        
        while !self.is_eof() {
            if let Token::Comment(content) = self.current() {
                let span = self.current_span();
                let is_block = content.contains('\n');
                self.comments.push(Comment::new(content.clone(), is_block, span));
                self.advance();
                continue;
            }
            
            while matches!(self.current(), Token::Semi) {
                self.advance();
                self.skip_comments();
            }
            
            if self.is_eof() {
                break;
            }
            
            statements.push(self.parse_statement()?);
            
            // return/break/continue must be the last statement in a block
            if matches!(
                statements.last().map(|s| &s.kind),
                Some(StmtKind::ReturnStatement(_))
                    | Some(StmtKind::BreakStatement)
                    | Some(StmtKind::ContinueStatement)
            ) {
                while matches!(self.current(), Token::Semi) {
                    self.advance();
                }
                break;
            }
            
            if matches!(self.current(), Token::Semi) {
                self.advance();
            }
        }
        
        // skip trailing comments
        self.skip_comments();
        
        // reject any trailing tokens after the block
        if !self.is_eof() {
            return Err(ParseError::InvalidSyntax {
                message: "unexpected statement after return".to_string(),
                span: self.current_span(),
                help: Some("return must be the last statement in a block".to_string()),
            });
        }
        
        let end = self.source.len();
        let block = Block::new(statements, start..end);
        
        Ok(Ast::new(block, self.comments))
    }
    
    /// Parses the source and returns the syntax tree along with type declarations.
    ///
    /// This is the Luau variant of [`parse`](Self::parse). It pulls `type` and
    /// `export type` declarations into a separate list so you can work with them
    /// independently from the rest of the code.
    pub fn parse_with_types(mut self) -> Result<AstWithTypes, ParseError> {
        let mut type_declarations = Vec::new();
        let start = 0;
        let mut statements = Vec::new();
        
        while !self.is_eof() {
            if let Token::Comment(content) = self.current() {
                let span = self.current_span();
                let is_block = content.contains('\n');
                self.comments.push(Comment::new(content.clone(), is_block, span));
                self.advance();
                continue;
            }
            
            // handle export type
            if matches!(self.current(), Token::Export) && V::HAS_EXPORT {
                let checkpoint = self.checkpoint();
                self.advance();
                
                if matches!(self.current(), Token::Type) && V::HAS_TYPE_ANNOTATIONS {
                    self.advance();
                    let mut type_decl = self.parse_type_declaration_full()?;
                    type_decl.exported = true;
                    type_declarations.push(type_decl);
                    continue;
                } else {
                    self.restore(checkpoint);
                }
            }
            
            // check for type dec.
            if matches!(self.current(), Token::Type) && V::HAS_TYPE_ANNOTATIONS {
                self.advance();
                let type_decl = self.parse_type_declaration_full()?;
                type_declarations.push(type_decl);
                continue;
            }
            
            statements.push(self.parse_statement()?);
            
            // return/break/continue must be the last statement in a block
            if matches!(
                statements.last().map(|s| &s.kind),
                Some(StmtKind::ReturnStatement(_))
                    | Some(StmtKind::BreakStatement)
                    | Some(StmtKind::ContinueStatement)
            ) {
                while matches!(self.current(), Token::Semi) {
                    self.advance();
                }
                break;
            }
        }
        
        // skip trailing comments
        self.skip_comments();
        
        // reject any trailing tokens after the block
        if !self.is_eof() {
            return Err(ParseError::InvalidSyntax {
                message: "unexpected statement after return".to_string(),
                span: self.current_span(),
                help: Some("return must be the last statement in a block".to_string()),
            });
        }
        
        let end = self.source.len();
        let block = Block::new(statements, start..end);
        let ast = Ast::new(block, self.comments);
        
        Ok(AstWithTypes::new(ast, type_declarations))
    }
    
    fn current(&self) -> &Token {
        self.tokens
            .get(self.position)
            .map(|(t, _)| t)
            .unwrap_or(&Token::Eof)
    }
    
    fn current_span(&self) -> Span {
        self.tokens
            .get(self.position)
            .map(|(_, s)| s.clone())
            .unwrap_or_else(|| {
                let pos = self.source.len();
                pos..pos
            })
    }
    
    fn peek(&self, n: usize) -> &Token {
        self.tokens
            .get(self.position + n)
            .map(|(t, _)| t)
            .unwrap_or(&Token::Eof)
    }
    
    fn advance(&mut self) -> (Token, Span) {
        let result = self.tokens
            .get(self.position)
            .cloned()
            .unwrap_or_else(|| {
                let pos = self.source.len();
                (Token::Eof, pos..pos)
            });
        self.position += 1;
        result
    }
    
    pub(super) fn split_greater_eq(&mut self) {
        if matches!(self.current(), Token::GreaterEq) {
            let span = self.current_span();
            self.tokens[self.position] = (Token::Greater, span.clone());
            self.tokens.insert(self.position + 1, (Token::Eq, span));
        }
    }
    
    pub(super) fn split_right_shift(&mut self) {
        if matches!(self.current(), Token::RightShift) {
            let span = self.current_span();
            let mid = span.start + 1;
            self.tokens[self.position] = (Token::Greater, span.start..mid);
            self.tokens.insert(self.position + 1, (Token::Greater, mid..span.end));
        }
    }

    fn is_eof(&self) -> bool {
        matches!(self.current(), Token::Eof)
    }
    
    fn expect(&mut self, expected: Token) -> Result<Span, ParseError> {
        if self.current() == &expected {
            let (_, span) = self.advance();
            Ok(span)
        } else {
            Err(ParseError::UnexpectedToken {
                expected: vec![format!("{:?}", expected)],
                found: format!("{:?}", self.current()),
                span: self.current_span(),
            })
        }
    }
    
    fn consume(&mut self, token: Token) -> bool {
        if self.current() == &token {
            self.advance();
            true
        } else {
            false
        }
    }
    
    fn checkpoint(&self) -> usize {
        self.position
    }
    
    fn restore(&mut self, checkpoint: usize) {
        self.position = checkpoint;
    }
    
    fn try_parse<T, F>(&mut self, f: F) -> Option<T>
    where
        F: FnOnce(&mut Self) -> Result<T, ParseError>,
    {
        let checkpoint = self.checkpoint();
        match f(self) {
            Ok(value) => Some(value),
            Err(_) => {
                self.restore(checkpoint);
                None
            }
        }
    }
    
    fn skip_comments(&mut self) {
        while matches!(self.current(), Token::Comment(_)) {
            let (token, span) = self.advance();
            if let Token::Comment(content) = token {
                let is_block = content.contains('\n');
                self.comments.push(Comment::new(content, is_block, span));
            }
        }
    }
    
    fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        self.skip_comments();
        stmt::parse_statement(self)
    }
    
    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.skip_comments();
        expr::parse_expression(self)
    }
}
