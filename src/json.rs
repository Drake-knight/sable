#[derive(Clone, Debug)]
pub enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

const MAX_DEPTH: usize = 128;

struct Parser<'a> {
    input: &'a [u8],
    pos: usize,
    depth: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<u8> {
        self.input.get(self.pos).copied()
    }

    fn bump(&mut self) -> Option<u8> {
        let c = self.peek();
        if c.is_some() {
            self.pos += 1;
        }
        c
    }

    fn skip_ws(&mut self) {
        while let Some(c) = self.peek() {
            if c == b' ' || c == b'\t' || c == b'\n' || c == b'\r' {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn expect_literal(&mut self, lit: &[u8]) -> Result<(), String> {
        if self.pos + lit.len() > self.input.len() {
            return Err(format!("unexpected end of input at position {}", self.pos));
        }
        if &self.input[self.pos..self.pos + lit.len()] == lit {
            self.pos += lit.len();
            Ok(())
        } else {
            Err(format!("invalid literal at position {}", self.pos))
        }
    }

    fn parse_value(&mut self) -> Result<Json, String> {
        self.skip_ws();
        match self.peek() {
            Some(b'n') => self.parse_null(),
            Some(b't') => self.parse_true(),
            Some(b'f') => self.parse_false(),
            Some(b'"') => self.parse_string().map(Json::Str),
            Some(b'[') => self.parse_array(),
            Some(b'{') => self.parse_object(),
            Some(b'-') => self.parse_number(),
            Some(b) if b.is_ascii_digit() => self.parse_number(),
            Some(b) => Err(format!("unexpected byte {} at position {}", b, self.pos)),
            None => Err("unexpected end of input".to_string()),
        }
    }

    fn parse_null(&mut self) -> Result<Json, String> {
        self.expect_literal(b"null")?;
        Ok(Json::Null)
    }

    fn parse_true(&mut self) -> Result<Json, String> {
        self.expect_literal(b"true")?;
        Ok(Json::Bool(true))
    }

    fn parse_false(&mut self) -> Result<Json, String> {
        self.expect_literal(b"false")?;
        Ok(Json::Bool(false))
    }

    fn scan_digits(&mut self) -> bool {
        let mut any = false;
        while let Some(d) = self.peek() {
            if d.is_ascii_digit() {
                self.pos += 1;
                any = true;
            } else {
                break;
            }
        }
        any
    }

    fn parse_number(&mut self) -> Result<Json, String> {
        let start = self.pos;
        if self.peek() == Some(b'-') {
            self.pos += 1;
        }
        match self.peek() {
            Some(b'0') => {
                self.pos += 1;
            }
            Some(d) if d.is_ascii_digit() => {
                self.scan_digits();
            }
            _ => return Err(format!("invalid number at position {}", start)),
        }
        if self.peek() == Some(b'.') {
            self.pos += 1;
            if !self.scan_digits() {
                return Err(format!("invalid number at position {}", start));
            }
        }
        if matches!(self.peek(), Some(b'e') | Some(b'E')) {
            self.pos += 1;
            if matches!(self.peek(), Some(b'+') | Some(b'-')) {
                self.pos += 1;
            }
            if !self.scan_digits() {
                return Err(format!("invalid number at position {}", start));
            }
        }
        let slice = &self.input[start..self.pos];
        let text = match std::str::from_utf8(slice) {
            Ok(text) => text,
            Err(_) => return Err(format!("invalid number encoding at position {}", start)),
        };
        match text.parse::<f64>() {
            Ok(n) => Ok(Json::Num(n)),
            Err(_) => Err(format!("invalid number at position {}", start)),
        }
    }

    fn parse_string(&mut self) -> Result<String, String> {
        match self.bump() {
            Some(b'"') => {}
            _ => return Err(format!("expected string at position {}", self.pos)),
        }
        let mut out = String::new();
        loop {
            match self.bump() {
                None => return Err("unterminated string".to_string()),
                Some(b'"') => return Ok(out),
                Some(b'\\') => self.parse_escape(&mut out)?,
                Some(b) if b < 0x20 => {
                    return Err(format!("control character in string at position {}", self.pos));
                }
                Some(b) if b < 0x80 => out.push(b as char),
                Some(b) => self.parse_utf8_tail(b, &mut out)?,
            }
        }
    }

    fn parse_escape(&mut self, out: &mut String) -> Result<(), String> {
        match self.bump() {
            None => return Err("unterminated escape sequence".to_string()),
            Some(b'"') => out.push('"'),
            Some(b'\\') => out.push('\\'),
            Some(b'/') => out.push('/'),
            Some(b'b') => out.push('\u{0008}'),
            Some(b'f') => out.push('\u{000C}'),
            Some(b'n') => out.push('\n'),
            Some(b'r') => out.push('\r'),
            Some(b't') => out.push('\t'),
            Some(b'u') => {
                let ch = self.parse_unicode_escape()?;
                out.push(ch);
            }
            Some(_) => return Err(format!("invalid escape sequence at position {}", self.pos)),
        }
        Ok(())
    }

    fn parse_unicode_escape(&mut self) -> Result<char, String> {
        let cp = self.parse_hex4()?;
        if cp >= 0xD800 && cp <= 0xDBFF {
            if self.peek() == Some(b'\\') {
                let checkpoint = self.pos;
                self.pos += 1;
                if self.peek() == Some(b'u') {
                    self.pos += 1;
                    let low = self.parse_hex4()?;
                    if low >= 0xDC00 && low <= 0xDFFF {
                        let combined = 0x10000u32 + ((cp - 0xD800) << 10) + (low - 0xDC00);
                        return Ok(char::from_u32(combined).unwrap_or('\u{FFFD}'));
                    }
                    self.pos = checkpoint;
                    return Ok('\u{FFFD}');
                }
                self.pos = checkpoint;
                return Ok('\u{FFFD}');
            }
            return Ok('\u{FFFD}');
        }
        if cp >= 0xDC00 && cp <= 0xDFFF {
            return Ok('\u{FFFD}');
        }
        Ok(char::from_u32(cp).unwrap_or('\u{FFFD}'))
    }

    fn parse_hex4(&mut self) -> Result<u32, String> {
        let mut value: u32 = 0;
        for _ in 0..4 {
            let b = match self.bump() {
                Some(b) => b,
                None => return Err("unterminated unicode escape".to_string()),
            };
            let digit = match b {
                b'0'..=b'9' => (b - b'0') as u32,
                b'a'..=b'f' => (b - b'a' + 10) as u32,
                b'A'..=b'F' => (b - b'A' + 10) as u32,
                _ => return Err(format!("invalid unicode escape at position {}", self.pos)),
            };
            value = value * 16 + digit;
        }
        Ok(value)
    }

    fn parse_utf8_tail(&mut self, first: u8, out: &mut String) -> Result<(), String> {
        let start = self.pos - 1;
        let len = utf8_len(first);
        if len == 0 {
            return Err(format!("invalid utf-8 in string at position {}", start));
        }
        let end = start + len;
        if end > self.input.len() {
            return Err(format!("invalid utf-8 in string at position {}", start));
        }
        match std::str::from_utf8(&self.input[start..end]) {
            Ok(text) => {
                out.push_str(text);
                self.pos = end;
                Ok(())
            }
            Err(_) => Err(format!("invalid utf-8 in string at position {}", start)),
        }
    }

    fn parse_array(&mut self) -> Result<Json, String> {
        self.depth += 1;
        let result = if self.depth > MAX_DEPTH {
            Err(format!("maximum nesting depth exceeded at position {}", self.pos))
        } else {
            self.parse_array_inner()
        };
        self.depth -= 1;
        result
    }

    fn parse_array_inner(&mut self) -> Result<Json, String> {
        self.bump();
        let mut items = Vec::new();
        self.skip_ws();
        if self.peek() == Some(b']') {
            self.bump();
            return Ok(Json::Array(items));
        }
        loop {
            let value = self.parse_value()?;
            items.push(value);
            self.skip_ws();
            match self.bump() {
                Some(b',') => {
                    self.skip_ws();
                }
                Some(b']') => return Ok(Json::Array(items)),
                _ => return Err(format!("expected ',' or ']' at position {}", self.pos)),
            }
        }
    }

    fn parse_object(&mut self) -> Result<Json, String> {
        self.depth += 1;
        let result = if self.depth > MAX_DEPTH {
            Err(format!("maximum nesting depth exceeded at position {}", self.pos))
        } else {
            self.parse_object_inner()
        };
        self.depth -= 1;
        result
    }

    fn parse_object_inner(&mut self) -> Result<Json, String> {
        self.bump();
        let mut entries = Vec::new();
        self.skip_ws();
        if self.peek() == Some(b'}') {
            self.bump();
            return Ok(Json::Object(entries));
        }
        loop {
            self.skip_ws();
            if self.peek() != Some(b'"') {
                return Err(format!("expected string key at position {}", self.pos));
            }
            let key = self.parse_string()?;
            self.skip_ws();
            match self.bump() {
                Some(b':') => {}
                _ => return Err(format!("expected ':' at position {}", self.pos)),
            }
            self.skip_ws();
            let value = self.parse_value()?;
            entries.push((key, value));
            self.skip_ws();
            match self.bump() {
                Some(b',') => {
                    self.skip_ws();
                }
                Some(b'}') => return Ok(Json::Object(entries)),
                _ => return Err(format!("expected ',' or closing brace at position {}", self.pos)),
            }
        }
    }
}

fn utf8_len(byte: u8) -> usize {
    if byte & 0x80 == 0 {
        1
    } else if byte & 0xE0 == 0xC0 {
        2
    } else if byte & 0xF0 == 0xE0 {
        3
    } else if byte & 0xF8 == 0xF0 {
        4
    } else {
        0
    }
}

pub fn parse(input: &[u8]) -> Result<Json, String> {
    let mut parser = Parser {
        input,
        pos: 0,
        depth: 0,
    };
    parser.skip_ws();
    let value = parser.parse_value()?;
    parser.skip_ws();
    if parser.pos != parser.input.len() {
        return Err(format!("trailing characters at position {}", parser.pos));
    }
    Ok(value)
}

pub fn stringify(value: &Json) -> String {
    let mut out = String::new();
    write_value(value, &mut out);
    out
}

fn write_value(value: &Json, out: &mut String) {
    match value {
        Json::Null => out.push_str("null"),
        Json::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Json::Num(n) => write_number(*n, out),
        Json::Str(s) => write_string(s, out),
        Json::Array(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_value(item, out);
            }
            out.push(']');
        }
        Json::Object(entries) => {
            out.push('{');
            for (i, (key, val)) in entries.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_string(key, out);
                out.push(':');
                write_value(val, out);
            }
            out.push('}');
        }
    }
}

fn write_number(n: f64, out: &mut String) {
    if !n.is_finite() {
        out.push_str("null");
        return;
    }
    if n.fract() == 0.0 && n.abs() < 1e18 {
        let as_i64 = n as i64;
        if as_i64 as f64 == n {
            out.push_str(&as_i64.to_string());
            return;
        }
    }
    out.push_str(&n.to_string());
}

fn write_string(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{0008}' => out.push_str("\\b"),
            '\u{000C}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                out.push_str("\\u");
                push_hex4(c as u32, out);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

fn push_hex4(value: u32, out: &mut String) {
    const HEX: [u8; 16] = [
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e',
        b'f',
    ];
    out.push(HEX[((value >> 12) & 0xF) as usize] as char);
    out.push(HEX[((value >> 8) & 0xF) as usize] as char);
    out.push(HEX[((value >> 4) & 0xF) as usize] as char);
    out.push(HEX[(value & 0xF) as usize] as char);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_object() {
        let input = b"{\"a\":1,\"b\":[true,false,null],\"c\":\"hi\\n\"}";
        let value = parse(input).unwrap();
        let text = stringify(&value);
        let value2 = parse(text.as_bytes()).unwrap();
        assert_eq!(text, stringify(&value2));
    }

    #[test]
    fn rejects_malformed_input_without_panicking() {
        let mut deep = String::new();
        for _ in 0..200 {
            deep.push('[');
        }
        let samples: [&[u8]; 9] = [
            b"123 abc",
            b"\xff\xfe\xfd",
            b"{",
            b"[1,2,",
            b"\"\\u",
            b"-",
            b"{\"a\":}",
            b"\"\xff\"",
            deep.as_bytes(),
        ];
        for sample in samples {
            assert!(parse(sample).is_err());
        }
    }

    #[test]
    fn formats_numbers() {
        assert_eq!(stringify(&Json::Num(5.0)), "5");
        assert_eq!(stringify(&Json::Num(5.5)), "5.5");
        assert_eq!(stringify(&Json::Num(f64::NAN)), "null");
        assert_eq!(stringify(&Json::Num(f64::INFINITY)), "null");
    }

    #[test]
    fn surrogate_pair_decodes() {
        let input = b"\"\\ud83d\\ude00\"";
        let value = parse(input).unwrap();
        match value {
            Json::Str(s) => assert_eq!(s, "\u{1F600}"),
            _ => assert!(false),
        }
    }
}
