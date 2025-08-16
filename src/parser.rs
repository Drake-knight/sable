use crate::ast::{BinOp, Expr, LogOp, Stmt, UnOp};
use crate::error::{SableError, SableResult, Span};
use crate::token::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    gensym: u32,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            pos: 0,
            gensym: 0,
        }
    }

    pub fn parse_program(mut self) -> SableResult<Vec<Stmt>> {
        let mut stmts = Vec::new();
        while !self.check(&TokenKind::Eof) {
            stmts.push(self.declaration()?);
        }
        Ok(stmts)
    }

    fn peek_kind(&self) -> &TokenKind {
        &self.tokens[self.pos].kind
    }

    fn peek2_kind(&self) -> &TokenKind {
        let p = self.pos + 1;
        if p < self.tokens.len() {
            &self.tokens[p].kind
        } else {
            &self.tokens[self.tokens.len() - 1].kind
        }
    }

    fn span(&self) -> Span {
        self.tokens[self.pos].span
    }

    fn advance(&mut self) -> Token {
        let t = self.tokens[self.pos].clone();
        if self.pos + 1 < self.tokens.len() {
            self.pos += 1;
        }
        t
    }

    fn check(&self, k: &TokenKind) -> bool {
        kind_eq(self.peek_kind(), k)
    }

    fn matches(&mut self, k: &TokenKind) -> bool {
        if self.check(k) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, k: &TokenKind, msg: &str) -> SableResult<Token> {
        if self.check(k) {
            Ok(self.advance())
        } else {
            Err(SableError::Parse(msg.to_string(), self.span()))
        }
    }

    fn ident_name(&mut self, msg: &str) -> SableResult<String> {
        if let TokenKind::Ident(s) = self.peek_kind().clone() {
            self.advance();
            Ok(s)
        } else {
            Err(SableError::Parse(msg.to_string(), self.span()))
        }
    }

    fn declaration(&mut self) -> SableResult<Stmt> {
        match self.peek_kind() {
            TokenKind::Let => self.let_decl(),
            TokenKind::Fn => {
                if matches!(self.peek2_kind(), TokenKind::Ident(_)) {
                    self.fn_decl()
                } else {
                    self.statement()
                }
            }
            _ => self.statement(),
        }
    }

    fn let_decl(&mut self) -> SableResult<Stmt> {
        self.advance();
        let name = self.ident_name("expected variable name after let")?;
        let init = if self.matches(&TokenKind::Assign) {
            Some(self.expression()?)
        } else {
            None
        };
        self.expect(&TokenKind::Semicolon, "expected ';' after let")?;
        Ok(Stmt::Let(name, init))
    }

    fn fn_decl(&mut self) -> SableResult<Stmt> {
        self.advance();
        let name = self.ident_name("expected function name")?;
        self.expect(&TokenKind::LParen, "expected '(' after function name")?;
        let params = self.params()?;
        self.expect(&TokenKind::LBrace, "expected '{' before function body")?;
        let body = self.block_body()?;
        Ok(Stmt::Function(name, params, body))
    }

    fn params(&mut self) -> SableResult<Vec<String>> {
        let mut params = Vec::new();
        if !self.check(&TokenKind::RParen) {
            loop {
                let p = self.ident_name("expected parameter name")?;
                params.push(p);
                if !self.matches(&TokenKind::Comma) {
                    break;
                }
            }
        }
        self.expect(&TokenKind::RParen, "expected ')' after parameters")?;
        Ok(params)
    }

    fn block_body(&mut self) -> SableResult<Vec<Stmt>> {
        let mut stmts = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.check(&TokenKind::Eof) {
            stmts.push(self.declaration()?);
        }
        self.expect(&TokenKind::RBrace, "expected '}'")?;
        Ok(stmts)
    }

    fn statement(&mut self) -> SableResult<Stmt> {
        match self.peek_kind() {
            TokenKind::If => self.if_stmt(),
            TokenKind::While => self.while_stmt(),
            TokenKind::For => self.for_stmt(),
            TokenKind::Return => self.return_stmt(),
            TokenKind::Break => {
                self.advance();
                self.expect(&TokenKind::Semicolon, "expected ';' after break")?;
                Ok(Stmt::Break)
            }
            TokenKind::Continue => {
                self.advance();
                self.expect(&TokenKind::Semicolon, "expected ';' after continue")?;
                Ok(Stmt::Continue)
            }
            TokenKind::LBrace => {
                self.advance();
                let body = self.block_body()?;
                Ok(Stmt::Block(body))
            }
            _ => self.expr_stmt(),
        }
    }

    fn if_stmt(&mut self) -> SableResult<Stmt> {
        self.advance();
        let cond = self.expression()?;
        self.expect(&TokenKind::LBrace, "expected '{' after if condition")?;
        let then_body = self.block_body()?;
        let then_branch = Box::new(Stmt::Block(then_body));
        let else_branch = if self.matches(&TokenKind::Else) {
            if self.check(&TokenKind::If) {
                Some(Box::new(self.if_stmt()?))
            } else {
                self.expect(&TokenKind::LBrace, "expected '{' after else")?;
                let eb = self.block_body()?;
                Some(Box::new(Stmt::Block(eb)))
            }
        } else {
            None
        };
        Ok(Stmt::If(cond, then_branch, else_branch))
    }

    fn while_stmt(&mut self) -> SableResult<Stmt> {
        self.advance();
        let cond = self.expression()?;
        self.expect(&TokenKind::LBrace, "expected '{' after while condition")?;
        let body = self.block_body()?;
        Ok(Stmt::While(cond, Box::new(Stmt::Block(body))))
    }

    fn return_stmt(&mut self) -> SableResult<Stmt> {
        self.advance();
        let value = if self.check(&TokenKind::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };
        self.expect(&TokenKind::Semicolon, "expected ';' after return")?;
        Ok(Stmt::Return(value))
    }

    fn expr_stmt(&mut self) -> SableResult<Stmt> {
        let e = self.expression()?;
        self.expect(&TokenKind::Semicolon, "expected ';' after expression")?;
        Ok(Stmt::Expr(e))
    }

    fn expression(&mut self) -> SableResult<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> SableResult<Expr> {
        let expr = self.or()?;
        if self.check(&TokenKind::Assign) {
            let span = self.span();
            self.advance();
            let value = self.assignment()?;
            return self.make_assign(expr, value, span);
        }
        if let Some(op) = self.compound_op() {
            let span = self.span();
            self.advance();
            let value = self.assignment()?;
            let combined = Expr::Binary(op, Box::new(expr.clone()), Box::new(value));
            return self.make_assign(expr, combined, span);
        }
        Ok(expr)
    }

    fn compound_op(&self) -> Option<BinOp> {
        match self.peek_kind() {
            TokenKind::PlusEq => Some(BinOp::Add),
            TokenKind::MinusEq => Some(BinOp::Sub),
            TokenKind::StarEq => Some(BinOp::Mul),
            TokenKind::SlashEq => Some(BinOp::Div),
            TokenKind::PercentEq => Some(BinOp::Mod),
            _ => None,
        }
    }

    fn make_assign(&self, target: Expr, value: Expr, span: Span) -> SableResult<Expr> {
        match &target {
            Expr::Ident(_) | Expr::Index(_, _) => {
                Ok(Expr::Assign(Box::new(target), Box::new(value)))
            }
            _ => Err(SableError::Parse(
                "invalid assignment target".to_string(),
                span,
            )),
        }
    }

    fn fresh(&mut self, tag: &str) -> String {
        let n = self.gensym;
        self.gensym += 1;
        format!("__{}_{}", tag, n)
    }

    fn for_stmt(&mut self) -> SableResult<Stmt> {
        self.advance();
        let var = self.ident_name("expected loop variable after 'for'")?;
        self.expect(&TokenKind::In, "expected 'in' after loop variable")?;
        let start = self.expression()?;
        if self.matches(&TokenKind::DotDot) {
            let end = self.expression()?;
            self.expect(&TokenKind::LBrace, "expected '{' before loop body")?;
            let body = self.block_body()?;
            Ok(self.desugar_range_for(var, start, end, body))
        } else {
            self.expect(&TokenKind::LBrace, "expected '{' before loop body")?;
            let body = self.block_body()?;
            Ok(self.desugar_array_for(var, start, body))
        }
    }

    fn desugar_array_for(&mut self, var: String, iter: Expr, body: Vec<Stmt>) -> Stmt {
        let it = self.fresh("it");
        let ix = self.fresh("ix");
        let nn = self.fresh("n");
        let mut inner: Vec<Stmt> = Vec::new();
        inner.push(Stmt::Let(
            var,
            Some(Expr::Index(
                Box::new(Expr::Ident(it.clone())),
                Box::new(Expr::Ident(ix.clone())),
            )),
        ));
        inner.push(Stmt::Expr(Expr::Assign(
            Box::new(Expr::Ident(ix.clone())),
            Box::new(Expr::Binary(
                BinOp::Add,
                Box::new(Expr::Ident(ix.clone())),
                Box::new(Expr::Number(1.0)),
            )),
        )));
        for s in body {
            inner.push(s);
        }
        let cond = Expr::Binary(
            BinOp::Lt,
            Box::new(Expr::Ident(ix.clone())),
            Box::new(Expr::Ident(nn.clone())),
        );
        Stmt::Block(vec![
            Stmt::Let(it.clone(), Some(iter)),
            Stmt::Let(ix, Some(Expr::Number(0.0))),
            Stmt::Let(
                nn,
                Some(Expr::Call(
                    Box::new(Expr::Ident("len".to_string())),
                    vec![Expr::Ident(it)],
                )),
            ),
            Stmt::While(cond, Box::new(Stmt::Block(inner))),
        ])
    }

    fn desugar_range_for(&mut self, var: String, start: Expr, end: Expr, body: Vec<Stmt>) -> Stmt {
        let ix = self.fresh("ix");
        let en = self.fresh("end");
        let mut inner: Vec<Stmt> = Vec::new();
        inner.push(Stmt::Let(var, Some(Expr::Ident(ix.clone()))));
        inner.push(Stmt::Expr(Expr::Assign(
            Box::new(Expr::Ident(ix.clone())),
            Box::new(Expr::Binary(
                BinOp::Add,
                Box::new(Expr::Ident(ix.clone())),
                Box::new(Expr::Number(1.0)),
            )),
        )));
        for s in body {
            inner.push(s);
        }
        let cond = Expr::Binary(
            BinOp::Lt,
            Box::new(Expr::Ident(ix.clone())),
            Box::new(Expr::Ident(en.clone())),
        );
        Stmt::Block(vec![
            Stmt::Let(ix, Some(start)),
            Stmt::Let(en, Some(end)),
            Stmt::While(cond, Box::new(Stmt::Block(inner))),
        ])
    }

    fn or(&mut self) -> SableResult<Expr> {
        let mut expr = self.and()?;
        while self.check(&TokenKind::Or) {
            self.advance();
            let right = self.and()?;
            expr = Expr::Logical(LogOp::Or, Box::new(expr), Box::new(right));
        }
        Ok(expr)
    }

    fn and(&mut self) -> SableResult<Expr> {
        let mut expr = self.equality()?;
        while self.check(&TokenKind::And) {
            self.advance();
            let right = self.equality()?;
            expr = Expr::Logical(LogOp::And, Box::new(expr), Box::new(right));
        }
        Ok(expr)
    }

    fn equality(&mut self) -> SableResult<Expr> {
        let mut expr = self.comparison()?;
        loop {
            let op = match self.peek_kind() {
                TokenKind::Eq => BinOp::Eq,
                TokenKind::Ne => BinOp::Ne,
                _ => break,
            };
            self.advance();
            let right = self.comparison()?;
            expr = Expr::Binary(op, Box::new(expr), Box::new(right));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> SableResult<Expr> {
        let mut expr = self.term()?;
        loop {
            let op = match self.peek_kind() {
                TokenKind::Lt => BinOp::Lt,
                TokenKind::Le => BinOp::Le,
                TokenKind::Gt => BinOp::Gt,
                TokenKind::Ge => BinOp::Ge,
                _ => break,
            };
            self.advance();
            let right = self.term()?;
            expr = Expr::Binary(op, Box::new(expr), Box::new(right));
        }
        Ok(expr)
    }

    fn term(&mut self) -> SableResult<Expr> {
        let mut expr = self.factor()?;
        loop {
            let op = match self.peek_kind() {
                TokenKind::Plus => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.factor()?;
            expr = Expr::Binary(op, Box::new(expr), Box::new(right));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> SableResult<Expr> {
        let mut expr = self.unary()?;
        loop {
            let op = match self.peek_kind() {
                TokenKind::Star => BinOp::Mul,
                TokenKind::Slash => BinOp::Div,
                TokenKind::Percent => BinOp::Mod,
                _ => break,
            };
            self.advance();
            let right = self.unary()?;
            expr = Expr::Binary(op, Box::new(expr), Box::new(right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> SableResult<Expr> {
        match self.peek_kind() {
            TokenKind::Bang => {
                self.advance();
                let e = self.unary()?;
                Ok(Expr::Unary(UnOp::Not, Box::new(e)))
            }
            TokenKind::Minus => {
                self.advance();
                let e = self.unary()?;
                Ok(Expr::Unary(UnOp::Neg, Box::new(e)))
            }
            _ => self.call(),
        }
    }

    fn call(&mut self) -> SableResult<Expr> {
        let mut expr = self.primary()?;
        loop {
            match self.peek_kind() {
                TokenKind::LParen => {
                    self.advance();
                    let args = self.args()?;
                    expr = Expr::Call(Box::new(expr), args);
                }
                TokenKind::LBracket => {
                    self.advance();
                    let idx = self.expression()?;
                    self.expect(&TokenKind::RBracket, "expected ']' after index")?;
                    expr = Expr::Index(Box::new(expr), Box::new(idx));
                }
                TokenKind::Dot => {
                    self.advance();
                    let name = self.ident_name("expected field name after '.'")?;
                    expr = Expr::Index(Box::new(expr), Box::new(Expr::Str(name)));
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn args(&mut self) -> SableResult<Vec<Expr>> {
        let mut args = Vec::new();
        if !self.check(&TokenKind::RParen) {
            loop {
                args.push(self.expression()?);
                if !self.matches(&TokenKind::Comma) {
                    break;
                }
            }
        }
        self.expect(&TokenKind::RParen, "expected ')' after arguments")?;
        Ok(args)
    }

    fn primary(&mut self) -> SableResult<Expr> {
        let span = self.span();
        match self.peek_kind().clone() {
            TokenKind::Number(n) => {
                self.advance();
                Ok(Expr::Number(n))
            }
            TokenKind::Str(s) => {
                self.advance();
                Ok(Expr::Str(s))
            }
            TokenKind::True => {
                self.advance();
                Ok(Expr::Bool(true))
            }
            TokenKind::False => {
                self.advance();
                Ok(Expr::Bool(false))
            }
            TokenKind::Nil => {
                self.advance();
                Ok(Expr::Nil)
            }
            TokenKind::Ident(name) => {
                self.advance();
                Ok(Expr::Ident(name))
            }
            TokenKind::LParen => {
                self.advance();
                let e = self.expression()?;
                self.expect(&TokenKind::RParen, "expected ')'")?;
                Ok(e)
            }
            TokenKind::LBracket => {
                self.advance();
                let mut items = Vec::new();
                if !self.check(&TokenKind::RBracket) {
                    loop {
                        items.push(self.expression()?);
                        if !self.matches(&TokenKind::Comma) {
                            break;
                        }
                    }
                }
                self.expect(&TokenKind::RBracket, "expected ']' after array")?;
                Ok(Expr::Array(items))
            }
            TokenKind::Fn => {
                self.advance();
                self.expect(&TokenKind::LParen, "expected '(' in function expression")?;
                let params = self.params()?;
                self.expect(&TokenKind::LBrace, "expected '{' before function body")?;
                let body = self.block_body()?;
                Ok(Expr::Function(params, body))
            }
            _ => Err(SableError::Parse("expected expression".to_string(), span)),
        }
    }
}

fn kind_eq(a: &TokenKind, b: &TokenKind) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse(src: &str) -> SableResult<Vec<Stmt>> {
        let toks = Lexer::new(src).tokenize()?;
        Parser::new(toks).parse_program()
    }

    #[test]
    fn parses_valid_programs() {
        assert!(parse("let x = 1; return x + 2;").is_ok());
        assert!(parse("fn f(a, b) { return a * b; }").is_ok());
        assert!(parse("for i in 0..10 { let y = i; }").is_ok());
        assert!(parse("if a { b = 1; } else { b = 2; }").is_ok());
        assert!(parse("while x < 10 { x += 1; }").is_ok());
        assert!(parse("let a = [1, 2, 3]; a[0] = 9;").is_ok());
    }

    #[test]
    fn rejects_invalid_programs() {
        assert!(parse("let = 5;").is_err());
        assert!(parse("return (1 + ;").is_err());
        assert!(parse("1 = 2;").is_err());
        assert!(parse("let x 5;").is_err());
    }

    #[test]
    fn statement_count() {
        let prog = parse("let a = 1; let b = 2; return a + b;").unwrap();
        assert_eq!(prog.len(), 3);
    }
}
