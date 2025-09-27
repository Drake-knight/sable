use crate::error::SableResult;
use crate::table::Table;
use crate::value::Value;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum ObjKind {
    Str,
    Array,
    Map,
    Proto,
    Closure,
    Upvalue,
    Native,
}

#[repr(C)]
pub struct Obj {
    pub kind: ObjKind,
    pub marked: bool,
    pub next: *mut Obj,
}

#[repr(C)]
pub struct ObjStr {
    pub hdr: Obj,
    pub len: usize,
    pub cap: usize,
    pub data: *mut u8,
}

#[repr(C)]
pub struct ObjArray {
    pub hdr: Obj,
    pub len: usize,
    pub cap: usize,
    pub items: *mut Value,
}

#[repr(C)]
pub struct ObjMap {
    pub hdr: Obj,
    pub table: Table,
}

#[derive(Clone, Copy)]
pub struct UpInfo {
    pub is_local: bool,
    pub index: u8,
}

#[repr(C)]
pub struct ObjProto {
    pub hdr: Obj,
    pub arity: u8,
    pub upvals: u8,
    pub max_stack: u16,
    pub code: Vec<u8>,
    pub consts: Vec<Value>,
    pub protos: Vec<*mut Obj>,
    pub upinfo: Vec<UpInfo>,
    pub name: *mut Obj,
}

#[repr(C)]
pub struct ObjClosure {
    pub hdr: Obj,
    pub proto: *mut Obj,
    pub upvalues: *mut *mut Obj,
    pub upcount: usize,
}

#[repr(C)]
pub struct ObjUpvalue {
    pub hdr: Obj,
    pub location: *mut Value,
    pub closed: Value,
    pub next_open: *mut Obj,
}

pub type NativeFn = fn(&mut crate::vm::Vm, &[Value]) -> SableResult<Value>;

#[repr(C)]
pub struct ObjNative {
    pub hdr: Obj,
    pub arity: i32,
    pub func: NativeFn,
    pub name: *mut Obj,
}

pub unsafe fn kind_of(o: *mut Obj) -> ObjKind {
    (*o).kind
}

pub unsafe fn str_bytes<'a>(o: *mut ObjStr) -> &'a [u8] {
    std::slice::from_raw_parts((*o).data, (*o).len)
}
