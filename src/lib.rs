pub fn encode(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            c if c.is_ascii_alphanumeric() || is_reserved(c) => c.to_string(),
            c => escape_with_parcent(c),
        })
        .collect::<String>()
}

pub fn url_encode(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            c if is_whitespace(c) => "+".to_string(),
            c if c.is_ascii_alphanumeric() || is_reserved(c) => c.to_string(),
            c => escape_with_parcent(c),
        })
        .collect::<String>()
}

fn is_reserved(c: char) -> bool {
    ['-', '.', '_', '~'].contains(&c)
}

fn escape_with_parcent(c: char) -> String {
    let mut b = [0u8; 4];
    c.encode_utf8(&mut b);

    let mut res = String::new();
    for u in &b[..c.len_utf8()] {
        res.push('%');
        res.push_str(format!("{:02X}", u).as_ref());
    }

    res
}

fn is_whitespace(c: char) -> bool {
    c == ' '
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_reserved() {
        assert!(is_reserved('-'));
        assert!(is_reserved('.'));
        assert!(is_reserved('_'));
        assert!(is_reserved('~'));
        assert!(!is_reserved('a'));
    }

    #[test]
    fn test_is_whitespace() {
        assert!(is_whitespace(' '));
        assert!(!is_whitespace('　'));
    }

    #[test]
    fn test_encode() {
        assert_eq!(encode("テスト"), "%E3%83%86%E3%82%B9%E3%83%88");
    }

    #[test]
    fn test_url_encode() {
        assert_eq!(url_encode("テ ス ト"), "%E3%83%86+%E3%82%B9+%E3%83%88");
    }
}
