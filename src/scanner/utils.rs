use crate::error_handler::{handle_error, ErrorType};

pub fn number(symbols: &Vec<char>,  start_char: char, index: &mut usize, n: &usize, line: &u32, has_err: &mut bool) -> String {
    let mut num = String::from(start_char);
    let mut last_char: char = start_char;
    while *index < *n {
        last_char = symbols[*index];
        if last_char.is_ascii_digit() {
            num.push(last_char);
            *index += 1;
        }
        else {
            break;
        }
    }
    if last_char == '.' {
        while *index < *n {
            last_char = symbols[*index];
            if last_char.is_ascii_digit() {
                num.push(last_char);
                *index += 1;
            }
            else {
                break;
            }
        }
    }
    if last_char.is_ascii_alphabetic() || last_char == '_' {
        handle_error(line, ErrorType::LexicalError, format!("Unexpected character: {last_char}").as_str());
        *has_err = true;
    }  
    num  
}

pub fn string(symbols: &Vec<char>, index: &mut usize, n: &usize, line: &u32, has_err: &mut bool) -> String {
    let mut val= String::from("");
    let mut end_string_found = false;
    while *index < *n {
        let c = symbols[*index];
        if c != '\"' {
            val.push(c);
            *index += 1;
        } 
        else {
            end_string_found = true;
            break;
        }
    }
    if !end_string_found {
        handle_error(line, ErrorType::LexicalError, "Unterminated string.");
        *has_err = true;
    }
    val
}

pub fn identifier(symbols: &Vec<char>, start_char: char, index: &mut usize, n: &usize, line: &u32, has_err: &mut bool) -> String {
    let mut ident = String::from(start_char);
    let mut last_char: char = start_char;
    while *index < *n {
        last_char = symbols[*index];
        if is_identifier_symbol(last_char) {
            ident.push(last_char);
            *index += 1;
        }
    }
    if !(last_char.is_whitespace() || is_identifier_symbol(last_char)) {
        handle_error(line, ErrorType::LexicalError, format!("Unexpected character: {last_char}").as_str());
        *has_err = true;
    }
    ident
}

fn is_identifier_symbol(c: char) -> bool {
    return c.is_ascii_alphanumeric() || c == '_';
}

pub fn literal_number(num: &str) -> String {
    let mut number = String::from(num);
    match num.find('.') {
        Some(i) => {
            if i + 1 == num.len() {
                number.push('0');
            }
        },
        None => {
            number.push_str(".0");
        }
    }
    number
}
