use std::mem::take;

enum ParseState {
    FieldStart,
    InUnquoted,
    InQuoted,
    AfterQuote,
}

pub fn parse(input: &[u8]) -> Vec<Vec<String>> {
    let text = String::from_utf8_lossy(input);
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut row: Vec<String> = Vec::new();
    let mut field = String::new();
    let mut state = ParseState::FieldStart;
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        match state {
            ParseState::FieldStart => match c {
                '"' => {
                    state = ParseState::InQuoted;
                }
                ',' => {
                    row.push(String::new());
                }
                '\r' => {
                    if chars.peek() == Some(&'\n') {
                        chars.next();
                        row.push(String::new());
                        rows.push(take(&mut row));
                    } else {
                        field.push(c);
                        state = ParseState::InUnquoted;
                    }
                }
                '\n' => {
                    row.push(String::new());
                    rows.push(take(&mut row));
                }
                _ => {
                    field.push(c);
                    state = ParseState::InUnquoted;
                }
            },
            ParseState::InUnquoted => match c {
                ',' => {
                    row.push(take(&mut field));
                    state = ParseState::FieldStart;
                }
                '\r' => {
                    if chars.peek() == Some(&'\n') {
                        chars.next();
                        row.push(take(&mut field));
                        rows.push(take(&mut row));
                        state = ParseState::FieldStart;
                    } else {
                        field.push(c);
                    }
                }
                '\n' => {
                    row.push(take(&mut field));
                    rows.push(take(&mut row));
                    state = ParseState::FieldStart;
                }
                _ => {
                    field.push(c);
                }
            },
            ParseState::InQuoted => match c {
                '"' => {
                    state = ParseState::AfterQuote;
                }
                _ => {
                    field.push(c);
                }
            },
            ParseState::AfterQuote => match c {
                '"' => {
                    field.push('"');
                    state = ParseState::InQuoted;
                }
                ',' => {
                    row.push(take(&mut field));
                    state = ParseState::FieldStart;
                }
                '\r' => {
                    if chars.peek() == Some(&'\n') {
                        chars.next();
                        row.push(take(&mut field));
                        rows.push(take(&mut row));
                        state = ParseState::FieldStart;
                    } else {
                        field.push(c);
                        state = ParseState::InUnquoted;
                    }
                }
                '\n' => {
                    row.push(take(&mut field));
                    rows.push(take(&mut row));
                    state = ParseState::FieldStart;
                }
                _ => {
                    field.push(c);
                    state = ParseState::InUnquoted;
                }
            },
        }
    }

    let pending = !matches!(state, ParseState::FieldStart) || !row.is_empty();
    if pending {
        row.push(field);
        rows.push(row);
    }

    rows
}

fn field_needs_quoting(field: &str) -> bool {
    field.chars().any(|c| c == ',' || c == '"' || c == '\n' || c == '\r')
}

pub fn write(rows: &[Vec<String>]) -> String {
    let mut out = String::new();
    for row in rows {
        let mut first = true;
        for field in row {
            if !first {
                out.push(',');
            }
            first = false;
            if field_needs_quoting(field) {
                out.push('"');
                for c in field.chars() {
                    if c == '"' {
                        out.push('"');
                        out.push('"');
                    } else {
                        out.push(c);
                    }
                }
                out.push('"');
            } else {
                out.push_str(field);
            }
        }
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(fields: &[&str]) -> Vec<String> {
        fields.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn empty_input_yields_empty_vec() {
        assert_eq!(parse(b""), Vec::<Vec<String>>::new());
    }

    #[test]
    fn simple_fields() {
        assert_eq!(parse(b"a,b,c\n"), vec![row(&["a", "b", "c"])]);
    }

    #[test]
    fn trailing_newline_no_extra_row() {
        let expected = vec![row(&["a", "b"]), row(&["c", "d"])];
        assert_eq!(parse(b"a,b\nc,d\n"), expected);
    }

    #[test]
    fn no_trailing_newline_still_parses_last_row() {
        let expected = vec![row(&["a", "b"]), row(&["c", "d"])];
        assert_eq!(parse(b"a,b\nc,d"), expected);
    }

    #[test]
    fn crlf_line_endings() {
        let expected = vec![row(&["a", "b"]), row(&["c", "d"])];
        assert_eq!(parse(b"a,b\r\nc,d\r\n"), expected);
    }

    #[test]
    fn lone_cr_is_kept_as_literal() {
        assert_eq!(parse(b"a\rb,c\n"), vec![row(&["a\rb", "c"])]);
    }

    #[test]
    fn empty_fields_in_middle_and_trailing() {
        assert_eq!(parse(b"a,,b\n"), vec![row(&["a", "", "b"])]);
        assert_eq!(parse(b"a,b,\n"), vec![row(&["a", "b", ""])]);
    }

    #[test]
    fn quoted_field_with_comma_and_newline() {
        assert_eq!(
            parse(b"\"hello, world\",b\n"),
            vec![row(&["hello, world", "b"])]
        );
        assert_eq!(
            parse(b"\"line1\nline2\",b\n"),
            vec![row(&["line1\nline2", "b"])]
        );
    }

    #[test]
    fn quoted_escaped_quote_and_empty_quoted_field() {
        assert_eq!(
            parse(b"\"she said \"\"hi\"\"\"\n"),
            vec![row(&["she said \"hi\""])]
        );
        assert_eq!(parse(b"\"\",b\n"), vec![row(&["", "b"])]);
    }

    #[test]
    fn lossy_and_arbitrary_bytes_never_panic() {
        let lossy: [u8; 3] = [b'a', 0xFF, b'b'];
        assert_eq!(parse(&lossy), vec![row(&["a\u{FFFD}b"])]);
        let arbitrary: [u8; 6] = [0xFF, 0xFE, b',', b'"', 0x00, b'\n'];
        let _ = parse(&arbitrary);
    }

    #[test]
    fn write_basic_and_quoting_rules() {
        assert_eq!(
            write(&[row(&["a", "b"]), row(&["c", "d"])]),
            "a,b\nc,d\n"
        );
        assert_eq!(write(&[row(&["a,b", "c"])]), "\"a,b\",c\n");
        assert_eq!(write(&[row(&["a\"b"])]), "\"a\"\"b\"\n");
        assert_eq!(
            write(&[row(&["a\nb", "c\rd"])]),
            "\"a\nb\",\"c\rd\"\n"
        );
    }

    #[test]
    fn write_empty_rows_yields_empty_string() {
        let rows: Vec<Vec<String>> = vec![];
        assert_eq!(write(&rows), "");
    }

    #[test]
    fn round_trip_complex() {
        let rows = vec![
            row(&["a,b", "c\"d", "e\nf"]),
            row(&["", "g"]),
            row(&["h\r\ni"]),
        ];
        let text = write(&rows);
        assert_eq!(parse(text.as_bytes()), rows);
    }

    #[test]
    fn round_trip_many_rows() {
        let mut rows = Vec::new();
        for i in 0..50 {
            rows.push(vec![
                format!("field{}", i),
                "x,y".to_string(),
                "z\"w".to_string(),
            ]);
        }
        let text = write(&rows);
        assert_eq!(parse(text.as_bytes()), rows);
    }
}
