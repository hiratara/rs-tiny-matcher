use std::env;

fn reg_match(regexp: &[u8], text: &[u8]) -> bool {
    if let Some(&b'^') = regexp.get(0) {
        return reg_matchhere(&regexp[1..], text);
    }
    for i in 0..text.len() + 1 {
        let text = &text[i..];
        if reg_matchhere(regexp, text) {
            return true;
        }
    }
    false
}

fn reg_matchhere(regexp: &[u8], text: &[u8]) -> bool {
    if regexp.is_empty() {
        return true;
    }
    if let Some(&b'*') = regexp.get(1) {
        return reg_matchstar(regexp[0], &regexp[2..], text);
    } else if regexp[0] == b'$' {
        return text.is_empty();
    }
    if !text.is_empty() && (regexp[0] == b'.' || regexp[0] == text[0]) {
        return reg_matchhere(&regexp[1..], &text[1..]);
    }
    false
}

fn reg_matchstar(c: u8, regexp: &[u8], text: &[u8]) -> bool {
    for i in 0..text.len() + 1 {
        let text = &text[i..];
        if reg_matchhere(regexp, text) {
            return true;
        }
        if !text.is_empty() && text[0] != c && c != b'.' {
            break;
        }
    }
    false
}

fn main() {
    let mut args = env::args();
    let _ = args.next();
    let regex = args.next().expect("no regex found");
    let text = args.next().expect("no text found");

    println!(
        "{}",
        if reg_match(regex.as_bytes(), text.as_bytes()) {
            "matched"
        } else {
            "unmatched"
        }
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_() {
        assert!(reg_match(b"abc", b"abc"));
        assert!(reg_match(b"abc", b"_abc_"));
        assert!(reg_match(b"abc$", b"_abc"));
        assert!(reg_match(b"^abc", b"abc_"));
        assert!(reg_match(b"^abc$", b"abc"));
        assert!(reg_match(b"...", b"abc"));
        assert!(reg_match(b".*", b"abc"));
        assert!(reg_match(b".*", b""));
        assert!(reg_match(b"^.*$", b"a"));
        assert!(reg_match(b"^.*$", b""));
        assert!(reg_match(b"ab*c", b"abbc"));
        assert!(reg_match(b"ab*c", b"ac"));
        assert!(reg_match(b"^$", b""));
        assert!(reg_match(b"", b""));
        assert!(reg_match(b"", b"b"));

        assert!(!reg_match(b"abc", b"b"));
        assert!(!reg_match(b"abc", b"cbcab"));
        assert!(!reg_match(b"abc", b""));
        assert!(!reg_match(b"abc$", b"abc$"));
        assert!(!reg_match(b"abc$", b""));
        assert!(!reg_match(b"^abc", b"^abc"));
        assert!(!reg_match(b"^abc", b""));
        assert!(!reg_match(b"^abc$", b"^abc$"));
        assert!(!reg_match(b"^abc$", b""));
        assert!(!reg_match(b"...", b"bc"));
        assert!(!reg_match(b"...", b""));
        assert!(!reg_match(b"ab*c", b"adc"));
        assert!(!reg_match(b"ab*c", b"bbb"));
        assert!(!reg_match(b"ab*c", b"a"));
        assert!(!reg_match(b"ab*c", b"ab*c"));
        assert!(!reg_match(b"ab*c", b""));
        assert!(!reg_match(b"^$", b"a"));

        // invalid regexes
        assert!(reg_match(b"*", b"*"));
        assert!(reg_match(b"a^", b"a^"));
        assert!(reg_match(b"$a", b"$a"));
    }
}
