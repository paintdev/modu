use bat::input;
use logos::{Logos, Span};

#[derive(Default, Debug, PartialEq, Clone)]
pub enum LexingError {
    #[default]
    UnexpectedToken,
    InvalidInteger(String)
}

impl From<std::num::ParseIntError> for LexingError {
    fn from(err: std::num::ParseIntError) -> Self {
        use std::num::IntErrorKind;

        match err.kind() {
            IntErrorKind::PosOverflow | IntErrorKind::NegOverflow => {
                LexingError::InvalidInteger("Integer literal out of range".to_string())
            }
            _ => LexingError::InvalidInteger("Invalid integer literal".to_string()),
        }
    }
}

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(error = LexingError)]
pub enum Token {
    #[token("let")]
    Let,

    #[token("=")]
    Equals,

    #[token(";")]
    Semicolon,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[regex("[0-9]+", |lex| lex.slice().parse::<i64>())]
    Int(i64),

    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexingError> {
    let mut lexer = Token::lexer(input);
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Err(e) => return Err(e),

            Ok(v) => {
                tokens.push(v);
            }
        }
    }

    Ok(tokens)
}