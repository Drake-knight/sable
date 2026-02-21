fn match_class(pattern: &[u8], start: usize, c: u8) -> (bool, usize) {
    let len = pattern.len();
    if start >= len || pattern[start] != b'[' {
        return (false, start);
    }
    let mut i = start + 1;
    let mut negate = false;
    if i < len && pattern[i] == b'^' {
        negate = true;
        i += 1;
    }
    let body_start = i;
    let mut j = body_start;
    if j < len && pattern[j] == b']' {
        j += 1;
    }
    while j < len && pattern[j] != b']' {
        if pattern[j] == b'\\' && j + 1 < len {
            j += 2;
        } else {
            j += 1;
        }
    }
    if j >= len {
        return (c == b'[', start + 1);
    }
    let body_end = j;
    let mut matched = false;
    let mut k = body_start;
    while k < body_end {
        let lo;
        if pattern[k] == b'\\' && k + 1 < body_end {
            lo = pattern[k + 1];
            k += 2;
        } else {
            lo = pattern[k];
            k += 1;
        }
        if k + 1 < body_end && pattern[k] == b'-' {
            let hi;
            if pattern[k + 1] == b'\\' && k + 2 < body_end {
                hi = pattern[k + 2];
                k += 3;
            } else {
                hi = pattern[k + 1];
                k += 2;
            }
            let lo2 = if lo <= hi { lo } else { hi };
            let hi2 = if lo <= hi { hi } else { lo };
            if c >= lo2 && c <= hi2 {
                matched = true;
            }
        } else if c == lo {
            matched = true;
        }
    }
    if negate {
        matched = !matched;
    }
    (matched, body_end + 1)
}

fn match_one(pattern: &[u8], pi: usize, tc: u8) -> Option<usize> {
    let plen = pattern.len();
    if pi >= plen {
        return None;
    }
    match pattern[pi] {
        b'?' => Some(pi + 1),
        b'[' => {
            let (m, next_pi) = match_class(pattern, pi, tc);
            if m {
                Some(next_pi)
            } else {
                None
            }
        }
        b'\\' => {
            if pi + 1 < plen {
                if pattern[pi + 1] == tc {
                    Some(pi + 2)
                } else {
                    None
                }
            } else if tc == b'\\' {
                Some(pi + 1)
            } else {
                None
            }
        }
        c => {
            if c == tc {
                Some(pi + 1)
            } else {
                None
            }
        }
    }
}

fn scan_match(
    pattern: &[u8],
    text: &[u8],
    steps: &mut u64,
    max_steps: u64,
    require_full_text: bool,
) -> bool {
    let plen = pattern.len();
    let tlen = text.len();
    let mut pi = 0usize;
    let mut ti = 0usize;
    let mut has_star = false;
    let mut star_p = 0usize;
    let mut star_t = 0usize;
    loop {
        *steps += 1;
        if *steps > max_steps {
            return false;
        }
        if pi == plen {
            if !require_full_text || ti == tlen {
                return true;
            }
        } else if pattern[pi] == b'*' {
            has_star = true;
            star_p = pi;
            star_t = ti;
            pi += 1;
            continue;
        } else if ti < tlen {
            if let Some(next_pi) = match_one(pattern, pi, text[ti]) {
                pi = next_pi;
                ti += 1;
                continue;
            }
        }
        if has_star {
            star_t += 1;
            ti = star_t;
            pi = star_p + 1;
            if ti > tlen {
                return false;
            }
            continue;
        }
        return false;
    }
}

pub fn is_match(pattern: &[u8], text: &[u8]) -> bool {
    let mut steps: u64 = 0;
    scan_match(pattern, text, &mut steps, 500_000, true)
}

pub fn contains_match(pattern: &[u8], text: &[u8]) -> bool {
    let tlen = text.len();
    let mut steps: u64 = 0;
    let max_steps: u64 = 500_000;
    let mut start = 0usize;
    loop {
        if steps > max_steps {
            return false;
        }
        if scan_match(pattern, &text[start..], &mut steps, max_steps, false) {
            return true;
        }
        if start >= tlen {
            return false;
        }
        start += 1;
    }
}

pub fn split_once_byte(text: &[u8], sep: u8) -> Option<(&[u8], &[u8])> {
    let mut i = 0usize;
    while i < text.len() {
        if text[i] == sep {
            return Some((&text[..i], &text[i + 1..]));
        }
        i += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_pattern_empty_text() {
        assert!(is_match(b"", b""));
    }

    #[test]
    fn empty_pattern_nonempty_text() {
        assert!(!is_match(b"", b"a"));
    }

    #[test]
    fn empty_text_nonempty_pattern() {
        assert!(!is_match(b"a", b""));
        assert!(is_match(b"*", b""));
        assert!(is_match(b"***", b""));
    }

    #[test]
    fn literal_exact() {
        assert!(is_match(b"hello", b"hello"));
        assert!(!is_match(b"hello", b"hellox"));
        assert!(!is_match(b"hello", b"hell"));
    }

    #[test]
    fn star_basic() {
        assert!(is_match(b"*", b"anything at all"));
        assert!(is_match(b"a*", b"a"));
        assert!(is_match(b"a*", b"aXYZ"));
        assert!(is_match(b"*a", b"XYZa"));
        assert!(is_match(b"a*b", b"ab"));
        assert!(is_match(b"a*b", b"aXXXb"));
        assert!(!is_match(b"a*b", b"aXXXc"));
        assert!(is_match(b"a*b*c", b"aXbYc"));
        assert!(is_match(b"**", b"anything"));
        assert!(is_match(b"**", b""));
    }

    #[test]
    fn question_mark() {
        assert!(is_match(b"a?c", b"abc"));
        assert!(!is_match(b"a?c", b"ac"));
        assert!(!is_match(b"?", b""));
        assert!(is_match(b"???", b"xyz"));
        assert!(!is_match(b"a?", b"a"));
    }

    #[test]
    fn classes_basic() {
        assert!(is_match(b"[abc]", b"a"));
        assert!(is_match(b"[abc]", b"b"));
        assert!(!is_match(b"[abc]", b"d"));
        assert!(is_match(b"[a-z]", b"m"));
        assert!(!is_match(b"[a-z]", b"M"));
        assert!(is_match(b"[^a-z]", b"M"));
        assert!(!is_match(b"[^a-z]", b"m"));
    }

    #[test]
    fn classes_leading_bracket_literal() {
        assert!(is_match(b"[]a]", b"]"));
        assert!(is_match(b"[]a]", b"a"));
        assert!(!is_match(b"[]a]", b"b"));
        assert!(is_match(b"[^]a]", b"x"));
        assert!(!is_match(b"[^]a]", b"]"));
    }

    #[test]
    fn classes_reversed_range() {
        assert!(is_match(b"[z-a]", b"m"));
        assert!(!is_match(b"[z-a]", b"5"));
    }

    #[test]
    fn classes_unterminated() {
        assert!(is_match(b"[abc", b"[abc"));
        assert!(!is_match(b"[abc", b"xabc"));
        assert!(is_match(b"[]", b"[]"));
    }

    #[test]
    fn escapes() {
        assert!(is_match(b"\\*", b"*"));
        assert!(!is_match(b"\\*", b"x"));
        assert!(is_match(b"\\?", b"?"));
        assert!(is_match(b"\\[abc\\]", b"[abc]"));
        assert!(is_match(b"a\\", b"a\\"));
    }

    #[test]
    fn anchoring() {
        assert!(!is_match(b"bc", b"abcd"));
        assert!(is_match(b"*bc*", b"abcd"));
    }

    #[test]
    fn contains_basic() {
        assert!(contains_match(b"abc", b"xxabcyy"));
        assert!(!contains_match(b"abc", b"xxabyy"));
        assert!(contains_match(b"a*b", b"xaYYbz"));
        assert!(contains_match(b"", b"anything"));
        assert!(contains_match(b"", b""));
        assert!(!contains_match(b"a", b""));
        assert!(contains_match(b"xyz", b"xyz"));
    }

    #[test]
    fn unterminated_bracket_top_level() {
        assert!(is_match(b"[", b"["));
        assert!(!is_match(b"[", b"x"));
        assert!(is_match(b"[^", b"[^"));
        assert!(!is_match(b"[^", b"^"));
    }

    #[test]
    fn split_once_byte_basic() {
        assert_eq!(
            split_once_byte(b"a=b=c", b'='),
            Some((&b"a"[..], &b"b=c"[..]))
        );
        assert_eq!(split_once_byte(b"noequals", b'='), None);
        assert_eq!(split_once_byte(b"", b'='), None);
        assert_eq!(split_once_byte(b"=abc", b'='), Some((&b""[..], &b"abc"[..])));
        assert_eq!(split_once_byte(b"abc=", b'='), Some((&b"abc"[..], &b""[..])));
    }

    #[test]
    fn bounded_no_panic() {
        let mut pattern: Vec<u8> = Vec::new();
        for _ in 0..25 {
            pattern.push(b'a');
            pattern.push(b'*');
        }
        pattern.push(b'b');
        let mut text: Vec<u8> = Vec::new();
        for _ in 0..2000 {
            text.push(b'a');
        }
        assert!(!is_match(&pattern, &text));
        assert!(!contains_match(&pattern, &text));
    }

    #[test]
    fn contains_match_never_panics_on_long_star_pattern() {
        let mut pattern: Vec<u8> = Vec::new();
        for _ in 0..40 {
            pattern.push(b'?');
            pattern.push(b'*');
        }
        let mut text: Vec<u8> = Vec::new();
        for i in 0..3000u32 {
            text.push((b'a' as u32 + (i % 5)) as u8);
        }
        let _ = is_match(&pattern, &text);
        let _ = contains_match(&pattern, &text);
    }
}
