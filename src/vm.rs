use crate::chunk::{self, CompiledProto, Konst};
use crate::compiler;
use crate::error::{SableError, SableResult};
use crate::gc;
use crate::heap::{Heap, INITIAL_NEXT_GC};
use crate::lexer::Lexer;
use crate::object::{
    NativeFn, Obj, ObjArray, ObjClosure, ObjKind, ObjMap, ObjNative, ObjProto, ObjStr, ObjUpvalue,
    UpInfo,
};
use crate::opcode::Op;
use crate::parser::Parser;
use crate::table::{key_eq, Table};
use crate::value::Value;
use std::alloc::Layout;
use std::mem::size_of;

pub const STACK_MAX: usize = 1 << 16;
pub const STACK_PAD: usize = 512;
pub const FRAMES_MAX: usize = 1024;

pub struct CallFrame {
    pub closure: *mut Obj,
    pub ip: usize,
    pub base: usize,
}

pub struct Vm {
    pub stack: Vec<Value>,
    pub top: usize,
    pub frames: Vec<CallFrame>,
    pub globals: Table,
    pub open_upvalues: *mut Obj,
    pub heap: Heap,
    pub rng: u64,
}

fn obj_kind(v: Value) -> Option<ObjKind> {
    if v.is_object() {
        Some(unsafe { (*v.as_obj()).kind })
    } else {
        None
    }
}

impl Vm {
    pub fn new() -> Vm {
        let mut stack = Vec::new();
        stack.resize(STACK_MAX + STACK_PAD, Value::nil());
        let mut vm = Vm {
            stack,
            top: 0,
            frames: Vec::new(),
            globals: Table::new(),
            open_upvalues: std::ptr::null_mut(),
            heap: Heap::new(),
            rng: 0x2545f4914f6cdd1d,
        };
        crate::builtins::install(&mut vm);
        vm
    }

    pub fn eval_str(&mut self, src: &str) -> SableResult<Value> {
        let tokens = Lexer::new(src).tokenize()?;
        let program = Parser::new(tokens).parse_program()?;
        let compiled = compiler::compile(&program)?;
        self.run_compiled(&compiled)
    }

    pub fn run_chunk(&mut self, bytes: &[u8]) -> SableResult<Value> {
        let compiled = chunk::from_bytes(bytes)?;
        self.run_compiled(&compiled)
    }

    fn run_compiled(&mut self, cp: &CompiledProto) -> SableResult<Value> {
        self.heap.next_gc = usize::MAX;
        let proto = self.materialize(cp);
        self.push_temp(proto);
        let closure = self.new_closure(proto);
        self.pop_temp();
        self.heap.next_gc = self.heap.bytes_allocated + INITIAL_NEXT_GC;
        self.top = 0;
        self.frames.clear();
        self.open_upvalues = std::ptr::null_mut();
        self.push(Value::from_obj(closure))?;
        self.frames.push(CallFrame {
            closure,
            ip: 0,
            base: 0,
        });
        self.run()
    }

    pub fn push(&mut self, v: Value) -> SableResult<()> {
        if self.top >= STACK_MAX {
            return Err(SableError::Runtime("stack overflow".to_string()));
        }
        self.stack[self.top] = v;
        self.top += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Value {
        if self.top == 0 {
            return Value::nil();
        }
        self.top -= 1;
        self.stack[self.top]
    }

    pub fn peek(&self, dist: usize) -> Value {
        if dist >= self.top {
            return Value::nil();
        }
        self.stack[self.top - 1 - dist]
    }

    pub fn push_temp(&mut self, o: *mut Obj) {
        self.heap.temp_roots.push(o);
    }

    pub fn pop_temp(&mut self) {
        self.heap.temp_roots.pop();
    }

    fn pre_alloc(&mut self, size: usize) {
        self.heap.bytes_allocated += size;
        if self.heap.bytes_allocated > self.heap.next_gc {
            gc::collect(self);
        }
    }

    fn register(&mut self, o: *mut Obj) {
        unsafe {
            (*o).next = self.heap.head;
        }
        self.heap.head = o;
    }

    pub fn new_str(&mut self, bytes: &[u8]) -> *mut Obj {
        let len = bytes.len();
        let cap = if len == 0 { 1 } else { len };
        self.pre_alloc(size_of::<ObjStr>() + cap);
        let data = unsafe { std::alloc::alloc(Layout::from_size_align(cap, 1).unwrap()) };
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), data, len);
        }
        let s = Box::into_raw(Box::new(ObjStr {
            hdr: Obj {
                kind: ObjKind::Str,
                marked: false,
                next: std::ptr::null_mut(),
            },
            len,
            cap,
            data,
        })) as *mut Obj;
        self.register(s);
        s
    }

    pub fn new_str_uninit(&mut self, len: usize) -> *mut ObjStr {
        let cap = if len == 0 { 1 } else { len };
        self.pre_alloc(size_of::<ObjStr>() + cap);
        let data = unsafe { std::alloc::alloc(Layout::from_size_align(cap, 1).unwrap()) };
        let s = Box::into_raw(Box::new(ObjStr {
            hdr: Obj {
                kind: ObjKind::Str,
                marked: false,
                next: std::ptr::null_mut(),
            },
            len,
            cap,
            data,
        }));
        self.register(s as *mut Obj);
        s
    }

    pub fn new_array(&mut self, len: usize) -> *mut ObjArray {
        let cap = if len == 0 { 1 } else { len };
        self.pre_alloc(size_of::<ObjArray>() + cap * size_of::<Value>());
        let items =
            unsafe { std::alloc::alloc(Layout::array::<Value>(cap).unwrap()) as *mut Value };
        for i in 0..cap {
            unsafe {
                *items.add(i) = Value::nil();
            }
        }
        let a = Box::into_raw(Box::new(ObjArray {
            hdr: Obj {
                kind: ObjKind::Array,
                marked: false,
                next: std::ptr::null_mut(),
            },
            len,
            cap,
            items,
        }));
        self.register(a as *mut Obj);
        a
    }

    pub fn new_map(&mut self) -> *mut ObjMap {
        self.pre_alloc(size_of::<ObjMap>());
        let m = Box::into_raw(Box::new(ObjMap {
            hdr: Obj {
                kind: ObjKind::Map,
                marked: false,
                next: std::ptr::null_mut(),
            },
            table: Table::new(),
        }));
        self.register(m as *mut Obj);
        m
    }

    pub fn new_native(&mut self, arity: i32, func: NativeFn, name: *mut Obj) -> *mut Obj {
        self.pre_alloc(size_of::<ObjNative>());
        let n = Box::into_raw(Box::new(ObjNative {
            hdr: Obj {
                kind: ObjKind::Native,
                marked: false,
                next: std::ptr::null_mut(),
            },
            arity,
            func,
            name,
        })) as *mut Obj;
        self.register(n);
        n
    }

    pub fn new_upvalue(&mut self, location: *mut Value) -> *mut Obj {
        self.pre_alloc(size_of::<ObjUpvalue>());
        let u = Box::into_raw(Box::new(ObjUpvalue {
            hdr: Obj {
                kind: ObjKind::Upvalue,
                marked: false,
                next: std::ptr::null_mut(),
            },
            location,
            closed: Value::nil(),
            next_open: std::ptr::null_mut(),
        })) as *mut Obj;
        self.register(u);
        u
    }

    pub fn new_closure(&mut self, proto: *mut Obj) -> *mut Obj {
        let upcount = unsafe { (*(proto as *mut ObjProto)).upvals as usize };
        self.pre_alloc(size_of::<ObjClosure>() + upcount * size_of::<*mut Obj>());
        let upvalues = if upcount == 0 {
            std::ptr::null_mut()
        } else {
            let p = unsafe {
                std::alloc::alloc(Layout::array::<*mut Obj>(upcount).unwrap()) as *mut *mut Obj
            };
            for i in 0..upcount {
                unsafe {
                    *p.add(i) = std::ptr::null_mut();
                }
            }
            p
        };
        let c = Box::into_raw(Box::new(ObjClosure {
            hdr: Obj {
                kind: ObjKind::Closure,
                marked: false,
                next: std::ptr::null_mut(),
            },
            proto,
            upvalues,
            upcount,
        })) as *mut Obj;
        self.register(c);
        c
    }

    pub fn materialize(&mut self, cp: &CompiledProto) -> *mut Obj {
        let mut consts: Vec<Value> = Vec::with_capacity(cp.consts.len());
        for k in &cp.consts {
            let v = match k {
                Konst::Num(n) => Value::number(*n),
                Konst::Bool(b) => Value::bool(*b),
                Konst::Nil => Value::nil(),
                Konst::Str(s) => Value::from_obj(self.new_str(s.as_bytes())),
            };
            consts.push(v);
        }
        let mut protos: Vec<*mut Obj> = Vec::with_capacity(cp.protos.len());
        for child in &cp.protos {
            protos.push(self.materialize(child));
        }
        let upinfo: Vec<UpInfo> = cp
            .upinfo
            .iter()
            .map(|u| UpInfo {
                is_local: u.is_local,
                index: u.index,
            })
            .collect();
        let name = if cp.name.is_empty() {
            std::ptr::null_mut()
        } else {
            self.new_str(cp.name.as_bytes())
        };
        self.pre_alloc(size_of::<ObjProto>());
        let p = Box::into_raw(Box::new(ObjProto {
            hdr: Obj {
                kind: ObjKind::Proto,
                marked: false,
                next: std::ptr::null_mut(),
            },
            arity: cp.arity,
            upvals: cp.upvals,
            max_stack: cp.max_stack,
            code: cp.code.clone(),
            consts,
            protos,
            upinfo,
            name,
        })) as *mut Obj;
        self.register(p);
        p
    }

    pub fn define_global(&mut self, name: &str, value: Value) {
        if value.is_object() {
            self.push_temp(value.as_obj());
        }
        let key = self.new_str(name.as_bytes());
        self.push_temp(key);
        self.globals.set(Value::from_obj(key), value);
        self.pop_temp();
        if value.is_object() {
            self.pop_temp();
        }
    }

    fn cur_proto(&self) -> *mut ObjProto {
        let fi = self.frames.len() - 1;
        let closure = self.frames[fi].closure;
        unsafe { (*(closure as *mut ObjClosure)).proto as *mut ObjProto }
    }

    fn read_u8(&mut self) -> SableResult<u8> {
        let fi = self.frames.len() - 1;
        let proto = self.cur_proto();
        let ip = self.frames[fi].ip;
        let code = unsafe { &(*proto).code };
        if ip >= code.len() {
            return Err(SableError::Runtime("unexpected end of bytecode".to_string()));
        }
        let b = code[ip];
        self.frames[fi].ip = ip + 1;
        Ok(b)
    }

    fn read_u16(&mut self) -> SableResult<u16> {
        let lo = self.read_u8()? as u16;
        let hi = self.read_u8()? as u16;
        Ok(lo | (hi << 8))
    }

    fn read_const(&mut self, idx: usize) -> Value {
        let proto = self.cur_proto();
        unsafe {
            let consts = &(*proto).consts;
            *consts.as_ptr().add(idx)
        }
    }

    fn concat(&mut self, a: Value, b: Value) -> *mut Obj {
        let sa = a.as_obj() as *mut ObjStr;
        let sb = b.as_obj() as *mut ObjStr;
        let la = unsafe { (*sa).len };
        let lb = unsafe { (*sb).len };
        let dst = self.new_str_uninit(la + lb);
        unsafe {
            std::ptr::copy_nonoverlapping((*sa).data, (*dst).data, la);
            std::ptr::copy_nonoverlapping((*sb).data, (*dst).data.add(la), lb);
        }
        dst as *mut Obj
    }

    fn capture_upvalue(&mut self, slot: usize) -> *mut Obj {
        let location = unsafe { self.stack.as_mut_ptr().add(slot) };
        let mut prev: *mut Obj = std::ptr::null_mut();
        let mut cur = self.open_upvalues;
        unsafe {
            while !cur.is_null()
                && ((*(cur as *mut ObjUpvalue)).location as usize) > (location as usize)
            {
                prev = cur;
                cur = (*(cur as *mut ObjUpvalue)).next_open;
            }
            if !cur.is_null() && (*(cur as *mut ObjUpvalue)).location == location {
                return cur;
            }
        }
        let created = self.new_upvalue(location);
        unsafe {
            (*(created as *mut ObjUpvalue)).next_open = cur;
        }
        if prev.is_null() {
            self.open_upvalues = created;
        } else {
            unsafe {
                (*(prev as *mut ObjUpvalue)).next_open = created;
            }
        }
        created
    }

    fn close_upvalues(&mut self, from: usize) {
        let from_ptr = unsafe { self.stack.as_mut_ptr().add(from) } as usize;
        unsafe {
            while !self.open_upvalues.is_null() {
                let uv = self.open_upvalues as *mut ObjUpvalue;
                if ((*uv).location as usize) < from_ptr {
                    break;
                }
                (*uv).closed = *(*uv).location;
                (*uv).location = &mut (*uv).closed as *mut Value;
                self.open_upvalues = (*uv).next_open;
            }
        }
    }

    fn call_value(&mut self, callee: Value, argc: usize) -> SableResult<()> {
        match obj_kind(callee) {
            Some(ObjKind::Closure) => self.call_closure(callee.as_obj(), argc),
            Some(ObjKind::Native) => self.call_native(callee.as_obj(), argc),
            _ => Err(SableError::Type("can only call functions".to_string())),
        }
    }

    fn call_closure(&mut self, closure: *mut Obj, argc: usize) -> SableResult<()> {
        let proto = unsafe { (*(closure as *mut ObjClosure)).proto as *mut ObjProto };
        let arity = unsafe { (*proto).arity as usize };
        if argc != arity {
            return Err(SableError::Arity(format!(
                "expected {} arguments but got {}",
                arity, argc
            )));
        }
        if self.frames.len() >= FRAMES_MAX {
            return Err(SableError::Runtime("call depth exceeded".to_string()));
        }
        let base = self.top - argc - 1;
        self.frames.push(CallFrame {
            closure,
            ip: 0,
            base,
        });
        Ok(())
    }

    fn call_native(&mut self, native: *mut Obj, argc: usize) -> SableResult<()> {
        let n = native as *mut ObjNative;
        let func = unsafe { (*n).func };
        let arity = unsafe { (*n).arity };
        if arity >= 0 && argc != arity as usize {
            return Err(SableError::Arity(format!(
                "expected {} arguments but got {}",
                arity, argc
            )));
        }
        let start = self.top - argc;
        let args: Vec<Value> = self.stack[start..self.top].to_vec();
        let result = func(self, &args)?;
        self.top = start - 1;
        self.push(result)?;
        Ok(())
    }

    fn run(&mut self) -> SableResult<Value> {
        loop {
            let op_byte = self.read_u8()?;
            let op = match Op::from_u8(op_byte) {
                Some(o) => o,
                None => return Err(SableError::Runtime("invalid opcode".to_string())),
            };
            match op {
                Op::LoadConst => {
                    let idx = self.read_u16()? as usize;
                    let v = self.read_const(idx);
                    self.push(v)?;
                }
                Op::LoadNil => self.push(Value::nil())?,
                Op::LoadTrue => self.push(Value::bool(true))?,
                Op::LoadFalse => self.push(Value::bool(false))?,
                Op::LoadInt => {
                    let raw = self.read_u16()? as i16;
                    self.push(Value::number(raw as f64))?;
                }
                Op::Pop => {
                    self.pop();
                }
                Op::Dup => {
                    let v = self.peek(0);
                    self.push(v)?;
                }
                Op::Swap => {
                    if self.top >= 2 {
                        self.stack.swap(self.top - 1, self.top - 2);
                    }
                }
                Op::GetLocal => {
                    let slot = self.read_u8()? as usize;
                    let base = self.frames[self.frames.len() - 1].base;
                    let v = self.stack[base + slot];
                    self.push(v)?;
                }
                Op::SetLocal => {
                    let slot = self.read_u8()? as usize;
                    let base = self.frames[self.frames.len() - 1].base;
                    self.stack[base + slot] = self.peek(0);
                }
                Op::GetGlobal => {
                    let idx = self.read_u16()? as usize;
                    let name = self.read_const(idx);
                    if !self.globals.has(name) {
                        return Err(SableError::Runtime("undefined global".to_string()));
                    }
                    let v = self.globals.get(name);
                    self.push(v)?;
                }
                Op::SetGlobal => {
                    let idx = self.read_u16()? as usize;
                    let name = self.read_const(idx);
                    if !self.globals.has(name) {
                        return Err(SableError::Runtime("undefined global".to_string()));
                    }
                    let v = self.peek(0);
                    self.globals.set(name, v);
                }
                Op::DefineGlobal => {
                    let idx = self.read_u16()? as usize;
                    let name = self.read_const(idx);
                    let v = self.peek(0);
                    self.globals.set(name, v);
                    self.pop();
                }
                Op::GetUpvalue => {
                    let i = self.read_u8()? as usize;
                    let closure = self.frames[self.frames.len() - 1].closure as *mut ObjClosure;
                    let upcount = unsafe { (*closure).upcount };
                    if i >= upcount {
                        return Err(SableError::Runtime("bad upvalue index".to_string()));
                    }
                    let uv = unsafe { *(*closure).upvalues.add(i) } as *mut ObjUpvalue;
                    let v = unsafe { *(*uv).location };
                    self.push(v)?;
                }
                Op::SetUpvalue => {
                    let i = self.read_u8()? as usize;
                    let closure = self.frames[self.frames.len() - 1].closure as *mut ObjClosure;
                    let upcount = unsafe { (*closure).upcount };
                    if i >= upcount {
                        return Err(SableError::Runtime("bad upvalue index".to_string()));
                    }
                    let uv = unsafe { *(*closure).upvalues.add(i) } as *mut ObjUpvalue;
                    let v = self.peek(0);
                    unsafe {
                        *(*uv).location = v;
                    }
                }
                Op::CloseUpvalue => {
                    if self.top > 0 {
                        self.close_upvalues(self.top - 1);
                    }
                    self.pop();
                }
                Op::Add => {
                    let b = self.peek(0);
                    let a = self.peek(1);
                    if a.is_number() && b.is_number() {
                        self.pop();
                        self.pop();
                        self.push(Value::number(a.as_number() + b.as_number()))?;
                    } else if obj_kind(a) == Some(ObjKind::Str) && obj_kind(b) == Some(ObjKind::Str)
                    {
                        let r = self.concat(a, b);
                        self.pop();
                        self.pop();
                        self.push(Value::from_obj(r))?;
                    } else {
                        return Err(SableError::Type(
                            "operands must be numbers or strings".to_string(),
                        ));
                    }
                }
                Op::Sub => self.arith(op)?,
                Op::Mul => self.arith(op)?,
                Op::Div => self.arith(op)?,
                Op::Mod => self.arith(op)?,
                Op::Neg => {
                    let v = self.peek(0);
                    if !v.is_number() {
                        return Err(SableError::Type("operand must be a number".to_string()));
                    }
                    self.pop();
                    self.push(Value::number(-v.as_number()))?;
                }
                Op::Not => {
                    let v = self.pop();
                    self.push(Value::bool(!v.is_truthy()))?;
                }
                Op::Eq => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::bool(values_equal(a, b)))?;
                }
                Op::Ne => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::bool(!values_equal(a, b)))?;
                }
                Op::Lt => self.compare(op)?,
                Op::Le => self.compare(op)?,
                Op::Gt => self.compare(op)?,
                Op::Ge => self.compare(op)?,
                Op::NewArray => {
                    let count = self.read_u16()? as usize;
                    if count > self.top {
                        return Err(SableError::Runtime("array underflow".to_string()));
                    }
                    let arr = self.new_array(count);
                    let start = self.top - count;
                    for i in 0..count {
                        unsafe {
                            *(*arr).items.add(i) = self.stack[start + i];
                        }
                    }
                    self.top = start;
                    self.push(Value::from_obj(arr as *mut Obj))?;
                }
                Op::NewMap => {
                    let _count = self.read_u16()?;
                    let m = self.new_map();
                    self.push(Value::from_obj(m as *mut Obj))?;
                }
                Op::GetIndex => {
                    let key = self.pop();
                    let target = self.pop();
                    if !target.is_object() {
                        return Err(SableError::Type("can only index objects".to_string()));
                    }
                    let o = target.as_obj();
                    if unsafe { (*o).kind } == ObjKind::Array {
                        if !key.is_number() {
                            return Err(SableError::Type("array index must be a number".to_string()));
                        }
                        let a = o as *mut ObjArray;
                        let len = unsafe { (*a).len };
                        let idx = key.as_number() as i64;
                        if idx < 0 || idx as usize >= len {
                            return Err(SableError::Runtime("array index out of range".to_string()));
                        }
                        let v = unsafe { *(*a).items.add(idx as usize) };
                        self.push(v)?;
                    } else {
                        let m = o as *mut ObjMap;
                        let t = unsafe { &(*m).table };
                        let v = t.get(key);
                        self.push(v)?;
                    }
                }
                Op::SetIndex => {
                    let value = self.pop();
                    let key = self.pop();
                    let target = self.pop();
                    if !target.is_object() {
                        return Err(SableError::Type("can only index objects".to_string()));
                    }
                    let o = target.as_obj();
                    match unsafe { (*o).kind } {
                        ObjKind::Array => {
                            if !key.is_number() {
                                return Err(SableError::Type(
                                    "array index must be a number".to_string(),
                                ));
                            }
                            let a = o as *mut ObjArray;
                            let len = unsafe { (*a).len };
                            let idx = key.as_number() as i64;
                            if idx < 0 || idx as usize >= len {
                                return Err(SableError::Runtime(
                                    "array index out of range".to_string(),
                                ));
                            }
                            unsafe {
                                *(*a).items.add(idx as usize) = value;
                            }
                        }
                        ObjKind::Map => {
                            let m = o as *mut ObjMap;
                            let t = unsafe { &mut (*m).table };
                            t.set(key, value);
                        }
                        _ => {
                            return Err(SableError::Type(
                                "can only assign array or map index".to_string(),
                            ))
                        }
                    }
                    self.push(value)?;
                }
                Op::Len => {
                    let v = self.pop();
                    let n = length_of(v);
                    self.push(Value::number(n as f64))?;
                }
                Op::Jump => {
                    let off = self.read_u16()? as usize;
                    let fi = self.frames.len() - 1;
                    self.frames[fi].ip += off;
                }
                Op::JumpIfFalse => {
                    let off = self.read_u16()? as usize;
                    if !self.peek(0).is_truthy() {
                        let fi = self.frames.len() - 1;
                        self.frames[fi].ip += off;
                    }
                }
                Op::Loop => {
                    let off = self.read_u16()? as usize;
                    let fi = self.frames.len() - 1;
                    let ip = self.frames[fi].ip;
                    self.frames[fi].ip = ip.saturating_sub(off);
                }
                Op::Call => {
                    let argc = self.read_u8()? as usize;
                    let callee = self.peek(argc);
                    self.call_value(callee, argc)?;
                }
                Op::Return => {
                    let result = self.pop();
                    let fi = self.frames.len() - 1;
                    let base = self.frames[fi].base;
                    self.close_upvalues(base);
                    self.frames.pop();
                    if self.frames.is_empty() {
                        self.top = 0;
                        return Ok(result);
                    }
                    self.top = base;
                    self.push(result)?;
                }
                Op::Closure => {
                    let pi = self.read_u16()? as usize;
                    let cur = self.cur_proto();
                    let proto = {
                        let protos = unsafe { &(*cur).protos };
                        if pi >= protos.len() {
                            return Err(SableError::Runtime("bad function index".to_string()));
                        }
                        protos[pi]
                    };
                    let closure = self.new_closure(proto);
                    self.push_temp(closure);
                    let upcount = unsafe { (*(closure as *mut ObjClosure)).upcount };
                    for i in 0..upcount {
                        let cp = proto as *mut ObjProto;
                        let (is_local, index) = {
                            let ui = unsafe { &(*cp).upinfo };
                            if i >= ui.len() {
                                (false, 0usize)
                            } else {
                                (ui[i].is_local, ui[i].index as usize)
                            }
                        };
                        let captured = if is_local {
                            let base = self.frames[self.frames.len() - 1].base;
                            self.capture_upvalue(base + index)
                        } else {
                            let enclosing =
                                self.frames[self.frames.len() - 1].closure as *mut ObjClosure;
                            let ec = unsafe { (*enclosing).upcount };
                            if index >= ec {
                                std::ptr::null_mut()
                            } else {
                                unsafe { *(*enclosing).upvalues.add(index) }
                            }
                        };
                        unsafe {
                            *(*(closure as *mut ObjClosure)).upvalues.add(i) = captured;
                        }
                    }
                    self.pop_temp();
                    self.push(Value::from_obj(closure))?;
                }
            }
        }
    }

    fn arith(&mut self, op: Op) -> SableResult<()> {
        let b = self.peek(0);
        let a = self.peek(1);
        if !a.is_number() || !b.is_number() {
            return Err(SableError::Type("operands must be numbers".to_string()));
        }
        self.pop();
        self.pop();
        let x = a.as_number();
        let y = b.as_number();
        let r = match op {
            Op::Sub => x - y,
            Op::Mul => x * y,
            Op::Div => x / y,
            Op::Mod => x % y,
            _ => 0.0,
        };
        self.push(Value::number(r))
    }

    fn compare(&mut self, op: Op) -> SableResult<()> {
        let b = self.pop();
        let a = self.pop();
        if !a.is_number() || !b.is_number() {
            return Err(SableError::Type("operands must be numbers".to_string()));
        }
        let x = a.as_number();
        let y = b.as_number();
        let r = match op {
            Op::Lt => x < y,
            Op::Le => x <= y,
            Op::Gt => x > y,
            Op::Ge => x >= y,
            _ => false,
        };
        self.push(Value::bool(r))
    }
}

fn values_equal(a: Value, b: Value) -> bool {
    if a.is_number() && b.is_number() {
        return a.as_number() == b.as_number();
    }
    if a.is_object() && b.is_object() {
        return key_eq(a, b);
    }
    a.bits() == b.bits()
}

fn length_of(v: Value) -> usize {
    match obj_kind(v) {
        Some(ObjKind::Str) => unsafe { (*(v.as_obj() as *mut ObjStr)).len },
        Some(ObjKind::Array) => unsafe { (*(v.as_obj() as *mut ObjArray)).len },
        Some(ObjKind::Map) => unsafe { (*(v.as_obj() as *mut ObjMap)).table.count },
        _ => 0,
    }
}

impl Drop for Vm {
    fn drop(&mut self) {
        self.heap.free_all();
        self.globals.free();
    }
}
