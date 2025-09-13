use crate::error::{SableError, SableResult};

const MAGIC: &[u8; 4] = b"SABL";
const VERSION: u16 = 1;
const MAX_PROTO_DEPTH: u32 = 128;

#[derive(Clone)]
pub enum Konst {
    Num(f64),
    Str(String),
    Bool(bool),
    Nil,
}

#[derive(Clone, Copy)]
pub struct UpDesc {
    pub is_local: bool,
    pub index: u8,
}

#[derive(Clone)]
pub struct CompiledProto {
    pub arity: u8,
    pub upvals: u8,
    pub max_stack: u16,
    pub code: Vec<u8>,
    pub consts: Vec<Konst>,
    pub protos: Vec<CompiledProto>,
    pub upinfo: Vec<UpDesc>,
    pub name: String,
}

impl CompiledProto {
    pub fn new(name: String, arity: u8) -> CompiledProto {
        CompiledProto {
            arity,
            upvals: 0,
            max_stack: 0,
            code: Vec::new(),
            consts: Vec::new(),
            protos: Vec::new(),
            upinfo: Vec::new(),
            name,
        }
    }
}

pub fn to_bytes(p: &CompiledProto) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(MAGIC);
    write_u16(&mut out, VERSION);
    write_u16(&mut out, 0);
    write_proto(&mut out, p);
    out
}

fn write_u16(out: &mut Vec<u8>, v: u16) {
    out.push((v & 0xff) as u8);
    out.push((v >> 8) as u8);
}

fn write_u32(out: &mut Vec<u8>, v: u32) {
    out.push((v & 0xff) as u8);
    out.push(((v >> 8) & 0xff) as u8);
    out.push(((v >> 16) & 0xff) as u8);
    out.push(((v >> 24) & 0xff) as u8);
}

fn write_proto(out: &mut Vec<u8>, p: &CompiledProto) {
    out.push(p.arity);
    out.push(p.upvals);
    write_u16(out, p.max_stack);
    write_u32(out, p.code.len() as u32);
    out.extend_from_slice(&p.code);
    write_u32(out, p.consts.len() as u32);
    for k in &p.consts {
        write_konst(out, k);
    }
    write_u32(out, p.protos.len() as u32);
    for c in &p.protos {
        write_proto(out, c);
    }
    out.push(p.upinfo.len() as u8);
    for u in &p.upinfo {
        out.push(u.is_local as u8);
        out.push(u.index);
    }
    write_u32(out, p.name.len() as u32);
    out.extend_from_slice(p.name.as_bytes());
}

fn write_konst(out: &mut Vec<u8>, k: &Konst) {
    match k {
        Konst::Num(n) => {
            out.push(0);
            let bits = n.to_bits();
            for i in 0..8 {
                out.push(((bits >> (i * 8)) & 0xff) as u8);
            }
        }
        Konst::Str(s) => {
            out.push(1);
            write_u32(out, s.len() as u32);
            out.extend_from_slice(s.as_bytes());
        }
        Konst::Bool(b) => {
            out.push(2);
            out.push(*b as u8);
        }
        Konst::Nil => {
            out.push(3);
        }
    }
}

struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn u8(&mut self) -> SableResult<u8> {
        if self.pos >= self.buf.len() {
            return Err(SableError::Load("truncated chunk".to_string()));
        }
        let b = self.buf[self.pos];
        self.pos += 1;
        Ok(b)
    }

    fn u16(&mut self) -> SableResult<u16> {
        let a = self.u8()? as u16;
        let b = self.u8()? as u16;
        Ok(a | (b << 8))
    }

    fn u32(&mut self) -> SableResult<u32> {
        let a = self.u16()? as u32;
        let b = self.u16()? as u32;
        Ok(a | (b << 16))
    }

    fn take(&mut self, n: usize) -> SableResult<&'a [u8]> {
        if self.pos > self.buf.len() || n > self.buf.len() - self.pos {
            return Err(SableError::Load("truncated chunk".to_string()));
        }
        let s = &self.buf[self.pos..self.pos + n];
        self.pos += n;
        Ok(s)
    }

    fn f64(&mut self) -> SableResult<f64> {
        let mut bits: u64 = 0;
        for i in 0..8 {
            bits |= (self.u8()? as u64) << (i * 8);
        }
        Ok(f64::from_bits(bits))
    }
}

pub fn from_bytes(buf: &[u8]) -> SableResult<CompiledProto> {
    let mut r = Reader { buf, pos: 0 };
    let magic = r.take(4)?;
    if magic != MAGIC {
        return Err(SableError::Load("bad magic".to_string()));
    }
    let ver = r.u16()?;
    if ver != VERSION {
        return Err(SableError::Load("bad version".to_string()));
    }
    let _flags = r.u16()?;
    read_proto(&mut r, 0)
}

fn read_proto(r: &mut Reader, depth: u32) -> SableResult<CompiledProto> {
    if depth > MAX_PROTO_DEPTH {
        return Err(SableError::Load("proto nesting too deep".to_string()));
    }
    let arity = r.u8()?;
    let upvals = r.u8()?;
    let max_stack = r.u16()?;
    let code_len = r.u32()? as usize;
    let code = r.take(code_len)?.to_vec();
    let const_count = r.u32()? as usize;
    let mut consts = Vec::new();
    for _ in 0..const_count {
        consts.push(read_konst(r)?);
    }
    let proto_count = r.u32()? as usize;
    let mut protos = Vec::new();
    for _ in 0..proto_count {
        protos.push(read_proto(r, depth + 1)?);
    }
    let upinfo_count = r.u8()? as usize;
    let mut upinfo = Vec::new();
    for _ in 0..upinfo_count {
        let is_local = r.u8()? != 0;
        let index = r.u8()?;
        upinfo.push(UpDesc { is_local, index });
    }
    let name_len = r.u32()? as usize;
    let name_bytes = r.take(name_len)?;
    let name = String::from_utf8_lossy(name_bytes).into_owned();
    Ok(CompiledProto {
        arity,
        upvals,
        max_stack,
        code,
        consts,
        protos,
        upinfo,
        name,
    })
}

fn read_konst(r: &mut Reader) -> SableResult<Konst> {
    let tag = r.u8()?;
    match tag {
        0 => Ok(Konst::Num(r.f64()?)),
        1 => {
            let len = r.u32()? as usize;
            let bytes = r.take(len)?;
            Ok(Konst::Str(String::from_utf8_lossy(bytes).into_owned()))
        }
        2 => Ok(Konst::Bool(r.u8()? != 0)),
        3 => Ok(Konst::Nil),
        _ => Err(SableError::Load("bad constant tag".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_simple() {
        let mut p = CompiledProto::new("main".to_string(), 0);
        p.code = vec![0, 1, 2, 38];
        p.consts = vec![
            Konst::Num(3.5),
            Konst::Str("hi".to_string()),
            Konst::Bool(true),
            Konst::Nil,
        ];
        p.max_stack = 16;
        let bytes = to_bytes(&p);
        let back = from_bytes(&bytes).unwrap();
        assert_eq!(back.code, p.code);
        assert_eq!(back.consts.len(), 4);
        assert_eq!(back.name, "main");
        assert_eq!(back.max_stack, 16);
    }

    #[test]
    fn rejects_malformed() {
        assert!(from_bytes(b"XXXX").is_err());
        assert!(from_bytes(b"").is_err());
        assert!(from_bytes(b"SABL").is_err());
    }

    #[test]
    fn nested_protos_roundtrip() {
        let mut child = CompiledProto::new("inner".to_string(), 1);
        child.code = vec![38];
        let mut p = CompiledProto::new("main".to_string(), 0);
        p.protos = vec![child];
        let bytes = to_bytes(&p);
        let back = from_bytes(&bytes).unwrap();
        assert_eq!(back.protos.len(), 1);
        assert_eq!(back.protos[0].name, "inner");
        assert_eq!(back.protos[0].arity, 1);
    }
}
