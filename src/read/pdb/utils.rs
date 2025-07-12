/// Extracts u64 from a string.
pub(crate) fn fast_parse_u64_from_string(s: &str) -> Result<u64, String> {
    let bytes = s.as_bytes();
    let mut result: u64 = 0;
    let mut i = 0;
    let len = bytes.len();

    // Skip leading whitespace
    while i < len && bytes[i].is_ascii_whitespace() {
        i += 1;
    }

    // Parse digits
    let start = i;
    while i < len && bytes[i].is_ascii_digit() {
        result = result * 10 + (bytes[i] - b'0') as u64;
        i += 1;
    }

    if i == start {
        return Err("No digits found".to_string());
    }

    Ok(result)
}

/// Trims whitespace from a string, quickly.
pub(crate) fn fast_trim(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut start = 0;
    let mut end = bytes.len();

    while start < end {
        if bytes[start].is_ascii_whitespace() {
            start += 1;
        } else if bytes[end - 1].is_ascii_whitespace() {
            end -= 1;
        } else {
            break;
        }
    }

    // SAFETY: We only slice within the original string boundaries
    // SAFETY: We know the sliced bytes are valid UTF-8 because all rust strings are valid UTF-8 and it came from a string in the first place.
    unsafe { std::str::from_utf8_unchecked(&bytes[start..end]) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u64_parse() {
        assert_eq!(fast_parse_u64_from_string("  100").unwrap(), 100);
        assert_eq!(fast_parse_u64_from_string("    32").unwrap(), 32);
        assert_eq!(fast_parse_u64_from_string("9999").unwrap(), 9999);
        assert_eq!(fast_parse_u64_from_string("     1234").unwrap(), 1234);
        assert!(fast_parse_u64_from_string(" abc ").is_err());
        assert!(fast_parse_u64_from_string(" ").is_err());
        assert!(fast_parse_u64_from_string("     ").is_err());
        assert!(fast_parse_u64_from_string("").is_err());
    }

    #[test]
    fn test_trim() {
        assert_eq!(fast_trim("   hello   "), "hello");
        assert_eq!(fast_trim("  world"), "world");
        assert_eq!(fast_trim("rust"), "rust");
        assert_eq!(fast_trim("   "), "");
    }
}
