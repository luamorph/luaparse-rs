use alloc::{boxed::Box, string::ToString, vec::Vec};

use super::Parser;
use crate::{
    ast::*,
    lexer::Token,
    marker::LuaVersion,
    ParseError,
    Span,
};

pub(super) fn parse_expression<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
) -> Result<Expr, ParseError> {
    parse_binary_expression(parser, 0)
}

fn parse_binary_expression<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
    min_precedence: u8,
) -> Result<Expr, ParseError> {
    let mut left = parse_unary_or_primary(parser)?;
    
    loop {
        parser.skip_comments();
        let op = match binary_operator_from_token(parser.current()) {
            Some(op) => op,
            None => break,
        };
        
        if !V::HAS_BITWISE_OPS {
            match op {
                BinaryOperator::BitwiseAnd
                | BinaryOperator::BitwiseOr
                | BinaryOperator::BitwiseXor
                | BinaryOperator::LeftShift
                | BinaryOperator::RightShift => {
                    return Err(ParseError::UnsupportedFeature {
                        feature: "bitwise operators".to_string(),
                        version: V::NAME.to_string(),
                        span: parser.current_span(),
                    });
                }
                _ => {}
            }
        }
        
        if !V::HAS_FLOOR_DIV && matches!(op, BinaryOperator::FloorDiv) {
            return Err(ParseError::UnsupportedFeature {
                feature: "floor division (//)".to_string(),
                version: V::NAME.to_string(),
                span: parser.current_span(),
            });
        }
        
        let (left_bp, right_bp) = op.binding_power();
        
        if left_bp < min_precedence {
            break;
        }
        
        parser.advance();
        let right = parse_binary_expression(parser, right_bp)?;
        
        let span = left.span.start..right.span.end;
        left = Expr::new(
            ExprKind::Binary(BinaryExpr::new(op, left, right, span.clone())),
            span,
        );
    }
    
    if matches!(parser.current(), Token::ColonColon) && V::HAS_TYPE_ANNOTATIONS {
        parser.advance();
        let type_start = parser.current_span().start;
        parser.skip_type_expression()?;
        let type_end = parser.current_span().end;
        
        let type_annotation = TypeAnnotation::new(type_start..type_end);
        let span = left.span.start..type_end;
        
        left = Expr::new(
            ExprKind::TypeAssertion(TypeAssertion::new(left, type_annotation, span.clone())),
            span,
        );
    }
    
    Ok(left)
}

fn parse_unary_or_primary<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
) -> Result<Expr, ParseError> {
    if let Some(op) = unary_operator_from_token(parser.current()) {
        if !V::HAS_BITWISE_OPS && matches!(op, UnaryOperator::BitwiseNot) {
            return Err(ParseError::UnsupportedFeature {
                feature: "bitwise not operator".to_string(),
                version: V::NAME.to_string(),
                span: parser.current_span(),
            });
        }
        
        let start = parser.current_span().start;
        parser.advance();
        
        let operand = parse_unary_or_primary(parser)?;
        let end = operand.span.end;
        let span = start..end;
        
        Ok(Expr::new(
            ExprKind::Unary(UnaryExpr::new(op, operand, span.clone())),
            span,
        ))
    } else {
        parse_primary_expression(parser)
    }
}

fn parse_primary_expression<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
) -> Result<Expr, ParseError> {
    let mut expr = parse_simple_expression(parser)?;
    
    loop {
        match parser.current() {
            Token::Dot => {
                parser.advance();
                let field = parser.parse_identifier()?;
                let span = expr.span.start..field.span.end;
                expr = Expr::new(
                    ExprKind::FieldAccess(FieldAccess::new(expr, field, span.clone())),
                    span,
                );
            }
            Token::LBracket => {
                parser.advance();
                let index = parse_expression(parser)?;
                parser.expect(Token::RBracket)?;
                let span = expr.span.start..parser.current_span().end;
                expr = Expr::new(
                    ExprKind::IndexAccess(IndexAccess::new(expr, index, span.clone())),
                    span,
                );
            }
            Token::Colon => {
                parser.advance();
                let method = parser.parse_identifier()?;
                
                let arguments = parse_call_arguments(parser)?;
                let span = expr.span.start..parser.current_span().end;
                
                expr = Expr::new(
                    ExprKind::MethodCall(MethodCallExpr::new(expr, method, arguments, span.clone())),
                    span,
                );
            }
            Token::LParen | Token::LBrace | Token::String(_) | Token::LongString(_) => {
                let arguments = parse_call_arguments(parser)?;
                let span = expr.span.start..parser.current_span().end;
                expr = Expr::new(
                    ExprKind::Call(CallExpr::new(expr, arguments, span.clone())),
                    span,
                );
            }
            Token::LeftShift if V::HAS_TYPE_ANNOTATIONS => {
                parser.split_left_shift();
                parser.advance(); // first <
                let type_start = parser.current_span().start;
                parser.skip_generic_args()?;
                let type_end = parser.current_span().end;
                let type_annotation = TypeAnnotation::new(type_start..type_end);
                let span = expr.span.start..type_end;
                expr = Expr::new(
                    ExprKind::TypeInstantiation(TypeInstantiation::new(expr, type_annotation, span.clone())),
                    span,
                );
            }
            _ => break,
        }
    }
    
    Ok(expr)
}

fn parse_simple_expression<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
) -> Result<Expr, ParseError> {
    let start = parser.current_span().start;
    
    match parser.current() {
        Token::Nil => {
            parser.advance();
            let span = start..parser.current_span().start;
            Ok(Expr::new(ExprKind::Nil, span))
        }
        Token::True => {
            parser.advance();
            let span = start..parser.current_span().start;
            Ok(Expr::new(ExprKind::Boolean(true), span))
        }
        Token::False => {
            parser.advance();
            let span = start..parser.current_span().start;
            Ok(Expr::new(ExprKind::Boolean(false), span))
        }
        Token::Number(raw) => {
            let raw = raw.clone();
            parser.advance();
            let end = parser.current_span().start;
            let span = start..end;
            Ok(Expr::new(
                ExprKind::Number(NumberLiteral::new(raw, span.clone())),
                span,
            ))
        }
        Token::String(value) => {
            let value = value.clone();
            parser.advance();
            let end = parser.current_span().start;
            let span = start..end;
            Ok(Expr::new(
                ExprKind::String(StringLiteral::new(value, span.clone())),
                span,
            ))
        }
        Token::LongString(value) => {
            let value = value.clone();
            parser.advance();
            let end = parser.current_span().start;
            let span = start..end;
            Ok(Expr::new(
                ExprKind::String(StringLiteral::new(value, span.clone())),
                span,
            ))
        }
        Token::InterpolatedString(parts) if V::HAS_STRING_INTERP => {
            let parts = parts.clone();
            let span = parser.current_span();
            parser.advance();
            
            let segments = parse_interpolated_segments(parser, parts)?;
            Ok(Expr::new(
                ExprKind::InterpolatedString(InterpolatedString::new(segments, span.clone())),
                span,
            ))
        }
        Token::Dot3 => {
            parser.advance();
            let span = start..parser.current_span().start;
            Ok(Expr::new(ExprKind::Vararg, span))
        }
        Token::LBrace => {
            parse_table_constructor(parser)
        }
        Token::Function => {
            parser.advance();
            parse_function_expression(parser, start)
        }
        Token::If if V::HAS_IF_EXPR => {
            parser.advance();
            parse_if_expression(parser, start)
        }
        Token::Identifier(_) => {
            let ident = parser.parse_identifier()?;
            let span = ident.span.clone();
            Ok(Expr::new(ExprKind::Identifier(ident), span))
        }
        Token::LParen => {
            parser.advance();
            let expr = parse_expression(parser)?;
            parser.expect(Token::RParen)?;
            let end = parser.current_span().end;
            let span = start..end;
            Ok(Expr::new(ExprKind::Parenthesized(Box::new(expr)), span))
        }
        _ => Err(ParseError::UnexpectedToken {
            expected: vec!["expression".to_string()],
            found: format!("{:?}", parser.current()),
            span: parser.current_span(),
        }),
    }
}

fn parse_table_constructor<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
) -> Result<Expr, ParseError> {
    let start = parser.current_span().start;
    parser.expect(Token::LBrace)?;
    
    let mut fields = Vec::new();
    
    while !matches!(parser.current(), Token::RBrace | Token::Eof) {
        parser.skip_comments();
        
        while matches!(parser.current(), Token::Comma | Token::Semi) {
            parser.advance();
            parser.skip_comments();
        }
        
        if matches!(parser.current(), Token::RBrace) {
            break;
        }
        
        let field_start = parser.current_span().start;
        
        if matches!(parser.current(), Token::LBracket) {
            parser.advance();
            let key = parse_expression(parser)?;
            parser.expect(Token::RBracket)?;
            parser.expect(Token::Eq)?;
            let value = parse_expression(parser)?;
            
            let field_end = value.span.end;
            fields.push(TableField::new(
                TableFieldKind::Bracketed { key, value },
                field_start..field_end,
            ));
        } else {
            let checkpoint = parser.checkpoint();
            
            if let Ok(name) = parser.parse_identifier() {
                if parser.consume(Token::Eq) {
                    let value = parse_expression(parser)?;
                    let field_end = value.span.end;
                    fields.push(TableField::new(
                        TableFieldKind::Named { name, value },
                        field_start..field_end,
                    ));
                } else {
                    parser.restore(checkpoint);
                    let value = parse_expression(parser)?;
                    let field_end = value.span.end;
                    fields.push(TableField::new(
                        TableFieldKind::Positional(value),
                        field_start..field_end,
                    ));
                }
            } else {
                parser.restore(checkpoint);
                let value = parse_expression(parser)?;
                let field_end = value.span.end;
                fields.push(TableField::new(
                    TableFieldKind::Positional(value),
                    field_start..field_end,
                ));
            }
        }
        
        if matches!(parser.current(), Token::Comma | Token::Semi) {
            parser.advance();
        }
    }
    
    parser.expect(Token::RBrace)?;
    let end = parser.current_span().end;
    let span = start..end;
    
    Ok(Expr::new(
        ExprKind::Table(TableConstructor::new(fields, span.clone())),
        span,
    ))
}

fn parse_function_expression<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
    start: usize,
) -> Result<Expr, ParseError> {
    parser.expect(Token::LParen)?;
    let parameters = parser.parse_parameters()?;
    parser.expect(Token::RParen)?;
    
    let return_type = if V::HAS_TYPE_ANNOTATIONS && matches!(parser.current(), Token::Colon) {
        Some(parser.parse_type_annotation()?)
    } else {
        None
    };
    
    let body = parser.parse_block_until(&[Token::End])?;
    parser.expect(Token::End)?;
    
    let end = parser.current_span().end;
    let span = start..end;
    
    Ok(Expr::new(
        ExprKind::Function(FunctionExpr::new(parameters, return_type, body, span.clone())),
        span,
    ))
}

fn parse_if_expression<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
    start: usize,
) -> Result<Expr, ParseError> {
    let condition = parse_expression(parser)?;
    parser.expect(Token::Then)?;
    let then_branch = parse_expression(parser)?;
    
    let mut elseif_branches = Vec::new();
    while parser.consume(Token::Elseif) {
        let elseif_condition = parse_expression(parser)?;
        parser.expect(Token::Then)?;
        let elseif_then = parse_expression(parser)?;
        elseif_branches.push(ElseIfExprBranch::new(elseif_condition, elseif_then));
    }
    
    parser.expect(Token::Else)?;
    let else_branch = parse_expression(parser)?;
    
    let end = else_branch.span.end;
    let span = start..end;
    
    Ok(Expr::new(
        ExprKind::IfExpression(IfExpression::new(
            condition,
            then_branch,
            elseif_branches,
            else_branch,
            span.clone(),
        )),
        span,
    ))
}

fn parse_call_arguments<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
) -> Result<Vec<Expr>, ParseError> {
    match parser.current() {
        Token::LParen => {
            parser.advance();
            let mut args = Vec::new();
            
            while !matches!(parser.current(), Token::RParen | Token::Eof) {
                parser.skip_comments();
                args.push(parse_expression(parser)?);
                
                if !parser.consume(Token::Comma) {
                    break;
                }
            }
            
            parser.expect(Token::RParen)?;
            Ok(args)
        }
        Token::LBrace => {
            let table = parse_table_constructor(parser)?;
            Ok(vec![table])
        }
        Token::String(s) => {
            let s = s.clone();
            let span = parser.current_span();
            parser.advance();
            Ok(vec![Expr::new(
                ExprKind::String(StringLiteral::new(s, span.clone())),
                span,
            )])
        }
        Token::LongString(s) => {
            let s = s.clone();
            let span = parser.current_span();
            parser.advance();
            Ok(vec![Expr::new(
                ExprKind::String(StringLiteral::new(s, span.clone())),
                span,
            )])
        }
        _ => Err(ParseError::UnexpectedToken {
            expected: vec!["(, {, or string".to_string()],
            found: format!("{:?}", parser.current()),
            span: parser.current_span(),
        }),
    }
}

fn parse_interpolated_segments<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
    parts: Vec<crate::lexer::InterpolationPart>,
) -> Result<Vec<InterpolationSegment>, ParseError> {
    use alloc::string::ToString;
    use crate::lexer::InterpolationPart;
    
    let mut segments = Vec::new();
    
    for part in parts {
        match part {
            InterpolationPart::Text(text) => {
                segments.push(InterpolationSegment::Text(text));
            }
            InterpolationPart::ExprSpan { start, end } => {
                let expr_str = &parser.source[start..end];
                
                if expr_str.trim().is_empty() {
                    return Err(ParseError::InvalidSyntax {
                        message: "empty expression in string interpolation".to_string(),
                        span: start..end,
                        help: Some("use \\{} to escape braces".to_string()),
                    });
                }
                
                let expr_tokens = match crate::lexer::lex_for_version::<V>(expr_str) {
                    Ok(tokens) => tokens,
                    Err(e) => {
                        return Err(ParseError::InvalidSyntax {
                            message: alloc::format!("invalid expression in interpolation: {:?}", e),
                            span: start..end,
                            help: None,
                        });
                    }
                };
                
                let adjusted_tokens: Vec<_> = expr_tokens
                    .into_iter()
                    .map(|(tok, span)| {
                        let adjusted_span = (span.start + start)..(span.end + start);
                        (tok, adjusted_span)
                    })
                    .collect();
                
                let expr = parse_expr_from_tokens::<V>(adjusted_tokens, parser.source, start..end)?;
                segments.push(InterpolationSegment::Expression(expr));
            }
        }
    }
    
    Ok(segments)
}

fn parse_expr_from_tokens<V: LuaVersion>(
    tokens: Vec<(crate::lexer::Token, Span)>,
    source: &str,
    context_span: Span,
) -> Result<Expr, ParseError> {
    let mut temp_parser = Parser {
        tokens,
        position: 0,
        source,
        comments: Vec::new(),
        _version: core::marker::PhantomData::<V>,
    };
    
    temp_parser.parse_expression().map_err(|e| {
        ParseError::InvalidSyntax {
            message: alloc::format!("failed to parse interpolation expression: {}", e),
            span: context_span,
            help: Some("check expression syntax inside {}".to_string()),
        }
    })
}

fn binary_operator_from_token(token: &Token) -> Option<BinaryOperator> {
    match token {
        Token::Plus => Some(BinaryOperator::Add),
        Token::Minus => Some(BinaryOperator::Subtract),
        Token::Star => Some(BinaryOperator::Multiply),
        Token::Slash => Some(BinaryOperator::Divide),
        Token::FloorDiv => Some(BinaryOperator::FloorDiv),
        Token::Percent => Some(BinaryOperator::Modulo),
        Token::Caret => Some(BinaryOperator::Power),
        Token::Dot2 => Some(BinaryOperator::Concat),
        Token::EqEq => Some(BinaryOperator::Equal),
        Token::NotEq => Some(BinaryOperator::NotEqual),
        Token::Less => Some(BinaryOperator::Less),
        Token::LessEq => Some(BinaryOperator::LessEqual),
        Token::Greater => Some(BinaryOperator::Greater),
        Token::GreaterEq => Some(BinaryOperator::GreaterEqual),
        Token::And => Some(BinaryOperator::And),
        Token::Or => Some(BinaryOperator::Or),
        Token::LeftShift => Some(BinaryOperator::LeftShift),
        Token::RightShift => Some(BinaryOperator::RightShift),
        Token::Pipe => Some(BinaryOperator::BitwiseOr),
        Token::Ampersand => Some(BinaryOperator::BitwiseAnd),
        Token::Tilde => Some(BinaryOperator::BitwiseXor),
        _ => None,
    }
}

fn unary_operator_from_token(token: &Token) -> Option<UnaryOperator> {
    match token {
        Token::Minus => Some(UnaryOperator::Minus),
        Token::Not => Some(UnaryOperator::Not),
        Token::Hash => Some(UnaryOperator::Length),
        Token::Tilde => Some(UnaryOperator::BitwiseNot),
        _ => None,
    }
}
