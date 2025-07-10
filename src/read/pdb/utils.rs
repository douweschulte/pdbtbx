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

//TODO: Implement miri tests and fuzzing
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

    // SAFETY: This is safe because:
    // 1. We only slice within the original string boundaries (0 <= start <= end <= bytes.len())
    // 2. We only check and skip ASCII whitespace characters, which are always single-byte in UTF-8
    // 3. Since we're only moving past ASCII characters, we can't split a multi-byte UTF-8 sequence
    // 4. The original string was valid UTF-8, and we're creating a substring of it
    unsafe { std::str::from_utf8_unchecked(&bytes[start..end]) }
}
