#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Op {
    LoadConst,
    LoadNil,
    LoadTrue,
    LoadFalse,
    LoadInt,
    Pop,
    Dup,
    Swap,
    GetLocal,
    SetLocal,
    GetGlobal,
    SetGlobal,
    DefineGlobal,
    GetUpvalue,
    SetUpvalue,
    CloseUpvalue,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,
    Not,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    NewArray,
    NewMap,
    GetIndex,
    SetIndex,
    Len,
    Jump,
    JumpIfFalse,
    Loop,
    Call,
    Return,
    Closure,
}

impl Op {
    pub fn from_u8(b: u8) -> Option<Op> {
        let op = match b {
            0 => Op::LoadConst,
            1 => Op::LoadNil,
            2 => Op::LoadTrue,
            3 => Op::LoadFalse,
            4 => Op::LoadInt,
            5 => Op::Pop,
            6 => Op::Dup,
            7 => Op::Swap,
            8 => Op::GetLocal,
            9 => Op::SetLocal,
            10 => Op::GetGlobal,
            11 => Op::SetGlobal,
            12 => Op::DefineGlobal,
            13 => Op::GetUpvalue,
            14 => Op::SetUpvalue,
            15 => Op::CloseUpvalue,
            16 => Op::Add,
            17 => Op::Sub,
            18 => Op::Mul,
            19 => Op::Div,
            20 => Op::Mod,
            21 => Op::Neg,
            22 => Op::Not,
            23 => Op::Eq,
            24 => Op::Ne,
            25 => Op::Lt,
            26 => Op::Le,
            27 => Op::Gt,
            28 => Op::Ge,
            29 => Op::NewArray,
            30 => Op::NewMap,
            31 => Op::GetIndex,
            32 => Op::SetIndex,
            33 => Op::Len,
            34 => Op::Jump,
            35 => Op::JumpIfFalse,
            36 => Op::Loop,
            37 => Op::Call,
            38 => Op::Return,
            39 => Op::Closure,
            _ => return None,
        };
        Some(op)
    }

    pub fn operand_bytes(self) -> usize {
        match self {
            Op::LoadConst
            | Op::LoadInt
            | Op::GetGlobal
            | Op::SetGlobal
            | Op::DefineGlobal
            | Op::NewArray
            | Op::NewMap
            | Op::Jump
            | Op::JumpIfFalse
            | Op::Loop
            | Op::Closure => 2,
            Op::GetLocal | Op::SetLocal | Op::GetUpvalue | Op::SetUpvalue | Op::Call => 1,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u8_roundtrip() {
        for b in 0u8..=39 {
            let op = Op::from_u8(b).unwrap();
            assert_eq!(op as u8, b);
        }
        assert!(Op::from_u8(40).is_none());
        assert!(Op::from_u8(200).is_none());
        assert!(Op::from_u8(255).is_none());
    }

    #[test]
    fn operand_widths() {
        assert_eq!(Op::LoadConst.operand_bytes(), 2);
        assert_eq!(Op::Jump.operand_bytes(), 2);
        assert_eq!(Op::Closure.operand_bytes(), 2);
        assert_eq!(Op::GetLocal.operand_bytes(), 1);
        assert_eq!(Op::Call.operand_bytes(), 1);
        assert_eq!(Op::Return.operand_bytes(), 0);
        assert_eq!(Op::Add.operand_bytes(), 0);
    }
}
