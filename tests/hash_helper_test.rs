#[path = "../src/hash_helper.rs"] mod hash_helper;

#[cfg(test)]
mod tests {
    use crate::hash_helper;

    #[test]
    fn test_encrypt_sha256_input_a_success() {
        let result = hash_helper::encrypt_sha256("a").unwrap();

        assert_eq!(result, "CA978112CA1BBDCAFAC231B39A23DC4DA786EFF8147C4E72B9807785AFEE48BB".to_string());
    }

}