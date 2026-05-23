pub fn count_words(text: &[u8]) -> usize {
    let mut count = 0;
    let mut in_word = false;
    for &b in text {
        let ws = b == b' ' || b == b'\t' || b == b'\n' || b == b'\r';
        if ws {
            in_word = false;
        } else if !in_word {
            in_word = true;
            count += 1;
        }
    }
    count
}

pub fn is_palindrome(text: &[u8]) -> bool {
    let mut filtered: Vec<u8> = Vec::new();
    for &b in text {
        if b.is_ascii_alphanumeric() {
            filtered.push(b.to_ascii_lowercase());
        }
    }
    let n = filtered.len();
    let mut i = 0;
    while i < n / 2 {
        if filtered[i] != filtered[n - 1 - i] {
            return false;
        }
        i += 1;
    }
    true
}

pub fn levenshtein(a: &[u8], b: &[u8]) -> usize {
    let n = a.len();
    let m = b.len();
    if n == 0 {
        return m;
    }
    if m == 0 {
        return n;
    }
    let mut prev: Vec<usize> = (0..=m).collect();
    let mut cur: Vec<usize> = vec![0; m + 1];
    for i in 1..=n {
        cur[0] = i;
        for j in 1..=m {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            let del = prev[j] + 1;
            let ins = cur[j - 1] + 1;
            let sub = prev[j - 1] + cost;
            let mut best = del;
            if ins < best {
                best = ins;
            }
            if sub < best {
                best = sub;
            }
            cur[j] = best;
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[m]
}

pub fn word_wrap(text: &[u8], width: usize) -> String {
    if width == 0 {
        return String::from_utf8_lossy(text).into_owned();
    }
    let mut out = String::new();
    let mut line_len = 0;
    let mut first_on_line = true;
    for w in text.split(|&b| b == b' ' || b == b'\n' || b == b'\t' || b == b'\r') {
        if w.is_empty() {
            continue;
        }
        let wl = w.len();
        if !first_on_line && line_len + 1 + wl > width {
            out.push('\n');
            line_len = 0;
            first_on_line = true;
        }
        if !first_on_line {
            out.push(' ');
            line_len += 1;
        }
        out.push_str(&String::from_utf8_lossy(w));
        line_len += wl;
        first_on_line = false;
    }
    out
}

pub fn indent(text: &str, prefix: &str) -> String {
    let mut out = String::new();
    for (i, line) in text.split('\n').enumerate() {
        if i > 0 {
            out.push('\n');
        }
        if !line.is_empty() {
            out.push_str(prefix);
        }
        out.push_str(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words() {
        assert_eq!(count_words(b"the quick brown fox"), 4);
        assert_eq!(count_words(b"   spaced    out   "), 2);
        assert_eq!(count_words(b""), 0);
        assert_eq!(count_words(b"one"), 1);
    }

    #[test]
    fn palindromes() {
        assert!(is_palindrome(b"racecar"));
        assert!(is_palindrome(b"A man, a plan, a canal: Panama"));
        assert!(is_palindrome(b""));
        assert!(!is_palindrome(b"hello"));
    }

    #[test]
    fn edit_distance() {
        assert_eq!(levenshtein(b"kitten", b"sitting"), 3);
        assert_eq!(levenshtein(b"", b"abc"), 3);
        assert_eq!(levenshtein(b"same", b"same"), 0);
        assert_eq!(levenshtein(b"flaw", b"lawn"), 2);
    }

    #[test]
    fn wrapping() {
        let out = word_wrap(b"the quick brown fox jumps", 10);
        for line in out.split('\n') {
            assert!(line.len() <= 10 || !line.contains(' '));
        }
        assert!(out.contains('\n'));
    }

    #[test]
    fn indenting() {
        assert_eq!(indent("a\nb", ">> "), ">> a\n>> b");
        assert_eq!(indent("x\n\ny", "-"), "-x\n\n-y");
    }
}
