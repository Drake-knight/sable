const ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

pub fn encode(data: &[u8]) -> String {
    if data.is_empty() {
        return String::new();
    }
    let mut output = String::new();
    let mut buffer: u32 = 0;
    let mut bits_in_buffer: u32 = 0;
    for &byte in data {
        buffer = (buffer << 8) | byte as u32;
        bits_in_buffer += 8;
        while bits_in_buffer >= 5 {
            bits_in_buffer -= 5;
            let index = ((buffer >> bits_in_buffer) & 0x1F) as usize;
            output.push(ALPHABET[index] as char);
        }
    }
    if bits_in_buffer > 0 {
        let index = ((buffer << (5 - bits_in_buffer)) & 0x1F) as usize;
        output.push(ALPHABET[index] as char);
    }
    let remainder = output.len() % 8;
    if remainder != 0 {
        for _ in 0..(8 - remainder) {
            output.push('=');
        }
    }
    output
}

fn value_for_char(c: u8) -> Option<u32> {
    match c {
        b'A'..=b'Z' => Some((c - b'A') as u32),
        b'a'..=b'z' => Some((c - b'a') as u32),
        b'2'..=b'7' => Some((c - b'2') as u32 + 26),
        _ => None,
    }
}

pub fn decode(data: &[u8]) -> Option<Vec<u8>> {
    let mut output = Vec::new();
    let mut buffer: u32 = 0;
    let mut bits_in_buffer: u32 = 0;
    for &c in data {
        if c == b'=' || c.is_ascii_whitespace() {
            continue;
        }
        let value = value_for_char(c)?;
        buffer = (buffer << 5) | value;
        bits_in_buffer += 5;
        if bits_in_buffer >= 8 {
            bits_in_buffer -= 8;
            let byte = ((buffer >> bits_in_buffer) & 0xFF) as u8;
            output.push(byte);
        }
    }
    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_vectors() {
        assert_eq!(encode(b"foobar"), "MZXW6YTBOI======");
        assert_eq!(encode(b""), "");
        assert_eq!(encode(b"f"), "MY======");
    }

    #[test]
    fn round_trip_lengths_0_to_10() {
        for len in 0..=10usize {
            let data: Vec<u8> = (0..len).map(|i| i as u8).collect();
            let encoded = encode(&data);
            assert_eq!(encoded.len() % 8, 0);
            let decoded = decode(encoded.as_bytes());
            assert_eq!(decoded, Some(data));
        }
    }

    #[test]
    fn decode_is_case_insensitive() {
        let upper = encode(b"foobar");
        let lower = upper.to_ascii_lowercase();
        assert_eq!(decode(lower.as_bytes()), Some(b"foobar".to_vec()));
        assert_eq!(decode(upper.as_bytes()), Some(b"foobar".to_vec()));
    }

    #[test]
    fn decode_ignores_whitespace_and_padding() {
        let spaced = b"MZXW 6YTB\tOI=\n=====";
        assert_eq!(decode(spaced), Some(b"foobar".to_vec()));
    }

    #[test]
    fn decode_rejects_invalid_characters() {
        assert_eq!(decode(b"MZXW6YTB0I======"), None);
        assert_eq!(decode(b"!!!!"), None);
        assert_eq!(decode(b"01189998819991197253"), None);
    }

    #[test]
    fn decode_handles_empty_input() {
        assert_eq!(decode(b""), Some(Vec::new()));
        assert_eq!(decode(b"        "), Some(Vec::new()));
        assert_eq!(decode(b"========"), Some(Vec::new()));
    }
}
