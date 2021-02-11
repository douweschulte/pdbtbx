#![allow(clippy::missing_docs_in_private_items, dead_code, clippy::unwrap_used)]

/// Parse a CIF file into CIF intermediate structure
pub fn parse_cif(input: String) -> Result<DataBlock, ()> {
    parse_main(&mut Position {
        text: &input[..],
        line: 0,
        column: 0,
    })
}

// State: (String, isize, isize) (input, line, column)

#[derive(Debug)]
pub struct DataBlock {
    name: String,
    items: Vec<Item>,
}

#[derive(Debug)]
pub enum Item {
    DataItem(DataItem),
    SaveFrame(SaveFrame),
}

#[derive(Debug)]
pub struct SaveFrame {
    items: Vec<DataItem>,
}

#[derive(Debug)]
pub struct DataItem {
    name: String,
    content: MultiValue,
}

#[derive(Debug)]
pub enum MultiValue {
    Value(Value),
    Loop(Loop),
}

#[derive(Debug)]
pub struct Loop {
    header: Vec<String>,
    data: Vec<Value>,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Inapplicable,
    Unknown,
    Numeric(f64),
    NumericWithUncertainty(f64, u32),
    Text(String),
}

#[derive(Copy, Clone)]
struct Position<'a> {
    text: &'a str,
    line: usize,
    column: usize,
}

fn parse_main(input: &mut Position) -> Result<DataBlock, ()> {
    trim_comments_and_whitespace(input);
    parse_data_block(input)
}

fn parse_data_block(input: &mut Position) -> Result<DataBlock, ()> {
    start_with(input, "data_")?;
    let identifier = parse_identifier(input);
    let mut block = DataBlock {
        name: identifier.to_string(),
        items: Vec::new(),
    };
    loop {
        if input.text == "" {
            return Ok(block);
        }
        trim_comments_and_whitespace(input);
        let item = parse_data_item_or_saveframe(input)?;
        block.items.push(item);
    }
}

fn parse_data_item_or_saveframe(input: &mut Position) -> Result<Item, ()> {
    if let Ok(()) = start_with(input, "save_") {
        let mut frame = SaveFrame { items: Vec::new() };
        while !input.text.is_empty() && input.text.starts_with('_') {
            let item = parse_data_item(input)?;
            trim_comments_and_whitespace(input);
            frame.items.push(item);
        }
        if let Ok(()) = start_with(input, "save_") {
            Ok(Item::SaveFrame(frame))
        } else {
            Err(())
        }
    } else {
        let item = parse_data_item(input)?;
        Ok(Item::DataItem(item))
    }
}

fn parse_data_item(input: &mut Position) -> Result<DataItem, ()> {
    if let Ok(()) = start_with(input, "_") {
        let name = parse_identifier(input);
        trim_comments_and_whitespace(input);

        if let Ok(()) = start_with(input, "loop_") {
            let mut loop_value = Loop {
                header: Vec::new(),
                data: Vec::new(),
            };
            trim_comments_and_whitespace(input);

            while let Ok(()) = start_with(input, "_") {
                let inner_name = parse_identifier(input);
                trim_comments_and_whitespace(input);
                loop_value.header.push(inner_name.to_string());
            }

            while let Ok(value) = parse_value(input) {
                loop_value.data.push(value);
                trim_comments_and_whitespace(input);
            }

            Ok(DataItem {
                name: name.to_string(),
                content: MultiValue::Loop(loop_value),
            })
        } else if let Ok(value) = parse_value(input) {
            Ok(DataItem {
                name: name.to_string(),
                content: MultiValue::Value(value),
            })
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

fn parse_value(input: &mut Position) -> Result<Value, ()> {
    if input.text.is_empty() {
        Err(())
    } else if input.text.starts_with('.') {
        input.text = &input.text[1..];
        input.column += 1;
        Ok(Value::Inapplicable)
    } else if input.text.starts_with('?') {
        input.text = &input.text[1..];
        input.column += 1;
        Ok(Value::Unknown)
    } else if let Ok(num) = parse_numeric(input) {
        Ok(num)
    } else if let Ok(value) = parse_char_string(input) {
        Ok(Value::Text(value))
    } else if let Ok(value) = parse_text_field(input) {
        Ok(Value::Text(value))
    } else {
        Err(())
    }
}

fn parse_char_string(_input: &mut Position) -> Result<String, ()> {
    unimplemented!();
}

fn parse_text_field(_input: &mut Position) -> Result<String, ()> {
    unimplemented!();
}

fn parse_numeric(input: &mut Position) -> Result<Value, ()> {
    let mut chars_to_remove = 0;
    let first_char = input.text.chars().next().unwrap();
    // Parse a possible sign
    let mut minus = false;
    if first_char == '-' {
        minus = true;
        chars_to_remove += 1;
    } else if first_char == '+' {
        chars_to_remove += 1;
    }

    // Parse the integer part
    let mut integer_set = false;
    let mut value = 0;
    for c in input.text.chars().skip(chars_to_remove) {
        if c.is_ascii_digit() {
            integer_set = true;
            value *= 10;
            value += c.to_digit(10).unwrap();
            chars_to_remove += 1;
        } else {
            break;
        }
    }

    // Now take the decimal part
    let mut decimal_set = false;
    let mut decimal = 0.0;
    if input.text.len() > chars_to_remove && input.text.chars().nth(chars_to_remove).unwrap() == '.'
    {
        chars_to_remove += 1;
        let mut power: f64 = 1.0;
        for c in input.text.chars().skip(chars_to_remove) {
            if c.is_ascii_digit() {
                decimal_set = true;
                power *= 10.0;
                decimal += c.to_digit(10).unwrap() as f64 / power;
                chars_to_remove += 1;
            } else {
                break;
            }
        }
    }

    // Now take the exponent
    let mut exponent_set = false;
    let mut exponent = 0;
    if input.text.len() > chars_to_remove {
        let next_char = input.text.chars().nth(chars_to_remove).unwrap();
        if next_char == 'e' || next_char == 'E' {
            // Parse a possible sign
            chars_to_remove += 1;
            if input.text.len() == chars_to_remove {
                return Err(()); // No number after the exponent
            }
            let exp_first_char = input.text.chars().nth(chars_to_remove).unwrap();
            let mut exp_minus = false;
            if exp_first_char == '-' {
                exp_minus = true;
                chars_to_remove += 1;
            } else if exp_first_char == '+' {
                chars_to_remove += 1;
            }

            // Parse the integer part
            let mut exp_value = 0;
            for c in input.text.chars().skip(chars_to_remove) {
                if c.is_ascii_digit() {
                    exponent_set = true;
                    exp_value *= 10;
                    exp_value += c.to_digit(10).unwrap();
                    chars_to_remove += 1;
                } else {
                    break;
                }
            }
            #[allow(clippy::cast_possible_wrap)]
            if exp_minus {
                exponent = -(exp_value as i32);
            } else {
                exponent = exp_value as i32;
            }
        }
    }

    // Take the uncertainty
    let mut uncertainty_set = false;
    let mut uncertainty = 0;
    if input.text.len() > chars_to_remove && input.text.chars().nth(chars_to_remove).unwrap() == '('
    {
        uncertainty_set = true;
        chars_to_remove += 1;
        for c in input.text.chars().skip(chars_to_remove) {
            if c.is_ascii_digit() {
                uncertainty *= 10;
                uncertainty += c.to_digit(10).unwrap();
                chars_to_remove += 1;
            } else {
                break;
            }
        }
        if input.text.len() == chars_to_remove
            || input.text.chars().nth(chars_to_remove).unwrap() != ')'
        {
            return Err(());
        }
        chars_to_remove += 1;
    }

    if !integer_set && !decimal_set {
        Err(())
    } else {
        input.text = &input.text[chars_to_remove..];
        input.column += chars_to_remove;

        let mut number = value as f64 + decimal;
        if minus {
            number *= -1.0;
        }
        if exponent_set {
            number *= 10_f64.powi(exponent);
        }
        if uncertainty_set {
            Ok(Value::NumericWithUncertainty(number, uncertainty))
        } else {
            Ok(Value::Numeric(number))
        }
    }
}

fn parse_identifier<'a>(input: &mut Position<'a>) -> &'a str {
    let mut chars_to_remove = 0;

    for c in input.text.chars() {
        if c.is_ascii_whitespace() {
            let identifier = &input.text[..chars_to_remove];
            input.text = &input.text[chars_to_remove..];
            input.column += chars_to_remove;
            return identifier;
        } else {
            chars_to_remove += 1;
        }
    }

    let identifier = input.text;
    input.text = "";
    input.column += chars_to_remove;
    identifier
}

fn start_with<'a, 'b>(input: &mut Position<'a>, pattern: &'b str) -> Result<(), ()> {
    if input.text.len() < pattern.len() {
        Err(())
    } else {
        for (p, c) in pattern.chars().zip(input.text.chars()) {
            if p != c.to_ascii_lowercase() {
                return Err(());
            }
        }
        input.text = &input.text[pattern.len()..];
        input.column += pattern.len();
        Ok(())
    }
}

fn trim_comments_and_whitespace(input: &mut Position) {
    loop {
        trim_whitespace(input);
        if input.text == "" {
            return;
        }
        if input.text.starts_with('#') {
            skip_to_eol(input);
        } else {
            return;
        }
    }
}

fn skip_to_eol(input: &mut Position) {
    let mut chars_to_remove = 0;
    let mut eol = false;

    for c in input.text.chars() {
        if c == '\r' || c == '\n' {
            if eol {
                input.text = &input.text[chars_to_remove..];
                input.line += 1;
                input.column = 0;
                return;
            } else {
                chars_to_remove += 1;
                eol = true;
            }
        } else {
            if eol {
                input.text = &input.text[chars_to_remove..];
                input.line += 1;
                input.column = 0;
                return;
            }
            chars_to_remove += 1;
        }
    }

    input.text = "";
    input.column += chars_to_remove;
}

fn trim_whitespace(input: &mut Position) {
    let mut chars_to_remove = 0;
    let mut eol = false;

    for c in input.text.chars() {
        if c == ' ' || c == '\t' {
            input.column += 1;
            chars_to_remove += 1;
            eol = false;
        } else if c == '\r' || c == '\n' {
            if eol {
                chars_to_remove += 1;
                eol = false;
            } else {
                input.column = 0;
                input.line += 1;
                chars_to_remove += 1;
                eol = true;
            }
        } else {
            input.text = &input.text[chars_to_remove..];
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_numeric {
        ($res:expr, $exp:expr) => {
            if let Ok(Value::Numeric(n)) = $res {
                if !close(n, $exp) {
                    panic!("assertion failed: {} is not close to {}", n, $exp);
                }
            } else {
                panic!("assertion failed: {:?} is Err", $res);
            }
        };
        ($res:expr, $exp:expr, $un:expr) => {
            if let Ok(Value::NumericWithUncertainty(n, u)) = $res {
                if !close(n, $exp) {
                    panic!("assertion failed: {} is not close to {}", n, $exp);
                }
                if u != $un {
                    panic!("assertion failed: {} is not equal to {}", u, $un);
                }
            } else {
                panic!("assertion failed: {:?} is Err", $res);
            }
        };
    }

    #[test]
    fn trim_whitespace_only_spaces() {
        let mut pos = Position {
            text: "    a",
            line: 0,
            column: 0,
        };
        trim_whitespace(&mut pos);
        assert_eq!(pos.text, "a");
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 4);
    }

    #[test]
    fn trim_whitespace_tabs_and_spaces() {
        let mut pos = Position {
            text: " \t \t a",
            line: 0,
            column: 0,
        };
        trim_whitespace(&mut pos);
        assert_eq!(pos.text, "a");
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 5);
    }

    #[test]
    fn trim_whitespace_newlines() {
        let mut pos = Position {
            text: " \t \t \n \r \n\r \r\na",
            line: 0,
            column: 0,
        };
        trim_whitespace(&mut pos);
        assert_eq!(pos.text, "a");
        assert_eq!(pos.line, 4);
        assert_eq!(pos.column, 0);
    }

    #[test]
    fn skip_to_eol_test() {
        let mut pos = Position {
            text: "bla bla bla\na",
            line: 0,
            column: 0,
        };
        skip_to_eol(&mut pos);
        assert_eq!(pos.text, "a");
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 0);
    }

    #[test]
    fn trim_comments_and_whitespace_test() {
        let mut pos = Position {
            text: "  \n#comment\n  #comment\na",
            line: 0,
            column: 0,
        };
        trim_comments_and_whitespace(&mut pos);
        assert_eq!(pos.text, "a");
        assert_eq!(pos.line, 3);
        assert_eq!(pos.column, 0);
    }

    #[test]
    fn start_with_true() {
        let mut pos = Position {
            text: "BloCk_a",
            line: 0,
            column: 0,
        };
        let res = start_with(&mut pos, "block_");
        assert!(res.is_ok());
        assert_eq!(pos.text, "a");
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 6);
    }

    #[test]
    fn start_with_false() {
        let mut pos = Position {
            text: "loop_a",
            line: 0,
            column: 0,
        };
        let res = start_with(&mut pos, "block_");
        assert!(res.is_err());
        assert_eq!(pos.text, "loop_a");
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 0);
    }

    #[test]
    fn parse_identifier_test0() {
        let mut pos = Position {
            text: "hello_world a",
            line: 0,
            column: 0,
        };
        let res = parse_identifier(&mut pos);
        assert_eq!(res, "hello_world");
        assert_eq!(pos.text, " a");
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 11);
    }

    #[test]
    fn parse_identifier_test1() {
        let mut pos = Position {
            text: " a",
            line: 0,
            column: 0,
        };
        let res = parse_identifier(&mut pos);
        assert_eq!(res, "");
        assert_eq!(pos.text, " a");
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 0);
    }

    #[test]
    fn parse_numeric_integer() {
        let mut pos = Position {
            text: "42",
            line: 0,
            column: 0,
        };
        let res = parse_numeric(&mut pos);
        assert_numeric!(res, 42.0);
        assert_eq!(pos.text, "");
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 2);
    }

    #[test]
    fn parse_numeric_float_no_decimal() {
        let mut pos = Position {
            text: "42.",
            line: 0,
            column: 0,
        };
        let res = parse_numeric(&mut pos);
        assert_numeric!(res, 42.0);
        assert_eq!(pos.text, "");
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 3);
    }

    #[test]
    fn parse_numeric_float_no_integer() {
        let mut pos = Position {
            text: ".42",
            line: 0,
            column: 0,
        };
        let res = parse_numeric(&mut pos);
        assert_numeric!(res, 0.42);
        assert_eq!(pos.text, "");
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 3);
    }

    #[test]
    fn parse_numeric_float_exp() {
        let mut pos = Position {
            text: "42e1",
            line: 0,
            column: 0,
        };
        let res = parse_numeric(&mut pos);
        assert_numeric!(res, 420.0);
        assert_eq!(pos.text, "");
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 4);
    }

    #[test]
    fn parse_numeric_float_no_decimal_exp() {
        let mut pos = Position {
            text: "42.e10",
            line: 0,
            column: 0,
        };
        let res = parse_numeric(&mut pos);
        assert_numeric!(res, 42.0 * 10000000000.0);
        assert_eq!(pos.text, "");
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 6);
    }

    #[test]
    fn parse_numeric_float_no_integer_exp() {
        let mut pos = Position {
            text: ".42e10",
            line: 0,
            column: 0,
        };
        let res = parse_numeric(&mut pos);
        assert_numeric!(res, 0.42 * 10000000000.0);
        assert_eq!(pos.text, "");
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 6);
    }

    #[test]
    fn parse_numeric_float_no_integer_positive_exp() {
        let mut pos = Position {
            text: ".42e+10",
            line: 0,
            column: 0,
        };
        let res = parse_numeric(&mut pos);
        assert_numeric!(res, 0.42 * 10000000000.0);
        assert_eq!(pos.text, "");
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 7);
    }

    #[test]
    fn parse_numeric_float_no_integer_negative_exp() {
        let mut pos = Position {
            text: ".42e-10",
            line: 0,
            column: 0,
        };
        let res = parse_numeric(&mut pos);
        assert_numeric!(res, 0.42 / 10000000000.0);
        assert_eq!(pos.text, "");
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 7);
    }

    #[test]
    fn parse_numeric_float_uncertainty() {
        let mut pos = Position {
            text: "42.0(9)",
            line: 0,
            column: 0,
        };
        let res = parse_numeric(&mut pos);
        assert_numeric!(res, 42.0, 9);
        assert_eq!(pos.text, "");
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 7);
    }

    #[test]
    fn parse_numeric_float_uncertainty_missing_bracket() {
        let mut pos = Position {
            text: "42.0(9",
            line: 0,
            column: 0,
        };
        let res = parse_numeric(&mut pos);
        assert!(res.is_err());
        assert_eq!(pos.text, "42.0(9");
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 0);
    }

    #[test]
    fn parse_numeric_float_huge_uncertainty() {
        let mut pos = Position {
            text: "42.0(97895)",
            line: 0,
            column: 0,
        };
        let res = parse_numeric(&mut pos);
        assert_numeric!(res, 42.0, 97895);
        assert_eq!(pos.text, "");
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 11);
    }

    #[test]
    fn parse_numeric_missing_numbers0() {
        let mut pos = Position {
            text: ".",
            line: 0,
            column: 0,
        };
        let res = parse_numeric(&mut pos);
        assert!(res.is_err());
        assert_eq!(pos.text, ".");
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 0);
    }

    #[test]
    fn parse_numeric_missing_numbers1() {
        let mut pos = Position {
            text: ".e",
            line: 0,
            column: 0,
        };
        let res = parse_numeric(&mut pos);
        assert!(res.is_err());
        assert_eq!(pos.text, ".e");
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 0);
    }

    #[test]
    fn parse_numeric_missing_numbers2() {
        let mut pos = Position {
            text: ".e42",
            line: 0,
            column: 0,
        };
        let res = parse_numeric(&mut pos);
        assert!(res.is_err());
        assert_eq!(pos.text, ".e42");
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 0);
    }

    // Now test the higher order functions as well (text fields and up...)

    fn close(a: f64, b: f64) -> bool {
        let dif = a / b;
        (1.0 - dif) > -0.000000000000001 && (dif - 1.0) < 0.000000000000001
    }
}
