use std::ffi::{OsStr, OsString};

fn is_integer(arg_value: String) -> Result<(), String> {
    match arg_value.parse::<usize>() {
        Ok(_) => Ok(()),
        Err(_) => Err("Value provided is not an integer".to_string()),
    }
}


/// Check if the input provided is valid UTF-8
fn is_valid_string(os_str: &OsStr) -> Result<(), OsString> {
    match os_str.to_str() {
        Some(_) => Ok(()),
        None => Err(OsString::from("Value provided is not a valid UTF-8 string")),
    }
}

pub fn inspect(input: &String, digits: bool) -> (i32, String) {
    if !digits {
        return (input.len() as i32, String::from("char"));
    }
    return (inspect_numbers(input), String::from("digit"));
}
fn inspect_numbers(input: &String) -> i32 {
    let mut count = 0;
    for c in input.chars() {
        if c.is_digit(10) {
            count += 1;
        }
    }
    return count;
}