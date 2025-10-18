use crate::object::{
    Obj, ObjArray, ObjClosure, ObjKind, ObjMap, ObjNative, ObjProto, ObjStr, ObjUpvalue,
};
use crate::value::Value;

pub const INITIAL_NEXT_GC: usize = 1 << 14;

pub struct Heap {
    pub head: *mut Obj,
    pub bytes_allocated: usize,
    pub next_gc: usize,
    pub temp_roots: Vec<*mut Obj>,
    pub grey: Vec<*mut Obj>,
}

impl Heap {
    pub fn new() -> Heap {
        Heap {
            head: std::ptr::null_mut(),
            bytes_allocated: 0,
            next_gc: INITIAL_NEXT_GC,
            temp_roots: Vec::new(),
            grey: Vec::new(),
        }
    }

    pub fn mark_obj(&mut self, o: *mut Obj) {
        if o.is_null() {
            return;
        }
        unsafe {
            if (*o).marked {
                return;
            }
            (*o).marked = true;
            self.grey.push(o);
        }
    }

    pub fn mark_value(&mut self, v: Value) {
        if v.is_object() {
            self.mark_obj(v.as_obj());
        }
    }

    pub fn trace(&mut self) {
        while let Some(o) = self.grey.pop() {
            self.blacken(o);
        }
    }

    fn blacken(&mut self, o: *mut Obj) {
        unsafe {
            match (*o).kind {
                ObjKind::Str => {}
                ObjKind::Array => {
                    let a = o as *mut ObjArray;
                    for i in 0..(*a).len {
                        let v = *(*a).items.add(i);
                        self.mark_value(v);
                    }
                }
                ObjKind::Map => {
                    let m = o as *mut ObjMap;
                    (*m).table.mark(self);
                }
                ObjKind::Proto => {
                    let p = o as *mut ObjProto;
                    self.mark_obj((*p).name);
                    let consts = &(*p).consts;
                    for i in 0..consts.len() {
                        let v = consts[i];
                        self.mark_value(v);
                    }
                    let protos = &(*p).protos;
                    for i in 0..protos.len() {
                        self.mark_obj(protos[i]);
                    }
                }
                ObjKind::Closure => {
                    let c = o as *mut ObjClosure;
                    self.mark_obj((*c).proto);
                    for i in 0..(*c).upcount {
                        self.mark_obj(*(*c).upvalues.add(i));
                    }
                }
                ObjKind::Upvalue => {
                    let u = o as *mut ObjUpvalue;
                    self.mark_value((*u).closed);
                }
                ObjKind::Native => {
                    let n = o as *mut ObjNative;
                    self.mark_obj((*n).name);
                }
            }
        }
    }

    pub fn sweep(&mut self) {
        let mut prev: *mut Obj = std::ptr::null_mut();
        let mut cur = self.head;
        unsafe {
            while !cur.is_null() {
                if (*cur).marked {
                    (*cur).marked = false;
                    prev = cur;
                    cur = (*cur).next;
                } else {
                    let dead = cur;
                    cur = (*cur).next;
                    if prev.is_null() {
                        self.head = cur;
                    } else {
                        (*prev).next = cur;
                    }
                    free_obj(dead);
                }
            }
        }
        self.next_gc = self.bytes_allocated + INITIAL_NEXT_GC;
    }

    pub fn free_all(&mut self) {
        let mut cur = self.head;
        unsafe {
            while !cur.is_null() {
                let next = (*cur).next;
                free_obj(cur);
                cur = next;
            }
        }
        self.head = std::ptr::null_mut();
        self.temp_roots.clear();
        self.grey.clear();
    }
}

unsafe fn free_obj(o: *mut Obj) {
    match (*o).kind {
        ObjKind::Str => {
            let s = o as *mut ObjStr;
            if !(*s).data.is_null() && (*s).cap > 0 {
                std::alloc::dealloc(
                    (*s).data,
                    std::alloc::Layout::from_size_align((*s).cap, 1).unwrap(),
                );
            }
            drop(Box::from_raw(s));
        }
        ObjKind::Array => {
            let a = o as *mut ObjArray;
            if !(*a).items.is_null() && (*a).cap > 0 {
                std::alloc::dealloc(
                    (*a).items as *mut u8,
                    std::alloc::Layout::array::<Value>((*a).cap).unwrap(),
                );
            }
            drop(Box::from_raw(a));
        }
        ObjKind::Map => {
            let m = o as *mut ObjMap;
            (*m).table.free();
            drop(Box::from_raw(m));
        }
        ObjKind::Proto => {
            drop(Box::from_raw(o as *mut ObjProto));
        }
        ObjKind::Closure => {
            let c = o as *mut ObjClosure;
            if !(*c).upvalues.is_null() && (*c).upcount > 0 {
                std::alloc::dealloc(
                    (*c).upvalues as *mut u8,
                    std::alloc::Layout::array::<*mut Obj>((*c).upcount).unwrap(),
                );
            }
            drop(Box::from_raw(c));
        }
        ObjKind::Upvalue => {
            drop(Box::from_raw(o as *mut ObjUpvalue));
        }
        ObjKind::Native => {
            drop(Box::from_raw(o as *mut ObjNative));
        }
    }
}
