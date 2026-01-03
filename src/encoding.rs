const B64_ALPHA: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const HEX_ALPHA: &[u8; 16] = b"0123456789abcdef";

pub fn hex_encode(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len() * 2);
    for &b in data {
        out.push(HEX_ALPHA[(b >> 4) as usize] as char);
        out.push(HEX_ALPHA[(b & 0x0f) as usize] as char);
    }
    out
}

fn hex_nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

pub fn hex_decode(s: &[u8]) -> Option<Vec<u8>> {
    if s.len() % 2 != 0 {
        return None;
    }
    let mut out = Vec::with_capacity(s.len() / 2);
    let mut i = 0;
    while i < s.len() {
        let hi = hex_nibble(s[i])?;
        let lo = hex_nibble(s[i + 1])?;
        out.push((hi << 4) | lo);
        i += 2;
    }
    Some(out)
}

pub fn base64_encode(data: &[u8]) -> String {
    let mut out = String::new();
    let mut i = 0;
    while i + 3 <= data.len() {
        let n = ((data[i] as u32) << 16) | ((data[i + 1] as u32) << 8) | (data[i + 2] as u32);
        out.push(B64_ALPHA[((n >> 18) & 0x3f) as usize] as char);
        out.push(B64_ALPHA[((n >> 12) & 0x3f) as usize] as char);
        out.push(B64_ALPHA[((n >> 6) & 0x3f) as usize] as char);
        out.push(B64_ALPHA[(n & 0x3f) as usize] as char);
        i += 3;
    }
    let rem = data.len() - i;
    if rem == 1 {
        let n = (data[i] as u32) << 16;
        out.push(B64_ALPHA[((n >> 18) & 0x3f) as usize] as char);
        out.push(B64_ALPHA[((n >> 12) & 0x3f) as usize] as char);
        out.push('=');
        out.push('=');
    } else if rem == 2 {
        let n = ((data[i] as u32) << 16) | ((data[i + 1] as u32) << 8);
        out.push(B64_ALPHA[((n >> 18) & 0x3f) as usize] as char);
        out.push(B64_ALPHA[((n >> 12) & 0x3f) as usize] as char);
        out.push(B64_ALPHA[((n >> 6) & 0x3f) as usize] as char);
        out.push('=');
    }
    out
}

fn base64_value(c: u8) -> Option<u32> {
    match c {
        b'A'..=b'Z' => Some((c - b'A') as u32),
        b'a'..=b'z' => Some((c - b'a' + 26) as u32),
        b'0'..=b'9' => Some((c - b'0' + 52) as u32),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

pub fn base64_decode(s: &[u8]) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    let mut buf: u32 = 0;
    let mut bits: u32 = 0;
    for &c in s {
        if c == b'=' || c == b'\n' || c == b'\r' || c == b' ' || c == b'\t' {
            continue;
        }
        let v = base64_value(c)?;
        buf = (buf << 6) | v;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push(((buf >> bits) & 0xff) as u8);
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_roundtrip() {
        assert_eq!(hex_encode(b"hi"), "6869");
        assert_eq!(hex_decode(b"6869"), Some(b"hi".to_vec()));
        assert_eq!(hex_decode(b"zz"), None);
        assert_eq!(hex_decode(b"abc"), None);
        for s in [&b""[..], b"a", b"ab", b"hello world", &[0u8, 255, 128, 1]] {
            let e = hex_encode(s);
            assert_eq!(hex_decode(e.as_bytes()), Some(s.to_vec()));
        }
    }

    #[test]
    fn base64_roundtrip() {
        assert_eq!(base64_encode(b"abc"), "YWJj");
        assert_eq!(base64_encode(b""), "");
        assert_eq!(base64_decode(b"YWJj"), Some(b"abc".to_vec()));
        for s in [&b""[..], b"a", b"ab", b"abc", b"hello world!", &[0u8, 1, 2, 250]] {
            let e = base64_encode(s);
            assert_eq!(base64_decode(e.as_bytes()), Some(s.to_vec()));
        }
    }
}
