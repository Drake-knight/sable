#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Neg,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogOp {
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
    Ident(String),
    Unary(UnOp, Box<Expr>),
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Logical(LogOp, Box<Expr>, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Array(Vec<Expr>),
    Map(Vec<(Expr, Expr)>),
    Function(Vec<String>, Vec<Stmt>),
    Assign(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let(String, Option<Expr>),
    Expr(Expr),
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    Return(Option<Expr>),
    Break,
    Continue,
    Function(String, Vec<String>, Vec<Stmt>),
}
