use core::fmt;

#[derive(Debug, Clone, Copy)]
pub enum WriteKind {
    Truncate,
    Append,
}

pub(super) enum Token<'a> {
    Word(&'a str),
    Redirect(WriteKind),
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Word(_) => write!(f, "a word"),
            Token::Redirect(WriteKind::Truncate) => write!(f, "a redirect '>'"),
            Token::Redirect(WriteKind::Append) => write!(f, "a redirect '>>'"),
        }
    }
}
