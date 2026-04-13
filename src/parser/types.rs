use alloc::{boxed::Box, string::ToString, vec::Vec};

use super::Parser;
use crate::{ast::types::*, lexer::Token, marker::LuaVersion, ParseError, Span};

impl<'src, V: LuaVersion> Parser<'src, V> {
    pub(super) fn parse_type_declaration_full(
        &mut self,
    ) -> Result<TypeDeclarationFull, ParseError> {
        let start = self.current_span().start;

        let exported = false; // will be set by caller if preceded by 'export'

        let name = self.parse_identifier()?;

        let generics = if self.consume(Token::Less) {
            let params = self.parse_generic_parameters()?;
            match self.current() {
                Token::Greater => {
                    self.advance();
                }
                Token::GreaterEq => {
                    self.split_greater_eq();
                }
                _ => {
                    return Err(ParseError::UnexpectedToken {
                        expected: alloc::vec![">".to_string()],
                        found: alloc::format!("{:?}", self.current()),
                        span: self.current_span(),
                    })
                }
            }
            params
        } else {
            Vec::new()
        };

        self.expect(Token::Eq)?;

        let type_expr = self.parse_type_expr()?;

        let end = self.current_span().end;

        Ok(TypeDeclarationFull::new(
            exported,
            name,
            generics,
            type_expr,
            start..end,
        ))
    }

    pub(super) fn parse_type_expr(&mut self) -> Result<TypeExpr, ParseError> {
        self.parse_union_or_intersection()
    }

    fn parse_union_or_intersection(&mut self) -> Result<TypeExpr, ParseError> {
        let start = self.current_span().start;
        let mut types = Vec::new();

        let mut is_union = false;
        let mut is_intersection = false;

        // leading | or &
        match self.current() {
            Token::Pipe => {
                is_union = true;
                self.advance();
            }
            Token::Ampersand => {
                is_intersection = true;
                self.advance();
            }
            _ => {}
        }

        types.push(self.parse_type_primary()?);

        loop {
            match self.current() {
                Token::Pipe => {
                    if is_intersection {
                        return Err(ParseError::InvalidSyntax {
                            message: "cannot mix union and intersection types without parentheses"
                                .to_string(),
                            span: self.current_span(),
                            help: Some("use parentheses to clarify precedence".to_string()),
                        });
                    }
                    is_union = true;
                    self.advance();
                    types.push(self.parse_type_primary()?);
                }
                Token::Ampersand => {
                    if is_union {
                        return Err(ParseError::InvalidSyntax {
                            message: "cannot mix union and intersection types without parentheses"
                                .to_string(),
                            span: self.current_span(),
                            help: Some("use parentheses to clarify precedence".to_string()),
                        });
                    }
                    is_intersection = true;
                    self.advance();
                    types.push(self.parse_type_primary()?);
                }
                _ => break,
            }
        }

        if types.len() == 1 {
            return Ok(types.into_iter().next().unwrap());
        }

        let end = self.current_span().end;
        let span = start..end;

        let kind = if is_union {
            TypeExprKind::Union(types)
        } else {
            TypeExprKind::Intersection(types)
        };

        Ok(TypeExpr::new(kind, span))
    }

    fn parse_type_primary(&mut self) -> Result<TypeExpr, ParseError> {
        let start = self.current_span().start;

        let mut base = self.parse_type_base()?;

        while self.consume(Token::Question) {
            let end = self.current_span().end;
            let span = start..end;
            base = TypeExpr::new(TypeExprKind::Optional(Box::new(base)), span);
        }

        Ok(base)
    }

    fn parse_type_base(&mut self) -> Result<TypeExpr, ParseError> {
        let start = self.current_span().start;

        match self.current() {
            Token::Nil => {
                self.advance();
                Ok(TypeExpr::new(
                    TypeExprKind::Nil,
                    start..self.current_span().start,
                ))
            }
            Token::True => {
                self.advance();
                Ok(TypeExpr::new(
                    TypeExprKind::Boolean(true),
                    start..self.current_span().start,
                ))
            }
            Token::False => {
                self.advance();
                Ok(TypeExpr::new(
                    TypeExprKind::Boolean(false),
                    start..self.current_span().start,
                ))
            }
            Token::Number(n) => {
                let n = n.clone();
                self.advance();
                Ok(TypeExpr::new(
                    TypeExprKind::Number(n),
                    start..self.current_span().start,
                ))
            }
            Token::String(s) => {
                let s = s.clone();
                self.advance();
                Ok(TypeExpr::new(
                    TypeExprKind::String(s),
                    start..self.current_span().start,
                ))
            }
            Token::Identifier(name) if name == "typeof" => {
                self.advance();
                self.expect(Token::LParen)?;
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                let end = self.current_span().end;
                Ok(TypeExpr::new(
                    TypeExprKind::Typeof(Box::new(expr)),
                    start..end,
                ))
            }
            Token::Identifier(_) => {
                let mut path = vec![self.parse_identifier()?];

                while self.consume(Token::Dot) {
                    path.push(self.parse_identifier()?);
                }

                let generics = if self.consume(Token::Less) {
                    let types = self.parse_type_arguments()?;
                    match self.current() {
                        Token::Greater => {
                            self.advance();
                        }
                        Token::RightShift => {
                            self.split_right_shift();
                            self.advance();
                        }
                        Token::GreaterEq => {
                            self.split_greater_eq();
                            self.advance();
                        }
                        _ => {
                            return Err(ParseError::UnexpectedToken {
                                expected: alloc::vec![">".to_string()],
                                found: alloc::format!("{:?}", self.current()),
                                span: self.current_span(),
                            });
                        }
                    }
                    Some(types)
                } else {
                    None
                };

                let end = self.current_span().end;
                Ok(TypeExpr::new(
                    TypeExprKind::Named { path, generics },
                    start..end,
                ))
            }
            Token::LBrace => {
                self.advance();
                let table = self.parse_table_type()?;
                self.expect(Token::RBrace)?;
                let end = self.current_span().end;
                Ok(TypeExpr::new(
                    TypeExprKind::Table(Box::new(table)),
                    start..end,
                ))
            }
            Token::LParen => {
                self.advance();

                let checkpoint = self.checkpoint();

                if let Some(params) = self.try_parse(|p| p.parse_function_type_parameters()) {
                    self.expect(Token::RParen)?;
                    self.expect(Token::Arrow)?;
                    let return_type = self.parse_type_expr()?;
                    let end = self.current_span().end;

                    return Ok(TypeExpr::new(
                        TypeExprKind::Function(Box::new(FunctionType::new(
                            Vec::new(),
                            params,
                            return_type,
                            start..end,
                        ))),
                        start..end,
                    ));
                }

                // Not a function, must be grouped type
                self.restore(checkpoint);
                let inner = self.parse_type_expr()?;
                self.expect(Token::RParen)?;
                let end = self.current_span().end;
                Ok(TypeExpr::new(
                    TypeExprKind::Parenthesized(Box::new(inner)),
                    start..end,
                ))
            }
            Token::Less => {

                // <T, U>(x: T) -> U

                self.advance();
                let generics = self.parse_generic_parameters()?;
                match self.current() {
                    Token::Greater => {
                        self.advance();
                    }
                    Token::RightShift => {
                        self.split_right_shift();
                        self.advance();
                    }
                    Token::GreaterEq => {
                        self.split_greater_eq();
                    }
                    _ => return Err(ParseError::UnexpectedToken {
                        expected: alloc::vec![">".to_string()],
                        found: alloc::format!("{:?}", self.current()),
                        span: self.current_span(),
                    }),
                }

                self.expect(Token::LParen)?;
                let params = self.parse_function_type_parameters()?;
                self.expect(Token::RParen)?;
                self.expect(Token::Arrow)?;
                let return_type = self.parse_type_expr()?;
                let end = self.current_span().end;

                Ok(TypeExpr::new(
                    TypeExprKind::Function(Box::new(FunctionType::new(
                        generics,
                        params,
                        return_type,
                        start..end,
                    ))),
                    start..end,
                ))
            }
            Token::Dot3 => {
                self.advance();
                let inner = self.parse_type_expr()?;
                let end = self.current_span().end;
                Ok(TypeExpr::new(
                    TypeExprKind::VariadicPack(Box::new(inner)),
                    start..end,
                ))
            }
            _ => Err(ParseError::UnexpectedToken {
                expected: alloc::vec!["type expression".to_string()],
                found: alloc::format!("{:?}", self.current()),
                span: self.current_span(),
            }),
        }
    }

    fn parse_table_type(&mut self) -> Result<TableType, ParseError> {
        let start = self.current_span().start;
        let mut properties = Vec::new();
        let mut indexer = None;

        while !matches!(self.current(), Token::RBrace | Token::Eof) {
            self.skip_comments();

            if matches!(self.current(), Token::Comma | Token::Semi) {
                self.advance();
                continue;
            }

            // [KeyType]: ValueType
            if self.consume(Token::LBracket) {
                let key_type = self.parse_type_expr()?;
                self.expect(Token::RBracket)?;
                self.expect(Token::Colon)?;
                let value_type = self.parse_type_expr()?;
                let end = self.current_span().end;

                indexer = Some(Box::new(TableIndexer::new(
                    key_type,
                    value_type,
                    None,
                    start..end,
                )));

                self.consume(Token::Comma);
                continue;
            }

            // name: Type
            let name = self.parse_identifier()?;
            self.expect(Token::Colon)?;
            let type_expr = self.parse_type_expr()?;
            let end = self.current_span().end;

            properties.push(TableProperty::new(name, type_expr, None, start..end));

            if !self.consume(Token::Comma) && !self.consume(Token::Semi) {
                break;
            }
        }

        let end = self.current_span().end;
        Ok(TableType::new(properties, indexer, start..end))
    }

    fn parse_function_type_parameters(&mut self) -> Result<Vec<FunctionTypeParameter>, ParseError> {
        let mut params = Vec::new();

        while !matches!(self.current(), Token::RParen | Token::Eof) {
            self.skip_comments();

            if self.consume(Token::Comma) {
                continue;
            }

            // variadic check
            if self.consume(Token::Dot3) {
                let type_expr = self.parse_type_expr()?;
                params.push(FunctionTypeParameter::new(
                    None,
                    type_expr,
                    self.current_span(),
                ));
                break;
            }

            let checkpoint = self.checkpoint();

            if let Ok(name) = self.parse_identifier() {
                if self.consume(Token::Colon) {
                    let type_expr = self.parse_type_expr()?;
                    params.push(FunctionTypeParameter::new(
                        Some(name),
                        type_expr,
                        self.current_span(),
                    ));
                } else {
                    self.restore(checkpoint);
                    let type_expr = self.parse_type_expr()?;
                    params.push(FunctionTypeParameter::new(
                        None,
                        type_expr,
                        self.current_span(),
                    ));
                }
            } else {
                let type_expr = self.parse_type_expr()?;
                params.push(FunctionTypeParameter::new(
                    None,
                    type_expr,
                    self.current_span(),
                ));
            }

            if !self.consume(Token::Comma) {
                break;
            }
        }

        Ok(params)
    }

    fn parse_type_arguments(&mut self) -> Result<Vec<TypeExpr>, ParseError> {
        let mut types = Vec::new();

        loop {
            types.push(self.parse_type_expr()?);

            if !self.consume(Token::Comma) {
                break;
            }
        }

        Ok(types)
    }

    fn parse_generic_parameters(&mut self) -> Result<Vec<GenericParameter>, ParseError> {
        let mut params = Vec::new();

        while !matches!(
            self.current(),
            Token::Greater | Token::GreaterEq | Token::RightShift | Token::Eof
        ) {
            self.skip_comments();

            if self.consume(Token::Comma) {
                continue;
            }

            let name = self.parse_identifier()?;

            let is_pack = self.consume(Token::Dot3);

            let constraint = None;
            let default = if self.consume(Token::Eq) {
                Some(self.parse_type_expr()?)
            } else {
                None
            };

            params.push(GenericParameter::new(
                name.clone(),
                constraint,
                default,
                is_pack,
                name.span,
            ));

            if !self.consume(Token::Comma) {
                break;
            }
        }

        Ok(params)
    }
}
