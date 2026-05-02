pub fn encode(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len());
    for &b in data {
        if is_unreserved(b) {
            out.push(b as char);
        } else {
            out.push('%');
            out.push(hex_digit_upper(b >> 4) as char);
            out.push(hex_digit_upper(b & 0x0F) as char);
        }
    }
    out
}

pub fn decode(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len());
    let mut i = 0;
    while i < data.len() {
        let b = data[i];
        if b == b'+' {
            out.push(b' ');
            i += 1;
        } else if b == b'%' {
            let h1 = data.get(i + 1).copied().and_then(hex_val);
            let h2 = data.get(i + 2).copied().and_then(hex_val);
            match (h1, h2) {
                (Some(h1), Some(h2)) => {
                    out.push((h1 << 4) | h2);
                    i += 3;
                }
                _ => {
                    out.push(b'%');
                    i += 1;
                }
            }
        } else {
            out.push(b);
            i += 1;
        }
    }
    out
}

pub fn parse_query(data: &[u8]) -> Vec<(String, String)> {
    let mut result = Vec::new();
    for pair in data.split(|&b| b == b'&') {
        if pair.is_empty() {
            continue;
        }
        let (raw_key, raw_value) = match pair.iter().position(|&b| b == b'=') {
            Some(pos) => (&pair[..pos], &pair[pos + 1..]),
            None => (pair, &pair[pair.len()..]),
        };
        let key = String::from_utf8_lossy(&decode(raw_key)).into_owned();
        let value = String::from_utf8_lossy(&decode(raw_value)).into_owned();
        result.push((key, value));
    }
    result
}

pub fn build_query(pairs: &[(String, String)]) -> String {
    let mut out = String::new();
    for (i, (k, v)) in pairs.iter().enumerate() {
        if i > 0 {
            out.push('&');
        }
        out.push_str(&encode(k.as_bytes()));
        out.push('=');
        out.push_str(&encode(v.as_bytes()));
    }
    out
}

fn is_unreserved(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.' || b == b'~'
}

fn hex_digit_upper(n: u8) -> u8 {
    const DIGITS: &[u8; 16] = b"0123456789ABCDEF";
    DIGITS[(n & 0x0F) as usize]
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_leaves_unreserved_untouched() {
        let data = b"abcXYZ019-_.~";
        assert_eq!(encode(data), "abcXYZ019-_.~");
    }

    #[test]
    fn encode_percent_escapes_space_and_reserved() {
        assert_eq!(encode(b" "), "%20");
        assert_eq!(encode(b"a b"), "a%20b");
        assert_eq!(encode(b"a=b&c"), "a%3Db%26c");
        assert_eq!(encode(b"100%"), "100%25");
    }

    #[test]
    fn decode_reverses_encode_for_reserved_chars() {
        let data = b":/?#[]@!$&'()*+,;=";
        let encoded = encode(data);
        assert_eq!(decode(encoded.as_bytes()), data.to_vec());
    }

    #[test]
    fn decode_hex_is_case_insensitive() {
        assert_eq!(decode(b"%2f"), b"/".to_vec());
        assert_eq!(decode(b"%2F"), b"/".to_vec());
        assert_eq!(decode(b"%aA"), vec![0xAA]);
    }

    #[test]
    fn decode_plus_becomes_space() {
        assert_eq!(decode(b"a+b+c"), b"a b c".to_vec());
    }

    #[test]
    fn decode_malformed_percent_passes_through() {
        assert_eq!(decode(b"%"), b"%".to_vec());
        assert_eq!(decode(b"%2"), b"%2".to_vec());
        assert_eq!(decode(b"%2z"), b"%2z".to_vec());
        assert_eq!(decode(b"100%"), b"100%".to_vec());
        assert_eq!(decode(b"%%41"), b"%A".to_vec());
    }

    #[test]
    fn roundtrip_bytes_with_spaces_and_reserved() {
        let data = b"hello world? a=b&c=d #frag";
        let encoded = encode(data);
        let decoded = decode(encoded.as_bytes());
        assert_eq!(decoded, data.to_vec());
    }

    #[test]
    fn roundtrip_unicode_bytes() {
        let data = "h\u{e9}llo w\u{f6}rld \u{65e5}\u{672c}\u{8a9e}".as_bytes();
        let encoded = encode(data);
        assert!(encoded.bytes().all(|b| b.is_ascii()));
        let decoded = decode(encoded.as_bytes());
        assert_eq!(decoded, data.to_vec());
    }

    #[test]
    fn roundtrip_arbitrary_binary_bytes() {
        let data: &[u8] = &[0, 1, 2, 9, 10, 13, 31, 32, 65, 127, 128, 200, 255];
        let encoded = encode(data);
        let decoded = decode(encoded.as_bytes());
        assert_eq!(decoded, data.to_vec());
    }

    #[test]
    fn parse_query_basic_example() {
        let result = parse_query(b"a=1&b=hello%20world&c");
        assert_eq!(
            result,
            vec![
                ("a".to_string(), "1".to_string()),
                ("b".to_string(), "hello world".to_string()),
                ("c".to_string(), "".to_string()),
            ]
        );
    }

    #[test]
    fn parse_query_empty_input() {
        let result = parse_query(b"");
        assert_eq!(result, Vec::<(String, String)>::new());
    }

    #[test]
    fn parse_query_ignores_stray_ampersands() {
        let result = parse_query(b"a=1&&b=2&");
        assert_eq!(
            result,
            vec![
                ("a".to_string(), "1".to_string()),
                ("b".to_string(), "2".to_string()),
            ]
        );
    }

    #[test]
    fn build_query_matches_expected_format() {
        let pairs = vec![
            ("a".to_string(), "1".to_string()),
            ("b".to_string(), "hello world".to_string()),
            ("c".to_string(), "".to_string()),
        ];
        assert_eq!(build_query(&pairs), "a=1&b=hello%20world&c=");
        assert_eq!(build_query(&[]), "");
    }

    #[test]
    fn build_query_then_parse_query_roundtrip() {
        let pairs = vec![
            ("key one".to_string(), "value&one=".to_string()),
            ("k2".to_string(), "".to_string()),
            ("k3".to_string(), "100%".to_string()),
        ];
        let built = build_query(&pairs);
        let parsed = parse_query(built.as_bytes());
        assert_eq!(parsed, pairs);
    }
}
