pub(crate) fn fast_parse_u64_from_string(s: &str) -> u64 {
    let bytes = s.as_bytes();
    let mut result: u64 = 0;
    let mut i = 0;
    let len = bytes.len();

    // Skip leading whitespace
    while i < len && bytes[i].is_ascii_whitespace() {
        i += 1;
    }

    // Parse digits
    while i < len && bytes[i].is_ascii_digit() {
        result = result * 10 + (bytes[i] - b'0') as u64;
        i += 1;
    }

    result
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

    // SAFETY: We only slice within the original string boundaries
    // SAFETY: We know the sliced bytes are valid UTF-8 because all rust strings are valid UTF-8 and it came from a string in the first place.
    unsafe { std::str::from_utf8_unchecked(&bytes[start..end]) }
}