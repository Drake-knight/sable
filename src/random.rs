pub fn step(state: u64) -> u64 {
    let mut x = state;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}

pub fn to_unit(state: u64) -> f64 {
    ((state >> 11) as f64) / ((1u64 << 53) as f64)
}

pub fn bounded(state: u64, span: u64) -> u64 {
    if span == 0 {
        0
    } else {
        state % span
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_and_changes() {
        let s = 12345u64;
        assert_eq!(step(s), step(s));
        assert_ne!(step(s), s);
    }

    #[test]
    fn unit_in_range() {
        let mut s = 1u64;
        for _ in 0..2000 {
            s = step(s);
            let u = to_unit(s);
            assert!(u >= 0.0 && u < 1.0);
        }
    }

    #[test]
    fn bounded_range() {
        assert_eq!(bounded(100, 0), 0);
        for i in 0..200u64 {
            assert!(bounded(i, 7) < 7);
        }
    }
}
