use crate::error::{SableError, SableResult, Span};
use crate::token::{Token, TokenKind};

pub struct Lexer<'a> {
    src: &'a [u8],
    pos: usize,
    line: u32,
    col: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Lexer<'a> {
        Lexer {
            src: src.as_bytes(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn span(&self) -> Span {
        Span::new(self.line, self.col)
    }

    fn at_end(&self) -> bool {
        self.pos >= self.src.len()
    }

    fn peek(&self) -> u8 {
        if self.pos < self.src.len() {
            self.src[self.pos]
        } else {
            0
        }
    }

    fn peek_at(&self, off: usize) -> u8 {
        let p = self.pos + off;
        if p < self.src.len() {
            self.src[p]
        } else {
            0
        }
    }

    fn bump(&mut self) -> u8 {
        let c = self.peek();
        self.pos += 1;
        if c == b'\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        c
    }

    fn matches(&mut self, expected: u8) -> bool {
        if self.peek() == expected {
            self.bump();
            true
        } else {
            false
        }
    }

    fn skip_trivia(&mut self) {
        loop {
            let c = self.peek();
            match c {
                b' ' | b'\t' | b'\r' | b'\n' => {
                    self.bump();
                }
                b'#' => {
                    while !self.at_end() && self.peek() != b'\n' {
                        self.bump();
                    }
                }
                _ => break,
            }
        }
    }

    pub fn tokenize(mut self) -> SableResult<Vec<Token>> {
        let mut out = Vec::new();
        loop {
            let tok = self.next_token()?;
            let done = tok.kind == TokenKind::Eof;
            out.push(tok);
            if done {
                break;
            }
        }
        Ok(out)
    }

    fn next_token(&mut self) -> SableResult<Token> {
        self.skip_trivia();
        let span = self.span();
        if self.at_end() {
            return Ok(Token::new(TokenKind::Eof, span));
        }
        let c = self.peek();
        if is_digit(c) {
            return self.number(span);
        }
        if is_alpha(c) {
            return self.ident(span);
        }
        if c == b'"' {
            return self.string(span);
        }
        self.bump();
        let kind = match c {
            b'+' => {
                if self.matches(b'=') {
                    TokenKind::PlusEq
                } else {
                    TokenKind::Plus
                }
            }
            b'-' => {
                if self.matches(b'=') {
                    TokenKind::MinusEq
                } else {
                    TokenKind::Minus
                }
            }
            b'*' => {
                if self.matches(b'=') {
                    TokenKind::StarEq
                } else {
                    TokenKind::Star
                }
            }
            b'/' => {
                if self.matches(b'=') {
                    TokenKind::SlashEq
                } else {
                    TokenKind::Slash
                }
            }
            b'%' => {
                if self.matches(b'=') {
                    TokenKind::PercentEq
                } else {
                    TokenKind::Percent
                }
            }
            b'(' => TokenKind::LParen,
            b')' => TokenKind::RParen,
            b'{' => TokenKind::LBrace,
            b'}' => TokenKind::RBrace,
            b'[' => TokenKind::LBracket,
            b']' => TokenKind::RBracket,
            b',' => TokenKind::Comma,
            b'.' => {
                if self.matches(b'.') {
                    TokenKind::DotDot
                } else {
                    TokenKind::Dot
                }
            }
            b';' => TokenKind::Semicolon,
            b':' => TokenKind::Colon,
            b'=' => {
                if self.matches(b'=') {
                    TokenKind::Eq
                } else {
                    TokenKind::Assign
                }
            }
            b'!' => {
                if self.matches(b'=') {
                    TokenKind::Ne
                } else {
                    TokenKind::Bang
                }
            }
            b'<' => {
                if self.matches(b'=') {
                    TokenKind::Le
                } else {
                    TokenKind::Lt
                }
            }
            b'>' => {
                if self.matches(b'=') {
                    TokenKind::Ge
                } else {
                    TokenKind::Gt
                }
            }
            _ => {
                return Err(SableError::Lex(
                    format!("unexpected character '{}'", c as char),
                    span,
                ));
            }
        };
        Ok(Token::new(kind, span))
    }

    fn number(&mut self, span: Span) -> SableResult<Token> {
        let start = self.pos;
        while is_digit(self.peek()) {
            self.bump();
        }
        if self.peek() == b'.' && is_digit(self.peek_at(1)) {
            self.bump();
            while is_digit(self.peek()) {
                self.bump();
            }
        }
        if self.peek() == b'e' || self.peek() == b'E' {
            let save = self.pos;
            let save_col = self.col;
            self.bump();
            if self.peek() == b'+' || self.peek() == b'-' {
                self.bump();
            }
            if is_digit(self.peek()) {
                while is_digit(self.peek()) {
                    self.bump();
                }
            } else {
                self.pos = save;
                self.col = save_col;
            }
        }
        let text = std::str::from_utf8(&self.src[start..self.pos]).unwrap_or("");
        match text.parse::<f64>() {
            Ok(n) => Ok(Token::new(TokenKind::Number(n), span)),
            Err(_) => Err(SableError::Lex(format!("invalid number '{}'", text), span)),
        }
    }

    fn ident(&mut self, span: Span) -> SableResult<Token> {
        let start = self.pos;
        while is_alnum(self.peek()) {
            self.bump();
        }
        let text = std::str::from_utf8(&self.src[start..self.pos]).unwrap_or("");
        let kind = match TokenKind::keyword(text) {
            Some(k) => k,
            None => TokenKind::Ident(text.to_string()),
        };
        Ok(Token::new(kind, span))
    }

    fn string(&mut self, span: Span) -> SableResult<Token> {
        self.bump();
        let mut s = String::new();
        loop {
            if self.at_end() {
                return Err(SableError::Lex("unterminated string".to_string(), span));
            }
            let c = self.bump();
            if c == b'"' {
                break;
            }
            if c == b'\\' {
                let e = self.bump();
                match e {
                    b'n' => s.push('\n'),
                    b't' => s.push('\t'),
                    b'r' => s.push('\r'),
                    b'0' => s.push('\0'),
                    b'\\' => s.push('\\'),
                    b'"' => s.push('"'),
                    _ => {
                        return Err(SableError::Lex(
                            format!("invalid escape '\\{}'", e as char),
                            span,
                        ));
                    }
                }
            } else {
                s.push(c as char);
            }
        }
        Ok(Token::new(TokenKind::Str(s), span))
    }
}

fn is_digit(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}

fn is_alpha(c: u8) -> bool {
    (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z') || c == b'_'
}

fn is_alnum(c: u8) -> bool {
    is_alpha(c) || is_digit(c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenKind;

    #[test]
    fn tokenizes_basics() {
        let toks = Lexer::new("let x = 42 + y;").tokenize().unwrap();
        assert_eq!(toks[0].kind, TokenKind::Let);
        assert!(matches!(toks[1].kind, TokenKind::Ident(_)));
        assert_eq!(toks[2].kind, TokenKind::Assign);
        assert!(matches!(toks[3].kind, TokenKind::Number(_)));
        assert_eq!(toks[toks.len() - 1].kind, TokenKind::Eof);
    }

    #[test]
    fn compound_and_range_tokens() {
        let toks = Lexer::new("+= -= *= /= %= .. == != <= >=").tokenize().unwrap();
        assert_eq!(toks[0].kind, TokenKind::PlusEq);
        assert_eq!(toks[1].kind, TokenKind::MinusEq);
        assert_eq!(toks[2].kind, TokenKind::StarEq);
        assert_eq!(toks[3].kind, TokenKind::SlashEq);
        assert_eq!(toks[4].kind, TokenKind::PercentEq);
        assert_eq!(toks[5].kind, TokenKind::DotDot);
        assert_eq!(toks[6].kind, TokenKind::Eq);
        assert_eq!(toks[7].kind, TokenKind::Ne);
        assert_eq!(toks[8].kind, TokenKind::Le);
        assert_eq!(toks[9].kind, TokenKind::Ge);
    }

    #[test]
    fn strings_numbers_comments() {
        let toks = Lexer::new("\"hi\\n\" # a comment\n1.5e3").tokenize().unwrap();
        assert!(matches!(toks[0].kind, TokenKind::Str(_)));
        assert!(matches!(toks[1].kind, TokenKind::Number(_)));
    }

    #[test]
    fn unterminated_string_errors() {
        assert!(Lexer::new("\"unterminated").tokenize().is_err());
    }
}
