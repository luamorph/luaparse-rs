use alloc::vec::Vec;

use super::Parser;
use crate::{
    ast::*,
    lexer::Token,
    marker::LuaVersion,
    ParseError,
};

impl<'src, V: LuaVersion> Parser<'src, V> {
    pub(super) fn parse_identifier(&mut self) -> Result<Identifier, ParseError> {
        let name = match self.current() {
            Token::Identifier(name) => name.clone(),
            Token::Continue if !V::HAS_CONTINUE => "continue".to_string(),
            Token::Export if !V::HAS_EXPORT => "export".to_string(),
            Token::Type if !V::HAS_TYPE_ANNOTATIONS => "type".to_string(),
            Token::Goto if !V::HAS_GOTO => "goto".to_string(),
            // . . .
            _ => {
                return Err(ParseError::UnexpectedToken {
                    expected: alloc::vec!["identifier".to_string()],
                    found: alloc::format!("{:?}", self.current()),
                    span: self.current_span(),
                });
            }
        };
        
        let span = self.current_span();
        self.advance();
        Ok(Identifier::new(name, span))
    }

    pub(super) fn can_be_identifier(&self, token: &Token) -> bool {
        match token {
            Token::Identifier(_) => true,
            Token::Continue => !V::HAS_CONTINUE,
            Token::Export => !V::HAS_EXPORT,
            Token::Type => !V::HAS_TYPE_ANNOTATIONS,
            Token::Goto => !V::HAS_GOTO,
            Token::Const => !V::HAS_CONST,
            _ => false,
        }
    }
    
    pub(super) fn parse_block_until(&mut self, terminators: &[Token]) -> Result<Block, ParseError> {
        let start = self.current_span().start;
        let mut statements = Vec::new();
        
        while !self.is_eof() {
            self.skip_comments();
            
            if terminators.contains(self.current()) {
                break;
            }
            
            while matches!(self.current(), Token::Semi) {
                self.advance();
                self.skip_comments();
            }
            
            if terminators.contains(self.current()) || self.is_eof() {
                break;
            }
            
            if matches!(
                self.current(),
                Token::Return | Token::Break | Token::Continue
            ) {
                statements.push(self.parse_statement()?);
                while matches!(self.current(), Token::Semi) {
                    self.advance();
                }
                
                break;
            }
            
            statements.push(self.parse_statement()?);
            
            if matches!(self.current(), Token::Semi) {
                self.advance();
            }
        }
        
        let end = self.current_span().end;
        Ok(Block::new(statements, start..end))
    }
    
    pub(super) fn parse_parameters(&mut self) -> Result<Vec<Parameter>, ParseError> {
        let mut parameters = Vec::new();
        let mut seen_vararg = false;
        
        while !matches!(self.current(), Token::RParen | Token::Eof) {
            self.skip_comments();
            
            if seen_vararg {
                return Err(ParseError::InvalidSyntax {
                    message: "varargs (...) must be the last parameter".to_string(),
                    span: self.current_span(),
                    help: Some("move ... to the end of parameter list".to_string()),
                });
            }
            
            if let Token::Dot3 = self.current() {
                let span = self.current_span();
                self.advance();
                
                self.skip_type_annotation()?;
                
                parameters.push(Parameter::vararg(span));
                seen_vararg = true;
                
                self.consume(Token::Comma);
                continue;
            }
            
            let name = self.parse_identifier()?;
            
            let attribute = if V::HAS_VARIABLE_ATTRIBUTES && matches!(self.current(), Token::Less) {
                self.advance();
                let attr = self.parse_identifier()?;
                self.expect(Token::Greater)?;
                Some(attr)
            } else {
                None
            };
            
            let type_annotation = if V::HAS_TYPE_ANNOTATIONS && matches!(self.current(), Token::Colon) {
                Some(self.parse_type_annotation()?)
            } else {
                None
            };
            
            let span = name.span.clone();
            parameters.push(Parameter::new(Some(name), attribute, type_annotation, false, span));
            
            if !self.consume(Token::Comma) {
                break;
            }
        }
        
        Ok(parameters)
    }

    pub(super) fn parse_variable_name(&mut self) -> Result<VariableName, ParseError> {
        let name = self.parse_identifier()?;
        
        let attribute = if V::HAS_VARIABLE_ATTRIBUTES && matches!(self.current(), Token::Less) {
            self.advance();
            let attr = self.parse_identifier()?;
            self.expect(Token::Greater)?;
            Some(attr)
        } else {
            None
        };
        
        self.skip_type_annotation()?;
        
        Ok(VariableName::new(name, attribute))
    }
    
    pub(super) fn parse_type_annotation(&mut self) -> Result<TypeAnnotation, ParseError> {
        let start = self.current_span().start;
        self.expect(Token::Colon)?;
        self.skip_type_expression()?;
        let end = self.current_span().end;
        Ok(TypeAnnotation::new(start..end))
    }
    
    pub(super) fn skip_type_annotation(&mut self) -> Result<(), ParseError> {
        if V::HAS_TYPE_ANNOTATIONS && matches!(self.current(), Token::Colon) {
            self.advance();
            self.skip_type_expression()?;
        }
        Ok(())
    }
    
    pub(super) fn skip_type_expression(&mut self) -> Result<(), ParseError> {
        loop {
            self.skip_type_primary()?;
            
            while matches!(self.current(), Token::Question) {
                self.advance();
            }
            
            match self.current() {
                Token::Pipe | Token::Ampersand => {
                    self.advance();
                    continue;
                }
                _ => break,
            }
        }
        Ok(())
    }
    
    fn skip_type_primary(&mut self) -> Result<(), ParseError> {
        match self.current() {
            Token::Identifier(name) if name == "typeof" => {
                self.advance();
                self.expect(Token::LParen)?;
                // Skip the expression inside typeof(...)
                // We need to handle nested parens
                let mut depth = 1;
                while depth > 0 && !self.is_eof() {
                    match self.current() {
                        Token::LParen => {
                            depth += 1;
                            self.advance();
                        }
                        Token::RParen => {
                            depth -= 1;
                            if depth > 0 {
                                self.advance();
                            }
                        }
                        _ => {
                            self.advance();
                        }
                    }
                }
                self.expect(Token::RParen)?;
                Ok(())
            }
            Token::Identifier(_) => {
                self.advance();
                
                while matches!(self.current(), Token::Dot) {
                    self.advance();
                    self.expect_identifier()?;
                }
                
                if matches!(self.current(), Token::Less) {
                    self.skip_generic_args()?;
                }
                
                Ok(())
            }
            Token::LBrace => {
                self.advance();
                self.skip_table_type()?;
                self.expect(Token::RBrace)?;
                Ok(())
            }
            Token::LParen => {
                // Could be:
                // 1. Function type params: (T, U) -> V
                // 2. Parenthesized type: (string | number)
                // 3. Empty tuple: ()
                // 4. Tuple return type: (number, string)
                //
                // Strategy: use try_parse with checkpoint to attempt function type first
                let checkpoint = self.checkpoint();
                self.advance(); // consume (
                
                // Handle empty parens () immediately — void/unit type
                if matches!(self.current(), Token::RParen) {
                    self.advance(); // consume )
                    // Could be () -> T (function type) or just () (void type)
                    if matches!(self.current(), Token::Arrow) {
                        self.advance(); // consume ->
                        self.skip_type_expression()?;
                    }
                    return Ok(());
                }
                
                // Try parsing as function type params first
                let try_result = (|| -> Result<bool, ParseError> {
                    self.skip_function_type_params()?;
                    if !matches!(self.current(), Token::RParen) {
                        return Ok(false);
                    }
                    self.advance();
                    Ok(matches!(self.current(), Token::Arrow))
                })();
                
                match try_result {
                    Ok(true) => {
                        // It parsed as function params and has ->
                        self.advance(); // consume ->
                        self.skip_type_expression()?;
                    }
                    _ => {
                        // Fall back: could be parenthesized type or tuple
                        self.restore(checkpoint);
                        self.advance(); // consume (
                        // Parse comma-separated type list
                        self.skip_type_expression()?;
                        while self.consume(Token::Comma) {
                            self.skip_type_expression()?;
                        }
                        self.expect(Token::RParen)?;
                        // Check if it's a function type with ->
                        if matches!(self.current(), Token::Arrow) {
                            self.advance();
                            self.skip_type_expression()?;
                        }
                    }
                }
                
                Ok(())
            }
            Token::String(_) | Token::Number(_) | Token::True | Token::False | Token::Nil => {
                self.advance();
                Ok(())
            }
            Token::Less => {
                // Generic function type: <T, U>(x: T) -> U
                self.skip_generic_args()?;
                self.expect(Token::LParen)?;
                self.skip_function_type_params()?;
                self.expect(Token::RParen)?;
                if matches!(self.current(), Token::Arrow) {
                    self.advance();
                    self.skip_type_expression()?;
                }
                Ok(())
            }
            Token::Dot3 => {
                self.advance();
                self.skip_type_expression()?;
                Ok(())
            }
            _ => Err(ParseError::InvalidSyntax {
                message: "expected type expression".to_string(),
                span: self.current_span(),
                help: None,
            }),
        }
    }
    
    pub fn skip_generic_args(&mut self) -> Result<(), ParseError> {
        self.expect(Token::Less)?;
        
        if matches!(self.current(), Token::Greater) {
            self.advance();
            return Ok(());
        }
        
        loop {
            self.skip_type_expression()?;
            
            match self.current() {
                Token::Comma => {
                    self.advance();
                    continue;
                }
                Token::Greater => {
                    self.advance();
                    break;
                }
                Token::RightShift => {
                    // >> is actually two > closing nested generics like Map<string, Array<number>>
                    self.split_right_shift();
                    self.advance(); // consume first >
                    break;
                }
                Token::GreaterEq => {
                    return Err(ParseError::InvalidSyntax {
                        message: "unexpected '>=' in generic arguments".to_string(),
                        span: self.current_span(),
                        help: Some("use '>' to close generic arguments".to_string()),
                    });
                }
                _ => {
                    return Err(ParseError::UnexpectedToken {
                        expected: vec![",".to_string(), ">".to_string()],
                        found: format!("{:?}", self.current()),
                        span: self.current_span(),
                    });
                }
            }
        }
        
        Ok(())
    }
    
    fn skip_table_type(&mut self) -> Result<(), ParseError> {
        while !matches!(self.current(), Token::RBrace | Token::Eof) {
            self.skip_comments();
            
            if matches!(self.current(), Token::Comma | Token::Semi) {
                self.advance();
                continue;
            }
            
            if matches!(self.current(), Token::LBracket) {
                self.advance();
                self.skip_type_expression()?;
                self.expect(Token::RBracket)?;
                self.expect(Token::Colon)?;
                self.skip_type_expression()?;
            } else if let Token::Identifier(_) = self.current() {
                self.advance();
                if matches!(self.current(), Token::Colon) {
                    self.advance();
                    self.skip_type_expression()?;
                }
            } else {
                break;
            }
        }
        
        Ok(())
    }
    
    fn skip_function_type_params(&mut self) -> Result<(), ParseError> {
        while !matches!(self.current(), Token::RParen | Token::Eof) {
            self.skip_comments();
            
            if matches!(self.current(), Token::Dot3) {
                self.advance();
                if matches!(self.current(), Token::Identifier(_)) {
                    self.advance();
                }
                if matches!(self.current(), Token::Colon) {
                    self.advance();
                    self.skip_type_expression()?;
                }
                break;
            }
            
            if matches!(self.current(), Token::Identifier(_)) {
                self.advance();
                if matches!(self.current(), Token::Colon) {
                    self.advance();
                    self.skip_type_expression()?;
                }
            } else {
                self.skip_type_expression()?;
            }
            
            if !self.consume(Token::Comma) {
                break;
            }
        }
        
        Ok(())
    }
    
    fn expect_identifier(&mut self) -> Result<(), ParseError> {
        if matches!(self.current(), Token::Identifier(_)) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: vec!["identifier".to_string()],
                found: format!("{:?}", self.current()),
                span: self.current_span(),
            })
        }
    }
    
    pub(super) fn parse_attributes(&mut self) -> Result<Vec<Attribute>, ParseError> {
        let mut attributes = Vec::new();
        
        while matches!(self.current(), Token::At) {
            self.advance();
            
            let name = self.parse_identifier()?;
            let fields = if matches!(self.current(), Token::LBracket) {
                self.advance();
                let fields = self.parse_attribute_fields()?;
                self.expect(Token::RBracket)?;
                Some(fields)
            } else {
                None
            };
            
            let span = name.span.clone();
            attributes.push(Attribute::new(name, fields, span));
        }
        
        Ok(attributes)
    }
    
    fn parse_attribute_fields(&mut self) -> Result<Vec<AttributeField>, ParseError> {
        let mut fields = Vec::new();
        
        while !matches!(self.current(), Token::RBracket | Token::Eof) {
            self.skip_comments();
            
            if matches!(self.current(), Token::Comma) {
                self.advance();
                continue;
            }
            
            let checkpoint = self.checkpoint();
            let key = if let Ok(ident) = self.parse_identifier() {
                if matches!(self.current(), Token::Eq) {
                    self.advance();
                    Some(ident)
                } else {
                    self.restore(checkpoint);
                    None
                }
            } else {
                None
            };
            
            let value = self.parse_attribute_value()?;
            fields.push(AttributeField::new(key, value));
            
            if !self.consume(Token::Comma) {
                break;
            }
        }
        
        Ok(fields)
    }
    
    fn parse_attribute_value(&mut self) -> Result<AttributeValue, ParseError> {
        match self.current() {
            Token::String(s) => {
                let s = s.clone();
                self.advance();
                Ok(AttributeValue::String(s))
            }
            Token::Number(n) => {
                let n = n.clone();
                self.advance();
                Ok(AttributeValue::Number(n))
            }
            Token::True => {
                self.advance();
                Ok(AttributeValue::Boolean(true))
            }
            Token::False => {
                self.advance();
                Ok(AttributeValue::Boolean(false))
            }
            Token::Identifier(_) => {
                let ident = self.parse_identifier()?;
                Ok(AttributeValue::Identifier(ident))
            }
            _ => Err(ParseError::UnexpectedToken {
                expected: vec!["string, number, boolean, or identifier".to_string()],
                found: format!("{:?}", self.current()),
                span: self.current_span(),
            }),
        }
    }
}
