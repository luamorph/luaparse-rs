use alloc::{boxed::Box, vec::Vec};

use super::Parser;
use crate::{
    ast::*,
    lexer::Token,
    marker::LuaVersion,
    ParseError,
};

pub(super) fn parse_statement<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
) -> Result<Stmt, ParseError> {
    let start = parser.current_span().start;
    
    let kind = match parser.current() {
        Token::Local => parse_local_statement(parser)?,
        Token::Function => parse_function_statement(parser, false)?,
        Token::If => parse_if_statement(parser)?,
        Token::While => parse_while_loop(parser)?,
        Token::Repeat => parse_repeat_loop(parser)?,
        Token::For => parse_for_loop(parser)?,
        Token::Do => parse_do_block(parser)?,
        Token::Return => parse_return_statement(parser)?,
        Token::Break => {
            parser.advance();
            StmtKind::BreakStatement
        }
        Token::Continue if V::HAS_CONTINUE => {
            parser.advance();
            StmtKind::ContinueStatement
        }
        Token::Goto if V::HAS_GOTO => parse_goto_statement(parser)?,
        Token::ColonColon if V::HAS_GOTO => parse_label_statement(parser)?,
        Token::Type if V::HAS_TYPE_ANNOTATIONS => {
            parser.advance();
            parse_type_declaration(parser)?
        }
        Token::Const if V::HAS_CONST => parse_const_statement(parser)?,
        Token::Export if V::HAS_EXPORT => parse_export_statement(parser)?,
        Token::At if V::HAS_ATTRIBUTES => {
            let attrs = parser.parse_attributes()?;
            return parse_attributed_statement(parser, attrs, start);
        }
        _ => {
            return parse_assignment_or_call(parser, start);
        }
    };
    
    let end = parser.current_span().end;
    Ok(Stmt::new(kind, start..end))
}

fn parse_local_statement<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
) -> Result<StmtKind, ParseError> {
    parser.expect(Token::Local)?;
    
    if matches!(parser.current(), Token::Function) {
        parser.advance();
        
        let attrs = if V::HAS_ATTRIBUTES && matches!(parser.current(), Token::At) {
            parser.parse_attributes()?
        } else {
            Vec::new()
        };
        
        let name = parser.parse_identifier()?;
        
        // Skip generic parameters <T, U> if present (Luau)
        if V::HAS_TYPE_ANNOTATIONS && matches!(parser.current(), Token::Less) {
            parser.skip_generic_args()?;
        }
        
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
        
        let span = name.span.clone();
        return Ok(StmtKind::LocalFunctionDeclaration(
            LocalFunctionDeclaration::new(attrs, name, parameters, return_type, body, span),
        ));
    }
    
    let mut names = vec![parser.parse_variable_name()?];
    
    while parser.consume(Token::Comma) {
        parser.skip_comments();
        names.push(parser.parse_variable_name()?);
    }
    
    let values = if parser.consume(Token::Eq) {
        Some(parse_expression_list(parser)?)
    } else {
        None
    };
    
    let span = names[0].name.span.clone();
    Ok(StmtKind::LocalDeclaration(LocalDeclaration::new(
        names, values, span,
    )))
}

fn parse_const_statement<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
) -> Result<StmtKind, ParseError> {
    parser.expect(Token::Const)?;
    
    // `const function` is impossible
    if matches!(parser.current(), Token::Function) {
        return Err(ParseError::InvalidSyntax {
            message: "cannot use 'const' with function declarations".to_string(),
            span: parser.current_span(),
            help: Some("use 'local function' instead".to_string()),
        });
    }
    
    let mut names = vec![parser.parse_variable_name()?];
    
    while parser.consume(Token::Comma) {
        parser.skip_comments();
        names.push(parser.parse_variable_name()?);
    }
    
    // const declarations MUST have initializers
    if !matches!(parser.current(), Token::Eq) {
        return Err(ParseError::InvalidSyntax {
            message: "const declaration must have a value".to_string(),
            span: parser.current_span(),
            help: Some("add '= <value>' after the variable name".to_string()),
        });
    }
    
    parser.expect(Token::Eq)?;
    let values = parse_expression_list(parser)?;
    
    let span = names[0].name.span.clone();
    Ok(StmtKind::LocalDeclaration(LocalDeclaration::new_const(
        names, values, span,
    )))
}

fn parse_function_statement<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
    _is_local: bool,
) -> Result<StmtKind, ParseError> {
    parser.expect(Token::Function)?;
    
    let attrs = if V::HAS_ATTRIBUTES && matches!(parser.current(), Token::At) {
        parser.parse_attributes()?
    } else {
        Vec::new()
    };
    
    let mut segments = vec![parser.parse_identifier()?];
    
    while parser.consume(Token::Dot) {
        segments.push(parser.parse_identifier()?);
    }
    
    let method = if parser.consume(Token::Colon) {
        Some(parser.parse_identifier()?)
    } else {
        None
    };
    
    let name = FunctionName::new(segments, method);
    
    // Skip generic parameters <T, U> if present (Luau)
    if V::HAS_TYPE_ANNOTATIONS && matches!(parser.current(), Token::Less) {
        parser.skip_generic_args()?;
    }
    
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
    
    let span = body.span.clone();
    Ok(StmtKind::FunctionDeclaration(FunctionDeclaration::new(
        attrs,
        name,
        parameters,
        return_type,
        body,
        span,
    )))
}

fn parse_if_statement<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
) -> Result<StmtKind, ParseError> {
    parser.expect(Token::If)?;
    
    let condition = parser.parse_expression()?;
    parser.expect(Token::Then)?;
    
    let then_block = parser.parse_block_until(&[Token::Elseif, Token::Else, Token::End])?;
    
    let mut elseif_branches = Vec::new();
    while parser.consume(Token::Elseif) {
        let elseif_condition = parser.parse_expression()?;
        parser.expect(Token::Then)?;
        let elseif_block = parser.parse_block_until(&[Token::Elseif, Token::Else, Token::End])?;
        elseif_branches.push(ElseIfBranch::new(elseif_condition, elseif_block));
    }
    
    let else_block = if parser.consume(Token::Else) {
        Some(parser.parse_block_until(&[Token::End])?)
    } else {
        None
    };
    
    parser.expect(Token::End)?;
    
    let span = condition.span.clone();
    Ok(StmtKind::IfStatement(IfStatement::new(
        condition,
        then_block,
        elseif_branches,
        else_block,
        span,
    )))
}

fn parse_while_loop<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
) -> Result<StmtKind, ParseError> {
    parser.expect(Token::While)?;
    
    let condition = parser.parse_expression()?;
    parser.expect(Token::Do)?;
    
    let body = parser.parse_block_until(&[Token::End])?;
    parser.expect(Token::End)?;
    
    let span = condition.span.clone();
    Ok(StmtKind::WhileLoop(WhileLoop::new(condition, body, span)))
}

fn parse_repeat_loop<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
) -> Result<StmtKind, ParseError> {
    parser.expect(Token::Repeat)?;
    
    let body = parser.parse_block_until(&[Token::Until])?;
    parser.expect(Token::Until)?;
    
    let condition = parser.parse_expression()?;
    
    let span = body.span.clone();
    Ok(StmtKind::RepeatLoop(RepeatLoop::new(body, condition, span)))
}

fn parse_for_loop<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
) -> Result<StmtKind, ParseError> {
    parser.expect(Token::For)?;
    
    let variable = parser.parse_identifier()?;
    parser.skip_type_annotation()?;
    
    if parser.consume(Token::Eq) {
        let start = parser.parse_expression()?;
        parser.expect(Token::Comma)?;
        let end = parser.parse_expression()?;
        
        let step = if parser.consume(Token::Comma) {
            Some(parser.parse_expression()?)
        } else {
            None
        };
        
        parser.expect(Token::Do)?;
        let body = parser.parse_block_until(&[Token::End])?;
        parser.expect(Token::End)?;
        
        let span = variable.span.clone();
        Ok(StmtKind::NumericForLoop(NumericForLoop::new(
            variable, start, end, step, body, span,
        )))
    } else {
        let mut variables = vec![variable];
        
        while parser.consume(Token::Comma) {
            variables.push(parser.parse_identifier()?);
            parser.skip_type_annotation()?;
        }
        
        parser.expect(Token::In)?;
        let expressions = parse_expression_list(parser)?;
        
        parser.expect(Token::Do)?;
        let body = parser.parse_block_until(&[Token::End])?;
        parser.expect(Token::End)?;
        
        let span = variables[0].span.clone();
        Ok(StmtKind::GenericForLoop(GenericForLoop::new(
            variables,
            expressions,
            body,
            span,
        )))
    }
}

fn parse_do_block<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
) -> Result<StmtKind, ParseError> {
    parser.expect(Token::Do)?;
    let block = parser.parse_block_until(&[Token::End])?;
    parser.expect(Token::End)?;
    Ok(StmtKind::DoBlock(block))
}

fn parse_return_statement<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
) -> Result<StmtKind, ParseError> {
    let span = parser.current_span();
    parser.expect(Token::Return)?;
    
    let values = if matches!(
        parser.current(),
        Token::End
            | Token::Else
            | Token::Elseif
            | Token::Until
            | Token::Semi
            | Token::Eof
    ) {
        Vec::new()
    } else {
        parse_expression_list(parser)?
    };
    
    Ok(StmtKind::ReturnStatement(ReturnStatement::new(
        values, span,
    )))
}

fn parse_goto_statement<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
) -> Result<StmtKind, ParseError> {
    parser.expect(Token::Goto)?;
    let label = parser.parse_identifier()?;
    let span = label.span.clone();
    Ok(StmtKind::GotoStatement(GotoStatement::new(label, span)))
}

fn parse_label_statement<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
) -> Result<StmtKind, ParseError> {
    parser.expect(Token::ColonColon)?;
    let name = parser.parse_identifier()?;
    parser.expect(Token::ColonColon)?;
    let span = name.span.clone();
    Ok(StmtKind::LabelStatement(LabelStatement::new(name, span)))
}

fn parse_type_declaration<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
) -> Result<StmtKind, ParseError> {
    let name = parser.parse_identifier()?;
    
    let generics_span = if matches!(parser.current(), Token::Less) {
        let start = parser.current_span().start;
        parser.skip_generic_args()?;
        Some(start..parser.current_span().end)
    } else {
        None
    };
    
    parser.expect(Token::Eq)?;
    
    let type_start = parser.current_span().start;
    parser.skip_type_expression()?;
    let type_end = parser.current_span().end;
    
    let span = name.span.clone();
    Ok(StmtKind::TypeDeclaration(TypeDeclaration::new(
        false,
        name,
        generics_span,
        type_start..type_end,
        span,
    )))
}

fn parse_export_statement<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
) -> Result<StmtKind, ParseError> {
    parser.expect(Token::Export)?;
    
    let stmt = parse_statement(parser)?;
    Ok(StmtKind::ExportStatement(Box::new(stmt)))
}

fn parse_attributed_statement<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
    attrs: Vec<Attribute>,
    start: usize,
) -> Result<Stmt, ParseError> {
    if matches!(parser.current(), Token::Function) {
        let kind = parse_function_statement(parser, false)?;
        if let StmtKind::FunctionDeclaration(mut decl) = kind {
            decl.attributes = attrs;
            let end = parser.current_span().end;
            Ok(Stmt::new(StmtKind::FunctionDeclaration(decl), start..end))
        } else {
            unreachable!()
        }
    } else if matches!(parser.current(), Token::Local) {
        let kind = parse_local_statement(parser)?;
        
        if let StmtKind::LocalFunctionDeclaration(mut decl) = kind {
            decl.attributes = attrs;
            let end = parser.current_span().end;
            Ok(Stmt::new(
                StmtKind::LocalFunctionDeclaration(decl),
                start..end,
            ))
        } else {
            Err(ParseError::InvalidSyntax {
                message: "attributes can only be applied to function declarations".to_string(),
                span: parser.current_span(),
                help: None,
            })
        }
    } else {
        Err(ParseError::InvalidSyntax {
            message: "attributes can only be applied to function declarations".to_string(),
            span: parser.current_span(),
            help: None,
        })
    }
}

fn parse_assignment_or_call<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
    start: usize,
) -> Result<Stmt, ParseError> {
    let checkpoint = parser.checkpoint();
    
    match try_parse_assignment(parser, start) {
        Ok(stmt) => return Ok(stmt),
        Err(_) => parser.restore(checkpoint),
    }
    
    let expr = parser.parse_expression()?;
    let end = parser.current_span().end;
    
    match &expr.kind {
        ExprKind::Call(_) | ExprKind::MethodCall(_) => {
            Ok(Stmt::new(StmtKind::CallStatement(expr), start..end))
        }
        _ => Err(ParseError::InvalidSyntax {
            message: "expected statement".to_string(),
            span: expr.span.clone(),
            help: Some("only function calls can be used as statements".to_string()),
        }),
    }
}

fn try_parse_assignment<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
    start: usize,
) -> Result<Stmt, ParseError> {
    let mut targets = vec![parse_assignment_target(parser)?];
    
    while parser.consume(Token::Comma) {
        parser.skip_comments();
        targets.push(parse_assignment_target(parser)?);
    }
    
    parser.skip_comments();
    
    if parser.consume(Token::Eq) {
        let values = parse_expression_list(parser)?;
        let end = parser.current_span().end;
        
        return Ok(Stmt::new(
            StmtKind::Assignment(Assignment::new(targets, values, start..end)),
            start..end,
        ));
    }
    
    // Compound assignments are luau only
    if V::HAS_COMPOUND_ASSIGN {
        if let Some(op) = compound_operator_from_token(parser.current()) {
            if targets.len() != 1 {
                return Err(ParseError::InvalidSyntax {
                    message: "compound assignment requires exactly one target".to_string(),
                    span: parser.current_span(),
                    help: None,
                });
            }
            
            parser.advance();
            let value = parser.parse_expression()?;
            let end = parser.current_span().end;
            
            return Ok(Stmt::new(
                StmtKind::CompoundAssignment(CompoundAssignment::new(
                    targets.into_iter().next().unwrap(),
                    op,
                    value,
                    start..end,
                )),
                start..end,
            ));
        }
    }
    
    Err(ParseError::InvalidSyntax {
        message: "expected assignment".to_string(),
        span: parser.current_span(),
        help: None,
    })
}

fn parse_assignment_target<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
) -> Result<AssignmentTarget, ParseError> {
    // We have to parse primary base as an Expr (identifier or parenthesized expression)
    let mut expr = match parser.current() {
        Token::Identifier(_) => {
            let ident = parser.parse_identifier()?;
            let span = ident.span.clone();
            Expr::new(ExprKind::Identifier(ident), span)
        }
        Token::LParen => {
            parser.advance();
            let inner = parser.parse_expression()?;
            parser.expect(Token::RParen)?;
            inner
        }
        _ => return Err(ParseError::UnexpectedToken {
            expected: alloc::vec!["identifier or (expression)".to_string()],
            found: alloc::format!("{:?}", parser.current()),
            span: parser.current_span(),
        }),
    };

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
                let index = parser.parse_expression()?;
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
                let arguments = parse_call_args_for_target(parser)?;
                let span = expr.span.start..parser.current_span().end;
                expr = Expr::new(
                    ExprKind::MethodCall(MethodCallExpr::new(expr, method, arguments, span.clone())),
                    span,
                );
            }
            Token::LParen | Token::LBrace | Token::String(_) | Token::LongString(_) => {
                let arguments = parse_call_args_for_target(parser)?;
                let span = expr.span.start..parser.current_span().end;
                expr = Expr::new(
                    ExprKind::Call(CallExpr::new(expr, arguments, span.clone())),
                    span,
                );
            }
            _ => break,
        }
    }
    
    // Convert the final expression to an AssignmentTarget
    // Only identifiers, field accesses, and index accesses are valid targets
    expr_to_assignment_target(expr)
}

/// Convert an expression to an assignment target.
/// The expression must end with an identifier, field access, or index access
/// Call expressions in the middle of the chain are fine (e.g. foo().bar),
/// but a bare call result cannot be assigned to (e.g. foo() = 1)
fn expr_to_assignment_target(expr: Expr) -> Result<AssignmentTarget, ParseError> {
    match expr.kind {
        ExprKind::Identifier(id) => Ok(AssignmentTarget::Identifier(id)),
        ExprKind::FieldAccess(fa) => {
            let span = fa.span.clone();
            Ok(AssignmentTarget::FieldAccess {
                base: fa.base,
                field: fa.field,
                span,
            })
        }
        ExprKind::IndexAccess(ia) => {
            let span = ia.span.clone();
            Ok(AssignmentTarget::IndexAccess {
                base: ia.base,
                index: ia.index,
                span,
            })
        }
        _ => Err(ParseError::InvalidSyntax {
            message: "invalid assignment target".to_string(),
            span: expr.span,
            help: Some("only identifiers, field accesses, and index accesses can be assigned to".to_string()),
        }),
    }
}

fn parse_call_args_for_target<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
) -> Result<Vec<Expr>, ParseError> {
    match parser.current() {
        Token::LParen => {
            parser.advance();
            let mut args = Vec::new();
            while !matches!(parser.current(), Token::RParen | Token::Eof) {
                parser.skip_comments();
                args.push(parser.parse_expression()?);
                if !parser.consume(Token::Comma) {
                    break;
                }
            }
            parser.expect(Token::RParen)?;
            Ok(args)
        }
        Token::LBrace => {
            let table = super::expr::parse_expression(parser)?;
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
            expected: alloc::vec!["(, {, or string".to_string()],
            found: alloc::format!("{:?}", parser.current()),
            span: parser.current_span(),
        }),
    }
}

fn compound_operator_from_token(token: &Token) -> Option<CompoundOperator> {
    match token {
        Token::PlusEq => Some(CompoundOperator::Add),
        Token::MinusEq => Some(CompoundOperator::Subtract),
        Token::StarEq => Some(CompoundOperator::Multiply),
        Token::SlashEq => Some(CompoundOperator::Divide),
        Token::FloorDivEq => Some(CompoundOperator::FloorDiv),
        Token::PercentEq => Some(CompoundOperator::Modulo),
        Token::CaretEq => Some(CompoundOperator::Power),
        Token::ConcatEq => Some(CompoundOperator::Concat),
        _ => None,
    }
}

fn parse_expression_list<'src, V: LuaVersion>(
    parser: &mut Parser<'src, V>,
) -> Result<Vec<Expr>, ParseError> {
    let mut expressions = vec![parser.parse_expression()?];
    
    while parser.consume(Token::Comma) {
        parser.skip_comments();
        expressions.push(parser.parse_expression()?);
    }
    
    Ok(expressions)
}