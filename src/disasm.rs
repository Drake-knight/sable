use crate::chunk::CompiledProto;
use crate::chunk::Konst;
use crate::opcode::Op;

pub fn disassemble_proto(proto: &CompiledProto) -> String {
    let mut out = String::new();
    append_proto(proto, "", &mut out);
    out
}

fn append_proto(proto: &CompiledProto, prefix: &str, out: &mut String) {
    out.push_str(prefix);
    out.push_str("proto ");
    if proto.name.is_empty() {
        out.push_str("<anonymous>");
    } else {
        out.push_str(&proto.name);
    }
    out.push_str(&format!(
        " arity={} upvals={} max_stack={} consts={} protos={} upinfo={} code_bytes={}\n",
        proto.arity,
        proto.upvals,
        proto.max_stack,
        proto.consts.len(),
        proto.protos.len(),
        proto.upinfo.len(),
        proto.code.len()
    ));

    for line in disassemble_code_lines(&proto.code) {
        out.push_str(prefix);
        out.push_str(&line);
        out.push('\n');
    }

    out.push_str(prefix);
    out.push_str("constants:\n");
    if proto.consts.is_empty() {
        out.push_str(prefix);
        out.push_str("  (none)\n");
    } else {
        for (i, k) in proto.consts.iter().enumerate() {
            out.push_str(prefix);
            out.push_str(&format!("  {:04} {}\n", i, render_konst(k)));
        }
    }

    if !proto.protos.is_empty() {
        out.push_str(prefix);
        out.push_str("nested protos:\n");
        let mut child_prefix = String::from(prefix);
        child_prefix.push_str("  ");
        for (i, child) in proto.protos.iter().enumerate() {
            out.push_str(prefix);
            out.push_str(&format!("  -- proto {} --\n", i));
            append_proto(child, &child_prefix, out);
        }
    }
}

pub fn disassemble_code(code: &[u8]) -> String {
    let mut out = String::new();
    for line in disassemble_code_lines(code) {
        out.push_str(&line);
        out.push('\n');
    }
    out
}

fn disassemble_code_lines(code: &[u8]) -> Vec<String> {
    let mut lines = Vec::new();
    let len = code.len();
    let mut ip: usize = 0;
    while ip < len {
        let byte = code[ip];
        let op = match Op::from_u8(byte) {
            Some(op) => op,
            None => {
                lines.push(format!("{:04} .byte {:02X}", ip, byte));
                ip += 1;
                continue;
            }
        };
        let name = op_name(op);
        let operand_len = op.operand_bytes();
        if operand_len == 0 {
            lines.push(format!("{:04} {}", ip, name));
            ip += 1;
        } else if operand_len == 1 {
            if ip + 1 < len {
                let operand = code[ip + 1];
                lines.push(format!("{:04} {} {}", ip, name, operand));
                ip += 2;
            } else {
                lines.push(format!("{:04} {} <truncated>", ip, name));
                ip = len;
            }
        } else if operand_len == 2 {
            if ip + 2 < len {
                let lo = code[ip + 1] as u16;
                let hi = code[ip + 2] as u16;
                let operand = lo | (hi << 8);
                lines.push(format!("{:04} {} {}", ip, name, operand));
                ip += 3;
            } else {
                lines.push(format!("{:04} {} <truncated>", ip, name));
                ip = len;
            }
        } else {
            lines.push(format!("{:04} {} <unsupported>", ip, name));
            ip += 1;
        }
    }
    lines
}

fn op_name(op: Op) -> &'static str {
    match op {
        Op::LoadConst => "LOAD_CONST",
        Op::LoadNil => "LOAD_NIL",
        Op::LoadTrue => "LOAD_TRUE",
        Op::LoadFalse => "LOAD_FALSE",
        Op::LoadInt => "LOAD_INT",
        Op::Pop => "POP",
        Op::Dup => "DUP",
        Op::Swap => "SWAP",
        Op::GetLocal => "GET_LOCAL",
        Op::SetLocal => "SET_LOCAL",
        Op::GetGlobal => "GET_GLOBAL",
        Op::SetGlobal => "SET_GLOBAL",
        Op::DefineGlobal => "DEFINE_GLOBAL",
        Op::GetUpvalue => "GET_UPVALUE",
        Op::SetUpvalue => "SET_UPVALUE",
        Op::CloseUpvalue => "CLOSE_UPVALUE",
        Op::Add => "ADD",
        Op::Sub => "SUB",
        Op::Mul => "MUL",
        Op::Div => "DIV",
        Op::Mod => "MOD",
        Op::Neg => "NEG",
        Op::Not => "NOT",
        Op::Eq => "EQ",
        Op::Ne => "NE",
        Op::Lt => "LT",
        Op::Le => "LE",
        Op::Gt => "GT",
        Op::Ge => "GE",
        Op::NewArray => "NEW_ARRAY",
        Op::NewMap => "NEW_MAP",
        Op::GetIndex => "GET_INDEX",
        Op::SetIndex => "SET_INDEX",
        Op::Len => "LEN",
        Op::Jump => "JUMP",
        Op::JumpIfFalse => "JUMP_IF_FALSE",
        Op::Loop => "LOOP",
        Op::Call => "CALL",
        Op::Return => "RETURN",
        Op::Closure => "CLOSURE",
    }
}

fn render_konst(k: &Konst) -> String {
    match k {
        Konst::Num(n) => format!("Num({})", n),
        Konst::Str(s) => format!("Str(\"{}\")", short_escape(s)),
        Konst::Bool(b) => format!("Bool({})", b),
        Konst::Nil => "Nil".to_string(),
    }
}

fn short_escape(s: &str) -> String {
    let max_chars = 40;
    let mut out = String::new();
    let mut count = 0usize;
    for ch in s.chars() {
        if count >= max_chars {
            out.push_str("...");
            break;
        }
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            other => {
                let code_point = other as u32;
                if code_point >= 0x20 && code_point < 0x7f {
                    out.push(other);
                } else {
                    out.push('?');
                }
            }
        }
        count += 1;
    }
    out
}
