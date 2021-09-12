use anyhow::{anyhow, Result};
use std::str;

const RESERVED_CHARS: [char; 4] = ['-', '.', '_', '~'];
const FAILED_TO_DECODE: &str = "Failed to decode percent-encoding string";

pub fn encode(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            c if c.is_ascii_alphanumeric() || is_reserved(c) => c.to_string(),
            c => escape_with_parcent(c),
        })
        .collect()
}

pub fn url_encode(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            c if is_whitespace(c) => "+".to_string(),
            c if c.is_ascii_alphanumeric() || is_reserved(c) => c.to_string(),
            c => escape_with_parcent(c),
        })
        .collect()
}

fn is_reserved(c: char) -> bool {
    RESERVED_CHARS.contains(&c)
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

pub fn decode(s: &str) -> Result<String> {
    do_decode(s)
}

pub fn url_decode(s: &str) -> Result<String> {
    do_decode(s.replace("+", " ").as_ref())
}

pub fn strict_decode(s: &str) -> Result<String> {
    if s.chars().any(|c| !c.is_alphanumeric() && !is_reserved(c)) {
        return Err(anyhow!(FAILED_TO_DECODE));
    }

    do_decode(s)
}

pub fn strict_url_decode(s: &str) -> Result<String> {
    if s.chars()
        .any(|c| !c.is_alphanumeric() && !is_reserved(c) && c != '+')
    {
        return Err(anyhow!(FAILED_TO_DECODE));
    }

    do_decode(s)
}

fn do_decode(s: &str) -> Result<String> {
    let mut res = String::new();
    let mut b = Vec::new();
    let mut u = String::new();
    let mut latch = false;
    for c in s.chars() {
        if c == '%' {
            latch = true;
            continue;
        }

        if latch && u.is_empty() {
            u.push(c);
        } else if latch {
            u.push(c);
            b.push(u8::from_str_radix(u.as_ref(), 16)?);
            u.clear();
            latch = false;
        } else if !b.is_empty() {
            res.push_str(str::from_utf8(b.as_ref())?);
            res.push(c);
            b.clear();
        } else {
            res.push(c);
        }
    }

    if !u.is_empty() {
        return Err(anyhow!(FAILED_TO_DECODE));
    }

    if !b.is_empty() {
        res.push_str(str::from_utf8(b.as_ref())?);
    }

    Ok(res)
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
        assert_eq!(encode("abc"), "abc");
        assert_eq!(encode("%"), "%25");
        assert_eq!(encode(" "), "%20");
        assert_eq!(encode("テスト"), "%E3%83%86%E3%82%B9%E3%83%88");
    }

    #[test]
    fn test_url_encode() {
        assert_eq!(url_encode("abc"), "abc");
        assert_eq!(url_encode("%"), "%25");
        assert_eq!(url_encode(" "), "+");
        assert_eq!(url_encode("テスト"), "%E3%83%86%E3%82%B9%E3%83%88");
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode("abc").unwrap(), "abc");
        assert_eq!(decode("%25").unwrap(), "%");
        assert_eq!(decode("%20").unwrap(), " ");
        assert_eq!(decode("%E3%83%86%E3%82%B9%E3%83%88").unwrap(), "テスト");
    }

    #[test]
    fn test_url_decode() {
        assert_eq!(url_decode("abc").unwrap(), "abc");
        assert_eq!(url_decode("%25").unwrap(), "%");
        assert_eq!(url_decode("+").unwrap(), " ");
        assert_eq!(url_decode("%E3%83%86%E3%82%B9%E3%83%88").unwrap(), "テスト");
    }

    #[test]
    fn test_strict_decode() {
        assert!(strict_decode("abc123").is_ok());
        assert!(strict_decode(" ").is_err());
        assert!(strict_decode("%").is_err());
        assert!(strict_decode("+").is_err());
    }

    #[test]
    fn test_strict_url_decode() {
        assert!(strict_url_decode("abc123").is_ok());
        assert!(strict_url_decode(" ").is_err());
        assert!(strict_url_decode("%").is_err());
        assert!(strict_url_decode("+").is_ok());
    }
}
