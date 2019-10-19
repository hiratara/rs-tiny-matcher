use std::env;

fn match_(regexp: &[u8], mut text: &[u8]) -> bool {
    let opt_cs0 = regexp.get(0);
    if let Some(&c) = opt_cs0 {
        if c == b'^' {
            return matchhere(&regexp[1..], text);
        }
    }
    loop {
        if matchhere(regexp, text) {
            return true;
        }
        if text.len() <= 1 {
            break;
        }
        text = &text[1..];
    }
    return false;
}

fn matchhere(regexp: &[u8], text: &[u8]) -> bool {
    if regexp.is_empty() {
        return true;
    }
    let cs0 = regexp[0];
    let opt_cs1 = regexp.get(1);
    if let Some(&c) = opt_cs1 {
        if c == b'*' {
            return matchstar(cs0, &regexp[2..], text);
        }
    } else if cs0 == b'$' {
        return text.is_empty();
    }
    if !text.is_empty() && (cs0 == b'.' || cs0 == text[0]) {
        return matchhere(&regexp[1..], &text[1..]);
    }
    false
}

fn matchstar(c: u8, regexp: &[u8], mut text: &[u8]) -> bool {
    loop {
        if matchhere(regexp, text) {
            return true;
        }
        if text.is_empty() || text[0] != c && c != b'.' {
            break;
        }
        text = &text[1..];
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
        if match_(regex.as_bytes(), text.as_bytes()) {
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
        assert!(match_(b"abc", b"abc"));
        assert!(match_(b"abc", b"_abc_"));
        assert!(match_(b"abc$", b"_abc"));
        assert!(match_(b"^abc", b"abc_"));
        assert!(match_(b"^abc$", b"abc"));
        assert!(match_(b"...", b"abc"));
        assert!(match_(b".*", b"abc"));
        assert!(match_(b".*", b""));
        assert!(match_(b"ab*c", b"abbc"));
        assert!(match_(b"ab*c", b"ac"));
        assert!(match_(b"^$", b""));
        assert!(match_(b"", b""));
        assert!(match_(b"", b"b"));

        assert!(!match_(b"abc", b"b"));
        assert!(!match_(b"abc", b"cbcab"));
        assert!(!match_(b"abc", b""));
        assert!(!match_(b"abc$", b"abc$"));
        assert!(!match_(b"abc$", b""));
        assert!(!match_(b"^abc", b"^abc"));
        assert!(!match_(b"^abc", b""));
        assert!(!match_(b"^abc$", b"^abc$"));
        assert!(!match_(b"^abc$", b""));
        assert!(!match_(b"...", b"bc"));
        assert!(!match_(b"...", b""));
        assert!(!match_(b"ab*c", b"adc"));
        assert!(!match_(b"ab*c", b"bbb"));
        assert!(!match_(b"ab*c", b"a"));
        assert!(!match_(b"ab*c", b"ab*c"));
        assert!(!match_(b"ab*c", b""));
        assert!(!match_(b"^$", b"a"));

        // invalid regexes
        assert!(match_(b"*", b"*"));
        assert!(match_(b"a^", b"a^"));
        assert!(match_(b"$a", b"$a"));
    }
}
