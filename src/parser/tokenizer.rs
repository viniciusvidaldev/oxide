use crate::parser::{WriteKind, token::Token};

pub(super) struct Tokenizer<'a> {
    input: &'a str,
    cursor: usize,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub(super) fn new(input: &'a str) -> Self {
        Self {
            input,
            cursor: 0,
            chars: input.chars().peekable(),
        }
    }

    fn is_special(c: char) -> bool {
        matches!(c, ' ' | '\t' | '\n' | ';' | '<' | '>')
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn advance(&mut self) {
        if let Some(c) = self.chars.next() {
            self.cursor += c.len_utf8();
        }
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(|c| c == ' ' || c == '\t') {
            self.advance();
        }
    }

    fn read_string(&mut self) -> Token<'a> {
        let start = self.cursor;
        while self.peek().is_some_and(|c| !Self::is_special(c)) {
            self.advance();
        }
        Token::Word(&self.input[start..self.cursor])
    }

    fn read_redirect(&mut self) -> Token<'a> {
        self.advance();
        if self.peek() == Some('>') {
            self.advance();
            Token::Redirect(WriteKind::Append)
        } else {
            Token::Redirect(WriteKind::Truncate)
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        match self.peek()? {
            '>' => Some(self.read_redirect()),
            _ => {
                let start = self.cursor;
                let tok = self.read_string();
                if self.cursor == start {
                    None
                } else {
                    Some(tok)
                }
            }
        }
    }
}
