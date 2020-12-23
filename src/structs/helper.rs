pub fn check_char(c: char) -> bool {
    c.is_ascii_graphic() || c == ' '
}

pub fn check_char2(c: [char; 2]) -> bool {
    check_char(c[0]) && check_char(c[1])
}

pub fn check_char3(c: [char; 3]) -> bool {
    check_char(c[0]) && check_char(c[1]) && check_char(c[2])
}

pub fn check_char4(c: [char; 4]) -> bool {
    check_char(c[0]) && check_char(c[1]) && check_char(c[2]) && check_char(c[3])
}

pub fn check_chars(text: String) -> bool {
    for c in text.chars() {
        if !check_char(c) {
            return false;
        }
    }
    true
}
