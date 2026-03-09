use alloc::{string::String, vec::Vec, format};

use logos::Logos;
use crate::{Span, LexError};

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r\n]+")]
pub enum Token {
    // "shebang" is a line at the start of the file, it'll look like this:
    // #!/usr/bin/lua
    // it's used in unix-like systems to indicate which interpreter should be used to run the script
    // we give it a high priority so that it matches before any other token
    // since its only valid at the start of the file (in lua), we don't need to worry about it appearing elsewhere
    // unless it's invalid
    #[regex(r"#![^\n]*", priority = 10)]
    Shebang,

    #[token("true")]
    True,
    
    #[token("false")]
    False,
    
    #[token("nil")]
    Nil,
    
    #[token("and")]
    And,
    
    #[token("break")]
    Break,
    
    #[token("do")]
    Do,
    
    #[token("else")]
    Else,
    
    #[token("elseif")]
    Elseif,
    
    #[token("end")]
    End,
    
    #[token("for")]
    For,
    
    #[token("function")]
    Function,
    
    #[token("if")]
    If,
    
    #[token("in")]
    In,
    
    #[token("local")]
    Local,
    
    #[token("not")]
    Not,
    
    #[token("or")]
    Or,
    
    #[token("repeat")]
    Repeat,
    
    #[token("return")]
    Return,
    
    #[token("then")]
    Then,
    
    #[token("until")]
    Until,
    
    #[token("while")]
    While,
    
    #[token("continue")]
    Continue,
    
    #[token("export")]
    Export,
    
    #[token("type")]
    Type,
    
    #[token("goto")]
    Goto,
    
    #[token("const")]
    Const,
    
    // here, we use \p{L} for any unciode letter and \p{N} for any unicode number
    // as lua 5.3+ allows for unicode identifiers to be used
    // see: https://www.lua.org/manual/5.4/manual.html#3.1
    #[regex(r"[\p{L}_][\p{L}\p{N}_]*", |lex| lex.slice().to_string())]
    Identifier(String),
    
   #[regex(r"0[xX][0-9a-fA-F_]*(\.[0-9a-fA-F_]*)?([pP][+-]?\d*)?|\d[0-9_]*(\.\d[0-9_]*)?([eE][+-]?\d*)?|\.\d[0-9_]*([eE][+-]?\d*)?|0[bB][01_]*", |lex| lex.slice().to_string())]
    Number(String),
    
    #[regex(r#""([^"\\]|\\.)*""#, parse_string)]
    #[regex(r#"'([^'\\]|\\.)*'"#, parse_string)]
    String(String),
    
    #[token("`", parse_interpolation_parts)]
    InterpolatedString(Vec<InterpolationPart>),
    
    #[regex(r"--", parse_comment)]
    Comment(String),
    
    #[regex(r"\[[=]*\[", parse_long_string)]
    LongString(String),
    
    #[token("+")]
    Plus,
    
    #[token("-")]
    Minus,
    
    #[token("*")]
    Star,
    
    #[token("/")]
    Slash,
    
    #[token("//")]
    FloorDiv,
    
    #[token("%")]
    Percent,
    
    #[token("^")]
    Caret,
    
    #[token("#")]
    Hash,
    
    #[token("==")]
    EqEq,
    
    #[token("~=")]
    NotEq,
    
    #[token("<=")]
    LessEq,
    
    #[token(">=")]
    GreaterEq,
    
    #[token("<")]
    Less,
    
    #[token(">")]
    Greater,
    
    #[token("=")]
    Eq,
    
    #[token("+=")]
    PlusEq,
    
    #[token("-=")]
    MinusEq,
    
    #[token("*=")]
    StarEq,
    
    #[token("/=")]
    SlashEq,
    
    #[token("//=")]
    FloorDivEq,
    
    #[token("%=")]
    PercentEq,
    
    #[token("^=")]
    CaretEq,
    
    #[token("..=")]
    ConcatEq,
    
    #[token("(")]
    LParen,
    
    #[token(")")]
    RParen,
    
    #[token("{")]
    LBrace,
    
    #[token("}")]
    RBrace,
    
    #[token("[")]
    LBracket,
    
    #[token("]")]
    RBracket,
    
    #[token("::")]
    ColonColon,
    
    #[token(":")]
    Colon,
    
    #[token(";")]
    Semi,
    
    #[token(",")]
    Comma,
    
    #[token("...")]
    Dot3,
    
    #[token("..")]
    Dot2,
    
    #[token(".")]
    Dot,
    
    #[token("->")]
    Arrow,
    
    #[token("|")]
    Pipe,
    
    #[token("&")]
    Ampersand,
    
    #[token("?")]
    Question,
    
    #[token("@")]
    At,
    
    #[token("<<")]
    LeftShift,
    
    #[token(">>")]
    RightShift,
    
    #[token("~")]
    Tilde,
    
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InterpolationPart {
    Text(String),
    ExprSpan { start: usize, end: usize },
}

fn parse_string(lex: &mut logos::Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let content = &slice[1..slice.len() - 1];
    Some(unescape_string(content))
}

fn parse_interpolation_parts(lex: &mut logos::Lexer<Token>) -> Option<Vec<InterpolationPart>> {
    let start = lex.span().end;
    let source = lex.source();
    let bytes = source.as_bytes();
    
    let mut parts = Vec::new();
    let mut current_text = String::new();
    let mut pos = start;
    
    while pos < bytes.len() {
        match bytes[pos] {
            b'`' => {
                if !current_text.is_empty() {
                    parts.push(InterpolationPart::Text(current_text));
                }
                lex.bump(pos - start + 1);
                return Some(parts);
            }
            b'{' => {
                if !current_text.is_empty() {
                    parts.push(InterpolationPart::Text(current_text.clone()));
                    current_text.clear();
                }
                
                let expr_start = pos + 1;
                let mut depth = 1;
                pos += 1;
                
                while pos < bytes.len() && depth > 0 {
                    match bytes[pos] {
                        b'{' => depth += 1,
                        b'}' => depth -= 1,
                        _ => {}
                    }
                    pos += 1;
                }
                
                if depth != 0 {
                    return None;
                }
                
                let expr_end = pos - 1;
                parts.push(InterpolationPart::ExprSpan {
                    start: expr_start,
                    end: expr_end,
                });
            }
            b'\\' if pos + 1 < bytes.len() => {
                match bytes[pos + 1] {
                    b'n' => {
                        current_text.push('\n');
                        pos += 2;
                    }
                    b't' => {
                        current_text.push('\t');
                        pos += 2;
                    }
                    b'r' => {
                        current_text.push('\r');
                        pos += 2;
                    }
                    b'\\' | b'`' | b'{' | b'}' => {
                        current_text.push(bytes[pos + 1] as char);
                        pos += 2;
                    }
                    _ => {
                        current_text.push(bytes[pos] as char);
                        pos += 1;
                    }
                }
            }
            b => {
                current_text.push(b as char);
                pos += 1;
            }
        }
    }
    
    None
}

fn parse_comment(lex: &mut logos::Lexer<Token>) -> Option<String> {
    let start = lex.span().end;
    let source = lex.source();
    let rest = &source[start..];
    
    // Check if this is a block comment: --[[ or --[=*[
    if rest.starts_with('[') {
        let after_bracket = &rest[1..];
        let eq_count = after_bracket.chars().take_while(|&c| c == '=').count();
        if after_bracket.len() > eq_count && after_bracket[eq_count..].starts_with('[') {
            // It's a block comment — find the matching closing ]=*]
            let closing = format!("]{}]", "=".repeat(eq_count));
            let block_start = 1 + eq_count + 1; // skip [=*[
            let content_start = start + block_start;
            
            if let Some(end_pos) = source[content_start..].find(&closing) {
                let content = source[content_start..content_start + end_pos].to_string();
                lex.bump(block_start + end_pos + closing.len());
                return Some(content);
            } else {
                // Unterminated block comment — consume rest as comment
                let content = source[content_start..].to_string();
                lex.bump(source.len() - start);
                return Some(content);
            }
        }
    }
    
    // Regular line comment: consume until newline or EOF
    if let Some(newline_pos) = rest.find('\n') {
        let content = rest[..newline_pos].trim().to_string();
        lex.bump(newline_pos);
        Some(content)
    } else {
        let content = rest.trim().to_string();
        lex.bump(rest.len());
        Some(content)
    }
}

fn parse_long_string(lex: &mut logos::Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    
    let equals_count = slice.chars().filter(|&c| c == '=').count();
    let closing = format!("]{}]", "=".repeat(equals_count));
    
    let start = lex.span().end;
    let source = lex.source();
    
    let actual_start = if source[start..].starts_with('\n') {
        start + 1
    } else if source[start..].starts_with("\r\n") {
        start + 2
    } else {
        start
    };
    
    if let Some(end_pos) = source[actual_start..].find(&closing) {
        let content = source[actual_start..actual_start + end_pos].to_string();
        lex.bump(actual_start - start + end_pos + closing.len());
        Some(content)
    } else {
        None
    }
}

fn unescape_string(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('r') => result.push('\r'),
                Some('\\') => result.push('\\'),
                Some('"') => result.push('"'),
                Some('\'') => result.push('\''),
                Some('0') => result.push('\0'),
                Some('a') => result.push('\x07'), // bell
                Some('b') => result.push('\x08'), // backspace
                Some('f') => result.push('\x0C'), // form feed
                Some('v') => result.push('\x0B'), // vertical tab
                
                // \xHH
                Some('x') => {
                    let mut hex = String::new();
                    if let Some(&h1) = chars.peek() {
                        if h1.is_ascii_hexdigit() {
                            hex.push(chars.next().unwrap());
                            if let Some(&h2) = chars.peek() {
                                if h2.is_ascii_hexdigit() {
                                    hex.push(chars.next().unwrap());
                                }
                            }
                        }
                    }
                    if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                        result.push(byte as char);
                    } else {
                        result.push('\\');
                        result.push('x');
                        result.push_str(&hex);
                    }
                }
                
                // \u{XXXX}
                Some('u') => {
                    if chars.peek() == Some(&'{') {
                        chars.next(); // consume '{'
                        let mut hex = String::new();
                        
                        while let Some(&ch) = chars.peek() {
                            if ch == '}' {
                                chars.next();
                                break;
                            }
                            if ch.is_ascii_hexdigit() {
                                hex.push(chars.next().unwrap());
                            } else {
                                break;
                            }
                        }
                        
                        if let Ok(code) = u32::from_str_radix(&hex, 16) {
                            if let Some(unicode_char) = char::from_u32(code) {
                                result.push(unicode_char);
                            }
                        }
                    } else {
                        result.push('\\');
                        result.push('u');
                    }
                }
                
                // \z
                Some('z') => {
                    while let Some(&ch) = chars.peek() {
                        if ch.is_whitespace() {
                            chars.next();
                        } else {
                            break;
                        }
                    }
                }
                
                // \ddd
                Some(d) if d.is_ascii_digit() => {
                    let mut num = String::new();
                    num.push(d);
                    
                    for _ in 0..2 {
                        if let Some(&next) = chars.peek() {
                            if next.is_ascii_digit() {
                                num.push(chars.next().unwrap());
                            } else {
                                break;
                            }
                        }
                    }
                    
                    if let Ok(byte) = num.parse::<u8>() {
                        result.push(byte as char);
                    } else {
                        result.push('\\');
                        result.push_str(&num);
                    }
                }
                
                Some(c) => {
                    result.push('\\');
                    result.push(c);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(ch);
        }
    }
    
    result
}

pub fn lex(source: &str) -> Result<Vec<(Token, Span)>, LexError> {
    let mut tokens = Vec::new();
    let mut lexer = Token::lexer(source);
    
    while let Some(token_result) = lexer.next() {
        let span = lexer.span();
        match token_result {
            Ok(token) => {
                if matches!(token, Token::Shebang) {
                    if span.start == 0 {
                        continue;
                    } else {
                        return Err(LexError::InvalidShebang { span }); // shebang not at start
                    }
                }
                
                if let Token::Number(ref num) = token {
                    if !validate_number(num) {
                        return Err(LexError::InvalidNumber { span });
                    }
                }
                tokens.push((token, span));
            }
            Err(_) => {
                return Err(LexError::InvalidNumber { span });
            }
        }
    }
    
    let eof_pos = source.len();
    tokens.push((Token::Eof, eof_pos..eof_pos));
    
    Ok(tokens)
}

fn validate_number(s: &str) -> bool {
    if s.starts_with("0x") || s.starts_with("0X") {
        // HEX
        // has to have atleast one digit after 0x
        let after_prefix = &s[2..];
        if after_prefix.is_empty() {
            return false;
        }
        
        // check for valid hex with the optional p exponent
        let parts: Vec<&str> = after_prefix.split(|c| c == 'p' || c == 'P').collect();
        if parts.len() > 2 {
            return false;
        }
        
        // the first part must be valid hex (with an optional .)
        let hex_part = parts[0].replace('_', "");
        if !hex_part.chars().all(|c| c.is_ascii_hexdigit() || c == '.') {
            return false;
        }
        
        // if we encounter an exponent, then we validate it
        if parts.len() == 2 {
            let exp = parts[1].replace('_', "");
            let exp = exp.trim_start_matches('+').trim_start_matches('-');
            if exp.is_empty() || !exp.chars().all(|c| c.is_ascii_digit()) {
                return false;
            }
        }
    } else if s.starts_with("0b") || s.starts_with("0B") {
        // BINARY
        // has to have atleast one digit
        let after_prefix = &s[2..].replace('_', "");
        if after_prefix.is_empty() || !after_prefix.chars().all(|c| c == '0' || c == '1') {
            return false;
        }
    } else {
        // DECIMAL
        let cleaned = s.replace('_', "");
        
        // has to have at least one digit somewhere
        if !cleaned.chars().any(|c| c.is_ascii_digit()) {
            return false;
        }
        
        if cleaned.contains('e') || cleaned.contains('E') {
            let parts: Vec<&str> = cleaned.split(|c| c == 'e' || c == 'E').collect();
            if parts.len() != 2 {
                return false;
            }
            
            let exp = parts[1].trim_start_matches('+').trim_start_matches('-');
            if exp.is_empty() || !exp.chars().all(|c| c.is_ascii_digit()) {
                return false;
            }
        }
    }
    
    true
}

// Best choice..?
pub fn lex_for_version<V: crate::marker::LuaVersion>(
    source: &str,
) -> Result<Vec<(Token, Span)>, LexError> {
    let tokens = lex(source)?;

    Ok(tokens
        .into_iter()
        .map(|(token, span)| {
            let t = match token {
                Token::Continue if !V::HAS_CONTINUE => Token::Identifier("continue".to_string()),
                Token::Export if !V::HAS_EXPORT => Token::Identifier("export".to_string()),
                Token::Type if !V::HAS_TYPE_ANNOTATIONS => Token::Identifier("type".to_string()),
                Token::Goto if !V::HAS_GOTO => Token::Identifier("goto".to_string()),
                Token::Const if !V::HAS_CONST => Token::Identifier("const".to_string()),
                t => t,
            };
            (t, span)
        })
        .collect())  // <-- Add this
}
