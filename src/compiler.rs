use crate::ast::{BinOp, Expr, LogOp, Stmt, UnOp};
use crate::chunk::{CompiledProto, Konst, UpDesc};
use crate::error::{SableError, SableResult, Span};
use crate::opcode::Op;

struct Local {
    name: String,
    depth: i32,
    is_captured: bool,
}

struct LoopCtx {
    start: usize,
    breaks: Vec<usize>,
    base_locals: usize,
}

struct FnCtx {
    code: Vec<u8>,
    consts: Vec<Konst>,
    protos: Vec<CompiledProto>,
    locals: Vec<Local>,
    upvalues: Vec<UpDesc>,
    scope_depth: i32,
    arity: u8,
    name: String,
    loops: Vec<LoopCtx>,
}

impl FnCtx {
    fn new(name: String, arity: u8) -> FnCtx {
        FnCtx {
            code: Vec::new(),
            consts: Vec::new(),
            protos: Vec::new(),
            locals: Vec::new(),
            upvalues: Vec::new(),
            scope_depth: 0,
            arity,
            name,
            loops: Vec::new(),
        }
    }
}

pub struct Compiler {
    stack: Vec<FnCtx>,
}

pub fn compile(program: &[Stmt]) -> SableResult<CompiledProto> {
    let mut c = Compiler { stack: Vec::new() };
    c.stack.push(FnCtx::new("main".to_string(), 0));
    c.stack[0].locals.push(Local {
        name: String::new(),
        depth: 0,
        is_captured: false,
    });
    for stmt in program {
        c.stmt(stmt)?;
    }
    c.emit(Op::LoadNil);
    c.emit(Op::Return);
    let ctx = c.stack.pop().unwrap();
    Ok(finish(ctx))
}

fn finish(ctx: FnCtx) -> CompiledProto {
    CompiledProto {
        arity: ctx.arity,
        upvals: ctx.upvalues.len() as u8,
        max_stack: 250,
        code: ctx.code,
        consts: ctx.consts,
        protos: ctx.protos,
        upinfo: ctx.upvalues,
        name: ctx.name,
    }
}

fn konst_eq(a: &Konst, b: &Konst) -> bool {
    match (a, b) {
        (Konst::Num(x), Konst::Num(y)) => x.to_bits() == y.to_bits(),
        (Konst::Str(x), Konst::Str(y)) => x == y,
        (Konst::Bool(x), Konst::Bool(y)) => x == y,
        (Konst::Nil, Konst::Nil) => true,
        _ => false,
    }
}

impl Compiler {
    fn cur(&mut self) -> &mut FnCtx {
        self.stack.last_mut().unwrap()
    }

    fn emit(&mut self, op: Op) {
        self.cur().code.push(op as u8);
    }

    fn emit_u8(&mut self, b: u8) {
        self.cur().code.push(b);
    }

    fn emit_u16(&mut self, v: u16) {
        let code = &mut self.cur().code;
        code.push((v & 0xff) as u8);
        code.push((v >> 8) as u8);
    }

    fn emit_op_u8(&mut self, op: Op, b: u8) {
        self.emit(op);
        self.emit_u8(b);
    }

    fn emit_op_u16(&mut self, op: Op, v: u16) {
        self.emit(op);
        self.emit_u16(v);
    }

    fn add_const(&mut self, k: Konst) -> SableResult<u16> {
        let consts = &mut self.cur().consts;
        for (i, existing) in consts.iter().enumerate() {
            if konst_eq(existing, &k) {
                return Ok(i as u16);
            }
        }
        if consts.len() >= 65535 {
            return Err(SableError::Compile(
                "too many constants".to_string(),
                Span::zero(),
            ));
        }
        consts.push(k);
        Ok((consts.len() - 1) as u16)
    }

    fn emit_const(&mut self, k: Konst) -> SableResult<()> {
        let idx = self.add_const(k)?;
        self.emit_op_u16(Op::LoadConst, idx);
        Ok(())
    }

    fn jump(&mut self, op: Op) -> usize {
        self.emit(op);
        self.emit_u8(0xff);
        self.emit_u8(0xff);
        self.cur().code.len() - 2
    }

    fn patch(&mut self, offset: usize) -> SableResult<()> {
        let code_len = self.cur().code.len();
        let dist = code_len - offset - 2;
        if dist > 0xffff {
            return Err(SableError::Compile(
                "jump too large".to_string(),
                Span::zero(),
            ));
        }
        let code = &mut self.cur().code;
        code[offset] = (dist & 0xff) as u8;
        code[offset + 1] = (dist >> 8) as u8;
        Ok(())
    }

    fn emit_loop(&mut self, start: usize) -> SableResult<()> {
        self.emit(Op::Loop);
        let code_len = self.cur().code.len();
        let dist = code_len + 2 - start;
        if dist > 0xffff {
            return Err(SableError::Compile(
                "loop too large".to_string(),
                Span::zero(),
            ));
        }
        self.emit_u16(dist as u16);
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.cur().scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.cur().scope_depth -= 1;
        loop {
            let ctx = self.cur();
            let should_pop = match ctx.locals.last() {
                Some(l) => l.depth > ctx.scope_depth,
                None => false,
            };
            if !should_pop {
                break;
            }
            let captured = ctx.locals.last().unwrap().is_captured;
            if captured {
                ctx.code.push(Op::CloseUpvalue as u8);
            } else {
                ctx.code.push(Op::Pop as u8);
            }
            ctx.locals.pop();
        }
    }

    fn emit_pops_to(&mut self, base: usize) {
        let ctx = self.cur();
        let mut i = ctx.locals.len();
        while i > base {
            i -= 1;
            if ctx.locals[i].is_captured {
                ctx.code.push(Op::CloseUpvalue as u8);
            } else {
                ctx.code.push(Op::Pop as u8);
            }
        }
    }

    fn declare_local(&mut self, name: &str) -> SableResult<()> {
        let ctx = self.cur();
        if ctx.locals.len() >= 250 {
            return Err(SableError::Compile(
                "too many locals".to_string(),
                Span::zero(),
            ));
        }
        ctx.locals.push(Local {
            name: name.to_string(),
            depth: -1,
            is_captured: false,
        });
        Ok(())
    }

    fn mark_initialized(&mut self) {
        let ctx = self.cur();
        let d = ctx.scope_depth;
        if let Some(l) = ctx.locals.last_mut() {
            l.depth = d;
        }
    }

    fn resolve_local(&self, ci: usize, name: &str) -> SableResult<Option<usize>> {
        let ctx = &self.stack[ci];
        for i in (0..ctx.locals.len()).rev() {
            if ctx.locals[i].name == name {
                if ctx.locals[i].depth == -1 {
                    return Err(SableError::Compile(
                        "cannot read variable in its own initializer".to_string(),
                        Span::zero(),
                    ));
                }
                return Ok(Some(i));
            }
        }
        Ok(None)
    }

    fn add_upvalue(&mut self, ci: usize, index: u8, is_local: bool) -> SableResult<u8> {
        let ctx = &mut self.stack[ci];
        for (i, u) in ctx.upvalues.iter().enumerate() {
            if u.index == index && u.is_local == is_local {
                return Ok(i as u8);
            }
        }
        if ctx.upvalues.len() >= 250 {
            return Err(SableError::Compile(
                "too many upvalues".to_string(),
                Span::zero(),
            ));
        }
        ctx.upvalues.push(UpDesc { is_local, index });
        Ok((ctx.upvalues.len() - 1) as u8)
    }

    fn resolve_upvalue(&mut self, ci: usize, name: &str) -> SableResult<Option<u8>> {
        if ci == 0 {
            return Ok(None);
        }
        let enclosing = ci - 1;
        if let Some(local) = self.resolve_local(enclosing, name)? {
            self.stack[enclosing].locals[local].is_captured = true;
            let up = self.add_upvalue(ci, local as u8, true)?;
            return Ok(Some(up));
        }
        if let Some(upv) = self.resolve_upvalue(enclosing, name)? {
            let up = self.add_upvalue(ci, upv, false)?;
            return Ok(Some(up));
        }
        Ok(None)
    }

    fn get_variable(&mut self, name: &str) -> SableResult<()> {
        let ci = self.stack.len() - 1;
        if let Some(slot) = self.resolve_local(ci, name)? {
            self.emit_op_u8(Op::GetLocal, slot as u8);
        } else if let Some(up) = self.resolve_upvalue(ci, name)? {
            self.emit_op_u8(Op::GetUpvalue, up);
        } else {
            let idx = self.add_const(Konst::Str(name.to_string()))?;
            self.emit_op_u16(Op::GetGlobal, idx);
        }
        Ok(())
    }

    fn set_variable(&mut self, name: &str) -> SableResult<()> {
        let ci = self.stack.len() - 1;
        if let Some(slot) = self.resolve_local(ci, name)? {
            self.emit_op_u8(Op::SetLocal, slot as u8);
        } else if let Some(up) = self.resolve_upvalue(ci, name)? {
            self.emit_op_u8(Op::SetUpvalue, up);
        } else {
            let idx = self.add_const(Konst::Str(name.to_string()))?;
            self.emit_op_u16(Op::SetGlobal, idx);
        }
        Ok(())
    }

    fn stmt(&mut self, s: &Stmt) -> SableResult<()> {
        match s {
            Stmt::Let(name, init) => self.let_stmt(name, init),
            Stmt::Expr(e) => {
                self.expr(e)?;
                self.emit(Op::Pop);
                Ok(())
            }
            Stmt::Block(body) => {
                self.begin_scope();
                for st in body {
                    self.stmt(st)?;
                }
                self.end_scope();
                Ok(())
            }
            Stmt::If(cond, then_b, else_b) => self.if_stmt(cond, then_b, else_b),
            Stmt::While(cond, body) => self.while_stmt(cond, body),
            Stmt::Return(val) => self.return_stmt(val),
            Stmt::Break => self.break_stmt(),
            Stmt::Continue => self.continue_stmt(),
            Stmt::Function(name, params, body) => self.fn_stmt(name, params, body),
        }
    }

    fn let_stmt(&mut self, name: &str, init: &Option<Expr>) -> SableResult<()> {
        if self.cur().scope_depth == 0 {
            match init {
                Some(e) => self.expr(e)?,
                None => self.emit(Op::LoadNil),
            }
            let idx = self.add_const(Konst::Str(name.to_string()))?;
            self.emit_op_u16(Op::DefineGlobal, idx);
        } else {
            self.declare_local(name)?;
            match init {
                Some(e) => self.expr(e)?,
                None => self.emit(Op::LoadNil),
            }
            self.mark_initialized();
        }
        Ok(())
    }

    fn if_stmt(
        &mut self,
        cond: &Expr,
        then_b: &Stmt,
        else_b: &Option<Box<Stmt>>,
    ) -> SableResult<()> {
        self.expr(cond)?;
        let else_jump = self.jump(Op::JumpIfFalse);
        self.emit(Op::Pop);
        self.stmt(then_b)?;
        let end_jump = self.jump(Op::Jump);
        self.patch(else_jump)?;
        self.emit(Op::Pop);
        if let Some(eb) = else_b {
            self.stmt(eb)?;
        }
        self.patch(end_jump)?;
        Ok(())
    }

    fn while_stmt(&mut self, cond: &Expr, body: &Stmt) -> SableResult<()> {
        let loop_start = self.cur().code.len();
        self.expr(cond)?;
        let exit_jump = self.jump(Op::JumpIfFalse);
        self.emit(Op::Pop);
        let base_locals = self.cur().locals.len();
        self.cur().loops.push(LoopCtx {
            start: loop_start,
            breaks: Vec::new(),
            base_locals,
        });
        self.stmt(body)?;
        self.emit_loop(loop_start)?;
        self.patch(exit_jump)?;
        self.emit(Op::Pop);
        let lp = self.cur().loops.pop().unwrap();
        for b in lp.breaks {
            self.patch(b)?;
        }
        Ok(())
    }

    fn break_stmt(&mut self) -> SableResult<()> {
        let (base, has_loop) = match self.cur().loops.last() {
            Some(l) => (l.base_locals, true),
            None => (0, false),
        };
        if !has_loop {
            return Err(SableError::Compile(
                "break outside loop".to_string(),
                Span::zero(),
            ));
        }
        self.emit_pops_to(base);
        let j = self.jump(Op::Jump);
        self.cur().loops.last_mut().unwrap().breaks.push(j);
        Ok(())
    }

    fn continue_stmt(&mut self) -> SableResult<()> {
        let (base, start, has_loop) = match self.cur().loops.last() {
            Some(l) => (l.base_locals, l.start, true),
            None => (0, 0, false),
        };
        if !has_loop {
            return Err(SableError::Compile(
                "continue outside loop".to_string(),
                Span::zero(),
            ));
        }
        self.emit_pops_to(base);
        self.emit_loop(start)?;
        Ok(())
    }

    fn return_stmt(&mut self, val: &Option<Expr>) -> SableResult<()> {
        match val {
            Some(e) => self.expr(e)?,
            None => self.emit(Op::LoadNil),
        }
        self.emit(Op::Return);
        Ok(())
    }

    fn fn_stmt(&mut self, name: &str, params: &[String], body: &[Stmt]) -> SableResult<()> {
        if self.cur().scope_depth == 0 {
            self.function(name, params, body)?;
            let idx = self.add_const(Konst::Str(name.to_string()))?;
            self.emit_op_u16(Op::DefineGlobal, idx);
        } else {
            self.declare_local(name)?;
            self.mark_initialized();
            self.function(name, params, body)?;
        }
        Ok(())
    }

    fn function(&mut self, name: &str, params: &[String], body: &[Stmt]) -> SableResult<()> {
        let arity = params.len() as u8;
        self.stack.push(FnCtx::new(name.to_string(), arity));
        self.cur().locals.push(Local {
            name: String::new(),
            depth: 0,
            is_captured: false,
        });
        self.begin_scope();
        for p in params {
            self.declare_local(p)?;
            self.mark_initialized();
        }
        for st in body {
            self.stmt(st)?;
        }
        self.emit(Op::LoadNil);
        self.emit(Op::Return);
        let ctx = self.stack.pop().unwrap();
        let proto = finish(ctx);
        let pi = {
            let enclosing = self.cur();
            enclosing.protos.push(proto);
            enclosing.protos.len() - 1
        };
        if pi > 0xffff {
            return Err(SableError::Compile(
                "too many functions".to_string(),
                Span::zero(),
            ));
        }
        self.emit_op_u16(Op::Closure, pi as u16);
        Ok(())
    }

    fn expr(&mut self, e: &Expr) -> SableResult<()> {
        match e {
            Expr::Number(n) => self.emit_const(Konst::Num(*n)),
            Expr::Str(s) => self.emit_const(Konst::Str(s.clone())),
            Expr::Bool(b) => {
                self.emit(if *b { Op::LoadTrue } else { Op::LoadFalse });
                Ok(())
            }
            Expr::Nil => {
                self.emit(Op::LoadNil);
                Ok(())
            }
            Expr::Ident(name) => self.get_variable(name),
            Expr::Unary(op, inner) => {
                self.expr(inner)?;
                match op {
                    UnOp::Neg => self.emit(Op::Neg),
                    UnOp::Not => self.emit(Op::Not),
                }
                Ok(())
            }
            Expr::Binary(op, a, b) => {
                self.expr(a)?;
                self.expr(b)?;
                self.emit_binop(*op);
                Ok(())
            }
            Expr::Logical(op, a, b) => self.logical(*op, a, b),
            Expr::Call(callee, args) => {
                self.expr(callee)?;
                if args.len() > 255 {
                    return Err(SableError::Compile(
                        "too many arguments".to_string(),
                        Span::zero(),
                    ));
                }
                for a in args {
                    self.expr(a)?;
                }
                self.emit_op_u8(Op::Call, args.len() as u8);
                Ok(())
            }
            Expr::Index(obj, key) => {
                self.expr(obj)?;
                self.expr(key)?;
                self.emit(Op::GetIndex);
                Ok(())
            }
            Expr::Array(items) => {
                if items.len() > 0xffff {
                    return Err(SableError::Compile(
                        "array literal too large".to_string(),
                        Span::zero(),
                    ));
                }
                for it in items {
                    self.expr(it)?;
                }
                self.emit_op_u16(Op::NewArray, items.len() as u16);
                Ok(())
            }
            Expr::Map(_pairs) => {
                self.emit_op_u16(Op::NewMap, 0);
                Ok(())
            }
            Expr::Function(params, body) => self.function("", params, body),
            Expr::Assign(target, value) => self.assign(target, value),
        }
    }

    fn emit_binop(&mut self, op: BinOp) {
        let o = match op {
            BinOp::Add => Op::Add,
            BinOp::Sub => Op::Sub,
            BinOp::Mul => Op::Mul,
            BinOp::Div => Op::Div,
            BinOp::Mod => Op::Mod,
            BinOp::Eq => Op::Eq,
            BinOp::Ne => Op::Ne,
            BinOp::Lt => Op::Lt,
            BinOp::Le => Op::Le,
            BinOp::Gt => Op::Gt,
            BinOp::Ge => Op::Ge,
        };
        self.emit(o);
    }

    fn logical(&mut self, op: LogOp, a: &Expr, b: &Expr) -> SableResult<()> {
        match op {
            LogOp::And => {
                self.expr(a)?;
                let end = self.jump(Op::JumpIfFalse);
                self.emit(Op::Pop);
                self.expr(b)?;
                self.patch(end)?;
                Ok(())
            }
            LogOp::Or => {
                self.expr(a)?;
                let else_j = self.jump(Op::JumpIfFalse);
                let end_j = self.jump(Op::Jump);
                self.patch(else_j)?;
                self.emit(Op::Pop);
                self.expr(b)?;
                self.patch(end_j)?;
                Ok(())
            }
        }
    }

    fn assign(&mut self, target: &Expr, value: &Expr) -> SableResult<()> {
        match target {
            Expr::Ident(name) => {
                self.expr(value)?;
                self.set_variable(name)?;
                Ok(())
            }
            Expr::Index(obj, key) => {
                self.expr(obj)?;
                self.expr(key)?;
                self.expr(value)?;
                self.emit(Op::SetIndex);
                Ok(())
            }
            _ => Err(SableError::Compile(
                "invalid assignment target".to_string(),
                Span::zero(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::opcode::Op;
    use crate::parser::Parser;

    fn compile_source(src: &str) -> CompiledProto {
        let toks = Lexer::new(src).tokenize().unwrap();
        let prog = Parser::new(toks).parse_program().unwrap();
        compile(&prog).unwrap()
    }

    #[test]
    fn emits_return_and_consts() {
        let p = compile_source("return 5 + 3;");
        assert!(p.code.contains(&(Op::Return as u8)));
        assert!(!p.consts.is_empty());
    }

    #[test]
    fn functions_become_child_protos() {
        let p = compile_source("fn f() { return 1; } return f();");
        assert_eq!(p.protos.len(), 1);
        assert_eq!(p.protos[0].name, "f");
    }

    #[test]
    fn too_many_locals_is_error() {
        let mut src = String::from("fn f() {");
        for i in 0..260 {
            src.push_str(&format!("let v{} = {};", i, i));
        }
        src.push_str("return 0; }");
        let toks = Lexer::new(&src).tokenize().unwrap();
        let prog = Parser::new(toks).parse_program().unwrap();
        assert!(compile(&prog).is_err());
    }
}
