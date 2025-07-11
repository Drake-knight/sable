use crate::error::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Number(f64),
    Str(String),
    Ident(String),
    Let,
    Fn,
    If,
    Else,
    While,
    For,
    In,
    Return,
    Break,
    Continue,
    True,
    False,
    Nil,
    And,
    Or,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Assign,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Bang,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Dot,
    DotDot,
    Semicolon,
    Colon,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    Eof,
}

impl TokenKind {
    pub fn keyword(s: &str) -> Option<TokenKind> {
        let k = match s {
            "let" => TokenKind::Let,
            "fn" => TokenKind::Fn,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "in" => TokenKind::In,
            "return" => TokenKind::Return,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "nil" => TokenKind::Nil,
            "and" => TokenKind::And,
            "or" => TokenKind::Or,
            _ => return None,
        };
        Some(k)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Token {
        Token { kind, span }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyword_lookup() {
        assert_eq!(TokenKind::keyword("let"), Some(TokenKind::Let));
        assert_eq!(TokenKind::keyword("fn"), Some(TokenKind::Fn));
        assert_eq!(TokenKind::keyword("while"), Some(TokenKind::While));
        assert_eq!(TokenKind::keyword("return"), Some(TokenKind::Return));
        assert_eq!(TokenKind::keyword("nil"), Some(TokenKind::Nil));
        assert_eq!(TokenKind::keyword("notakeyword"), None);
        assert_eq!(TokenKind::keyword(""), None);
    }
}
