use crate::chunk::CompiledProto;
use crate::opcode::Op;

pub fn instruction_count(code: &[u8]) -> usize {
    let mut count = 0;
    let mut i = 0;
    while i < code.len() {
        match Op::from_u8(code[i]) {
            Some(op) => {
                count += 1;
                i += 1 + op.operand_bytes();
            }
            None => {
                i += 1;
            }
        }
    }
    count
}

pub fn op_counts(code: &[u8]) -> Vec<(u8, u32)> {
    let mut counts = [0u32; 256];
    let mut i = 0;
    while i < code.len() {
        match Op::from_u8(code[i]) {
            Some(op) => {
                counts[code[i] as usize] += 1;
                i += 1 + op.operand_bytes();
            }
            None => {
                i += 1;
            }
        }
    }
    let mut out = Vec::new();
    for b in 0..256 {
        if counts[b] > 0 {
            out.push((b as u8, counts[b]));
        }
    }
    out
}

pub fn uses_op(code: &[u8], target: Op) -> bool {
    let mut i = 0;
    while i < code.len() {
        match Op::from_u8(code[i]) {
            Some(op) => {
                if op == target {
                    return true;
                }
                i += 1 + op.operand_bytes();
            }
            None => {
                i += 1;
            }
        }
    }
    false
}

pub fn is_leaf(proto: &CompiledProto) -> bool {
    !uses_op(&proto.code, Op::Call)
}

pub fn total_instructions(proto: &CompiledProto) -> usize {
    let mut total = instruction_count(&proto.code);
    for child in &proto.protos {
        total += total_instructions(child);
    }
    total
}

pub fn max_const_index(code: &[u8]) -> Option<u16> {
    let mut max: Option<u16> = None;
    let mut i = 0;
    while i < code.len() {
        match Op::from_u8(code[i]) {
            Some(op) => {
                if op == Op::LoadConst && i + 2 < code.len() {
                    let idx = (code[i + 1] as u16) | ((code[i + 2] as u16) << 8);
                    max = Some(match max {
                        Some(m) if m >= idx => m,
                        _ => idx,
                    });
                }
                i += 1 + op.operand_bytes();
            }
            None => {
                i += 1;
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_instructions() {
        let code = vec![
            Op::LoadNil as u8,
            Op::LoadTrue as u8,
            Op::Pop as u8,
            Op::Return as u8,
        ];
        assert_eq!(instruction_count(&code), 4);
    }

    #[test]
    fn detects_ops_and_const_index() {
        let code = vec![Op::LoadConst as u8, 5, 0, Op::Return as u8];
        assert!(uses_op(&code, Op::LoadConst));
        assert!(!uses_op(&code, Op::Call));
        assert_eq!(max_const_index(&code), Some(5));
    }

    #[test]
    fn tolerates_trailing_truncated_operand() {
        let code = vec![Op::LoadConst as u8];
        let _ = instruction_count(&code);
        let _ = max_const_index(&code);
    }
}
