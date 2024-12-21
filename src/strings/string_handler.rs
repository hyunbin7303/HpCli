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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{ffi::{OsStr, OsString}, os::unix::ffi::OsStrExt};

    #[test]
    fn test_is_valid_string_utf8_string() {
        let valid_str = OsStr::new("Hello, world!");
        assert!(is_valid_string(valid_str).is_ok());
    }

    #[test]
    fn test_is_valid_string_empty_string_return_ok() {
        let empty_str = OsStr::new("");
        assert!(is_valid_string(empty_str).is_ok());
    }

    #[test]
    fn test_is_valid_string_unicode_string_return_ok() {
        let unicode_str = OsStr::new("안녕 세상아!");
        assert!(is_valid_string(unicode_str).is_ok());
    }

    #[test]
    fn test_is_valid_string_invalid_utf8_string_return_error() {
        // Create an invalid UTF-8 sequence
        let invalid_bytes = vec![0xFF, 0xFE, 0xFD];
        let invalid_os_str = OsStr::from_bytes(&invalid_bytes);

        let result = is_valid_string(invalid_os_str);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            OsString::from("Value provided is not a valid UTF-8 string")
        );
    }

    #[test]
    fn test_is_valid_string_mixed_valid_and_invalid() {
        // Mix valid and invalid UTF-8 sequences
        let mixed_bytes = b"Hello\xFF\xFEWorld";
        let mixed_os_str = OsStr::from_bytes(mixed_bytes);

        let result = is_valid_string(mixed_os_str);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            OsString::from("Value provided is not a valid UTF-8 string")
        );
    }
}