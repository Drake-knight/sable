use crate::heap::Heap;
use crate::object::{ObjKind, ObjStr};
use crate::value::Value;

#[derive(Clone, Copy)]
pub struct Entry {
    pub key: Value,
    pub value: Value,
    pub used: bool,
}

#[repr(C)]
pub struct Table {
    pub count: usize,
    pub cap: usize,
    pub entries: *mut Entry,
}

impl Table {
    pub fn new() -> Table {
        Table {
            count: 0,
            cap: 0,
            entries: std::ptr::null_mut(),
        }
    }

    pub fn get(&self, key: Value) -> Value {
        if self.cap == 0 {
            return Value::nil();
        }
        unsafe {
            let mask = self.cap - 1;
            let mut i = (hash_value(key) as usize) & mask;
            loop {
                let e = self.entries.add(i);
                if !(*e).used {
                    return Value::nil();
                }
                if key_eq((*e).key, key) {
                    return (*e).value;
                }
                i = (i + 1) & mask;
            }
        }
    }

    pub fn has(&self, key: Value) -> bool {
        if self.cap == 0 {
            return false;
        }
        unsafe {
            let mask = self.cap - 1;
            let mut i = (hash_value(key) as usize) & mask;
            loop {
                let e = self.entries.add(i);
                if !(*e).used {
                    return false;
                }
                if key_eq((*e).key, key) {
                    return true;
                }
                i = (i + 1) & mask;
            }
        }
    }

    pub fn set(&mut self, key: Value, value: Value) {
        if (self.count + 1) * 4 > self.cap * 3 {
            self.grow();
        }
        unsafe {
            let mask = self.cap - 1;
            let mut i = (hash_value(key) as usize) & mask;
            loop {
                let e = self.entries.add(i);
                if !(*e).used {
                    (*e).used = true;
                    (*e).key = key;
                    (*e).value = value;
                    self.count += 1;
                    return;
                }
                if key_eq((*e).key, key) {
                    (*e).value = value;
                    return;
                }
                i = (i + 1) & mask;
            }
        }
    }

    fn grow(&mut self) {
        let new_cap = if self.cap == 0 { 8 } else { self.cap * 2 };
        let layout = std::alloc::Layout::array::<Entry>(new_cap).unwrap();
        let new_entries = unsafe { std::alloc::alloc(layout) as *mut Entry };
        for i in 0..new_cap {
            unsafe {
                *new_entries.add(i) = Entry {
                    key: Value::nil(),
                    value: Value::nil(),
                    used: false,
                };
            }
        }
        let old = self.entries;
        let old_cap = self.cap;
        self.entries = new_entries;
        self.cap = new_cap;
        self.count = 0;
        if !old.is_null() {
            unsafe {
                for i in 0..old_cap {
                    let e = old.add(i);
                    if (*e).used {
                        self.set((*e).key, (*e).value);
                    }
                }
                std::alloc::dealloc(
                    old as *mut u8,
                    std::alloc::Layout::array::<Entry>(old_cap).unwrap(),
                );
            }
        }
    }

    pub fn free(&mut self) {
        if !self.entries.is_null() && self.cap > 0 {
            unsafe {
                std::alloc::dealloc(
                    self.entries as *mut u8,
                    std::alloc::Layout::array::<Entry>(self.cap).unwrap(),
                );
            }
            self.entries = std::ptr::null_mut();
            self.cap = 0;
            self.count = 0;
        }
    }

    pub fn mark(&self, heap: &mut Heap) {
        if self.entries.is_null() {
            return;
        }
        unsafe {
            for i in 0..self.cap {
                let e = self.entries.add(i);
                if (*e).used {
                    heap.mark_value((*e).key);
                    heap.mark_value((*e).value);
                }
            }
        }
    }
}

pub fn hash_value(v: Value) -> u64 {
    if v.is_object() {
        let o = v.as_obj();
        unsafe {
            if (*o).kind == ObjKind::Str {
                let s = o as *mut ObjStr;
                return hash_bytes(std::slice::from_raw_parts((*s).data, (*s).len));
            }
        }
    }
    let mut h = v.bits();
    h ^= h >> 33;
    h = h.wrapping_mul(0xff51afd7ed558ccd);
    h ^= h >> 33;
    h
}

pub fn key_eq(a: Value, b: Value) -> bool {
    if a.bits() == b.bits() {
        return true;
    }
    if a.is_object() && b.is_object() {
        let oa = a.as_obj();
        let ob = b.as_obj();
        unsafe {
            if (*oa).kind == ObjKind::Str && (*ob).kind == ObjKind::Str {
                let sa = oa as *mut ObjStr;
                let sb = ob as *mut ObjStr;
                if (*sa).len != (*sb).len {
                    return false;
                }
                let ba = std::slice::from_raw_parts((*sa).data, (*sa).len);
                let bb = std::slice::from_raw_parts((*sb).data, (*sb).len);
                return ba == bb;
            }
        }
    }
    false
}

fn hash_bytes(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}
