use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub line: u32,
    pub col: u32,
}

impl Span {
    pub fn new(line: u32, col: u32) -> Span {
        Span { line, col }
    }

    pub fn zero() -> Span {
        Span { line: 0, col: 0 }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SableError {
    Lex(String, Span),
    Parse(String, Span),
    Compile(String, Span),
    Load(String),
    Type(String),
    Arity(String),
    Runtime(String),
}

impl SableError {
    pub fn kind_str(&self) -> &'static str {
        match self {
            SableError::Lex(_, _) => "lex",
            SableError::Parse(_, _) => "parse",
            SableError::Compile(_, _) => "compile",
            SableError::Load(_) => "load",
            SableError::Type(_) => "type",
            SableError::Arity(_) => "arity",
            SableError::Runtime(_) => "runtime",
        }
    }

    pub fn message(&self) -> &str {
        match self {
            SableError::Lex(m, _) => m,
            SableError::Parse(m, _) => m,
            SableError::Compile(m, _) => m,
            SableError::Load(m) => m,
            SableError::Type(m) => m,
            SableError::Arity(m) => m,
            SableError::Runtime(m) => m,
        }
    }
}

impl fmt::Display for SableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SableError::Lex(m, s) => write!(f, "lex error at {}: {}", s, m),
            SableError::Parse(m, s) => write!(f, "parse error at {}: {}", s, m),
            SableError::Compile(m, s) => write!(f, "compile error at {}: {}", s, m),
            SableError::Load(m) => write!(f, "load error: {}", m),
            SableError::Type(m) => write!(f, "type error: {}", m),
            SableError::Arity(m) => write!(f, "arity error: {}", m),
            SableError::Runtime(m) => write!(f, "runtime error: {}", m),
        }
    }
}

impl std::error::Error for SableError {}

pub type SableResult<T> = Result<T, SableError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind_message_display() {
        let e = SableError::Type("bad operand".to_string());
        assert_eq!(e.kind_str(), "type");
        assert_eq!(e.message(), "bad operand");
        assert!(format!("{}", e).contains("bad operand"));

        let l = SableError::Lex("oops".to_string(), Span::new(2, 3));
        assert_eq!(l.kind_str(), "lex");
        assert!(format!("{}", l).contains("2:3"));

        let r = SableError::Runtime("boom".to_string());
        assert_eq!(r.kind_str(), "runtime");
        assert!(format!("{}", r).contains("boom"));
    }

    #[test]
    fn span_display() {
        assert_eq!(format!("{}", Span::new(7, 12)), "7:12");
        assert_eq!(format!("{}", Span::zero()), "0:0");
    }
}
