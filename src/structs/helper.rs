/// Checks if a char is allowed in a PDB file.
/// The char has to be ASCII graphic or a space.
/// Returns `true` if the char is valid.
pub const fn check_char(c: char) -> bool {
    (c as u32) < 127 && c as u32 > 31
}

/// Checks a string using `check_char`.
/// Returns `true` if the text is valid.
pub fn valid_text(text: impl AsRef<str>) -> bool {
    text.as_ref().chars().all(check_char)
}

/// Checks a string using `check_char`.
/// Returns `true` if the text is valid.
pub fn valid_identifier(text: impl AsRef<str>) -> bool {
    text.as_ref().chars().all(check_char)
}

/// Creates a valid identifier from the given string slice.
/// Also turns the identifier to uppercase.
pub fn prepare_identifier_uppercase(text: impl AsRef<str>) -> Option<String> {
    let text = text.as_ref();
    prepare_identifier(text).map(|s| s.to_uppercase())
}

/// Creates a valid identifier from the given string slice.
/// Does not change the case.
pub fn prepare_identifier(text: impl AsRef<str>) -> Option<String> {
    let text = text.as_ref();
    (valid_identifier(text) && !text.trim().is_empty()).then(|| text.trim().to_string())
}

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Converts a number into a base26 with only the alphabet as possible chars
#[allow(clippy::unwrap_used)]
pub fn number_to_base26(mut num: usize) -> String {
    let mut output = vec![ALPHABET.chars().nth(num % 26).unwrap()];
    num /= 26;
    while num != 0 {
        output.push(ALPHABET.chars().nth(num % 26).unwrap());
        num /= 26;
    }
    output.iter().rev().collect::<String>()
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
        assert!(valid_text("ResidueName"));
        assert!(valid_text("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890`-=[]\\;',./~!@#$%^&*()_+{}|:\"<>? "));
    }
    #[test]
    fn incorrect_examples() {
        assert!(!check_char('̊'));
        assert!(!check_char('∞'));
        assert!(!check_char('👍'));
        assert!(!check_char('ÿ'));
        assert!(!check_char('\u{0}'));
        assert!(!valid_text("ResidueName∞"));
        assert!(!valid_text("Escape\u{0}"));
    }
    #[test]
    fn number_to_base26_test() {
        assert_eq!(number_to_base26(26), "BA");
        assert_eq!(number_to_base26(0), "A");
        assert_eq!(number_to_base26(234), "JA");
        assert_eq!(number_to_base26(25), "Z");
        assert_eq!(number_to_base26(457), "RP");
        assert_eq!(number_to_base26(15250), "WOO");
        assert_eq!(number_to_base26(396_514), "WOOO");
    }
}
