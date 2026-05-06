mod token;
mod tokenizer;

use thiserror::Error;

pub use crate::parser::token::WriteKind;
use crate::parser::{token::Token, tokenizer::Tokenizer};

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("oxide: Unexpected token {0}")]
    UnexpectedToken(String),

    #[error("oxide: Unexpected end of input")]
    UnexpectedEnd,
}

pub struct Statement {
    pub argv: Vec<String>,
    pub stdout: Option<Redirect>,
}

pub struct Redirect {
    pub path: String,
    pub kind: WriteKind,
}

type TokenStream<'a> = std::iter::Peekable<Tokenizer<'a>>;
impl Statement {
    pub fn from_buf(buf: &str) -> Result<Self, ParseError> {
        Self::parse_statement(&mut Tokenizer::new(buf).peekable())
    }

    fn parse_statement(tokens: &mut TokenStream) -> Result<Self, ParseError> {
        let first_cmd = match tokens.next() {
            Some(Token::Word(s)) => Ok(s.to_owned()),
            Some(t) => Err(ParseError::UnexpectedToken(t.to_string())),
            None => Err(ParseError::UnexpectedEnd),
        }?;

        let mut argv = vec![first_cmd];
        let mut stdout = None;

        loop {
            match tokens.peek() {
                Some(Token::Word(_)) => argv.push(Self::parse_word(tokens)?),
                Some(Token::Redirect(_)) => stdout = Some(Self::parse_redirect(tokens)?),
                None => break,
            }
        }

        Ok(Self { argv, stdout })
    }

    fn parse_word(tokens: &mut TokenStream) -> Result<String, ParseError> {
        match tokens.next() {
            Some(Token::Word(s)) => Ok(s.to_owned()),
            Some(t) => Err(ParseError::UnexpectedToken(t.to_string())),
            None => Err(ParseError::UnexpectedEnd),
        }
    }

    fn parse_redirect(tokens: &mut TokenStream) -> Result<Redirect, ParseError> {
        let kind = match tokens.next() {
            Some(Token::Redirect(kind)) => kind,
            Some(t) => return Err(ParseError::UnexpectedToken(t.to_string())),
            None => return Err(ParseError::UnexpectedEnd),
        };

        let path = Self::parse_word(tokens)?;
        Ok(Redirect { path, kind })
    }
}
