/// Checks if a char is allowed in an identifier in a PDB file.
/// The char has to be ASCII graphic or a space.
/// Returns `true` if the chars is valid.
pub fn check_char(c: char) -> bool {
    (c as u32) < 127 && c as u32 > 31
}

/// Checks an array of two chars using `check_char`.
/// Returns `true` if the text is valid.
pub fn check_char2(c: [char; 2]) -> bool {
    check_char(c[0]) && check_char(c[1])
}

/// Checks an array of three chars using `check_char`.
/// Returns `true` if the text is valid.
pub fn check_char3(c: [char; 3]) -> bool {
    check_char(c[0]) && check_char(c[1]) && check_char(c[2])
}

/// Checks an array of four chars using `check_char`.
/// Returns `true` if the text is valid.
pub fn check_char4(c: [char; 4]) -> bool {
    check_char(c[0]) && check_char(c[1]) && check_char(c[2]) && check_char(c[3])
}

/// Checks a string using `check_char`.
/// Returns `true` if the text is valid.
pub fn check_chars(text: String) -> bool {
    for c in text.chars() {
        if !check_char(c) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn correct_examples() {
        assert!(check_char('a'));
        assert!(check_char('9'));
        assert!(check_char('*'));
        assert!(check_char('@'));
        assert!(check_char('O'));
        assert!(check_char2(['a', 'a']));
        assert!(check_char3(['V', 'A', 'L']));
        assert!(check_char4(['R', 'E', 'S', 'D']));
        assert!(check_chars("ResidueName".to_string()));
        assert!(check_chars("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890`-=[]\\;',./~!@#$%^&*()_+{}|:\"<>? ".to_string()));
    }
    #[test]
    fn incorrect_examples() {
        assert!(!check_char('̊'));
        assert!(!check_char('∞'));
        assert!(!check_char('👍'));
        assert!(!check_char('ÿ'));
        assert!(!check_char('\u{0}'));
        assert!(!check_char2(['\u{0}', 'a']));
        assert!(!check_char3(['V', 'ÿ', 'L']));
        assert!(!check_char4(['R', 'E', '\u{0}', 'D']));
        assert!(!check_chars("ResidueName∞".to_string()));
        assert!(!check_chars("Escape\u{0}".to_string()));
    }
}
