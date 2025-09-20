use crate::object::Obj;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Value(u64);

const QNAN: u64 = 0x7ffc_0000_0000_0000;
const SIGN: u64 = 0x8000_0000_0000_0000;
const TAG_NIL: u64 = 1;
const TAG_FALSE: u64 = 2;
const TAG_TRUE: u64 = 3;
const PTR_MASK: u64 = 0x0000_ffff_ffff_ffff;
const CANON_NAN: u64 = 0x7ff8_0000_0000_0000;

impl Value {
    pub fn nil() -> Value {
        Value(QNAN | TAG_NIL)
    }

    pub fn bool(b: bool) -> Value {
        Value(QNAN | if b { TAG_TRUE } else { TAG_FALSE })
    }

    pub fn number(n: f64) -> Value {
        if n.is_nan() {
            Value(CANON_NAN)
        } else {
            Value(n.to_bits())
        }
    }

    pub fn from_obj(o: *mut Obj) -> Value {
        Value(SIGN | QNAN | (o as u64 & PTR_MASK))
    }

    pub fn bits(self) -> u64 {
        self.0
    }

    pub fn from_bits(b: u64) -> Value {
        Value(b)
    }

    pub fn is_number(self) -> bool {
        (self.0 & QNAN) != QNAN
    }

    pub fn is_nil(self) -> bool {
        self.0 == (QNAN | TAG_NIL)
    }

    pub fn is_bool(self) -> bool {
        self.0 == (QNAN | TAG_FALSE) || self.0 == (QNAN | TAG_TRUE)
    }

    pub fn is_object(self) -> bool {
        (self.0 & (SIGN | QNAN)) == (SIGN | QNAN)
    }

    pub fn as_number(self) -> f64 {
        f64::from_bits(self.0)
    }

    pub fn as_bool(self) -> bool {
        self.0 == (QNAN | TAG_TRUE)
    }

    pub fn as_obj(self) -> *mut Obj {
        (self.0 & PTR_MASK) as *mut Obj
    }

    pub fn is_truthy(self) -> bool {
        !(self.is_nil() || self.0 == (QNAN | TAG_FALSE))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitives() {
        assert!(Value::nil().is_nil());
        assert!(!Value::nil().is_number());
        assert!(Value::bool(true).is_bool());
        assert!(Value::bool(true).as_bool());
        assert!(!Value::bool(false).as_bool());
        let n = Value::number(3.5);
        assert!(n.is_number());
        assert_eq!(n.as_number(), 3.5);
        assert!(!n.is_object());
    }

    #[test]
    fn truthiness() {
        assert!(!Value::nil().is_truthy());
        assert!(!Value::bool(false).is_truthy());
        assert!(Value::bool(true).is_truthy());
        assert!(Value::number(0.0).is_truthy());
        assert!(Value::number(1.0).is_truthy());
    }

    #[test]
    fn nan_and_negatives_are_numbers_not_objects() {
        let nan = Value::number(f64::NAN);
        assert!(nan.is_number());
        assert!(!nan.is_object());
        assert!(nan.as_number().is_nan());
        let neg = Value::number(-98765.4321);
        assert!(neg.is_number());
        assert!(!neg.is_object());
        assert_eq!(neg.as_number(), -98765.4321);
    }

    #[test]
    fn bits_roundtrip() {
        let v = Value::number(42.0);
        assert_eq!(Value::from_bits(v.bits()).as_number(), 42.0);
    }
}
