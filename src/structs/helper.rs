/// Checks if a char is allowed in an identifier in a PDB file.
/// The char has to be ASCII graphic or a space.
/// Returns `true` if the chars is valid.
pub fn check_char(c: u8) -> bool {
    c < 127 && c > 31
}

/// Checks an array of two chars using `check_char`.
/// Returns `true` if the text is valid.
pub fn check_chars(chars: &[u8]) -> bool {
    chars.iter().all(|c| check_char(*c))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn correct_examples() {
        assert!(check_char(b'a'));
        assert!(check_char(b'9'));
        assert!(check_char(b'*'));
        assert!(check_char(b'@'));
        assert!(check_char(b'O'));
        assert!(check_chars(&[b'a', b'a']));
        assert!(check_chars(&[b'V', b'A', b'L']));
        assert!(check_chars(&[b'R', b'E', b'S', b'D']));
        assert!(check_chars(b"ResidueName"));
        assert!(check_chars(b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890`-=[]\\;',./~!@#$%^&*()_+{}|:\"<>? "));
    }
    #[test]
    fn incorrect_examples() {
        // assert!(!check_char(b'̊'));
        // assert!(!check_char(b'∞'));
        // assert!(!check_char(b'👍'));
        // assert!(!check_char(b'ÿ'));
        // assert!(!check_char(b'\u{0}'));
        // assert!(!check_chars(&[b'\u{0}', b'a']));
        // assert!(!check_chars(&[b'V', b'ÿ', b'L']));
        // assert!(!check_chars(&[b'R', b'E', b'\u{0}', b'D']));
        // assert!(!check_chars(b"ResidueName∞".to_string()));
        // assert!(!check_chars(b"Escape\u{0}".to_string()));
    }
}
