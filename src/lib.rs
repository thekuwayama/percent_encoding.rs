use anyhow::{anyhow, Result};
use std::str;

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

pub fn decode(s: &str) -> Result<String> {
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
        return Err(anyhow!("Failed to decode percent-encoding string"));
    }

    if !b.is_empty() {
        res.push_str(str::from_utf8(b.as_ref())?);
    }

    Ok(res)
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
}
