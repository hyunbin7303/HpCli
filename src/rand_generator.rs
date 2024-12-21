use rand::{distributions::Alphanumeric, prelude::Distribution, Rng};
const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
abcdefghijklmnopqrstuvwxyz\
0123456789)(*&^%$#@!~";
const PASSWORD_LEN: usize = 30;

fn just_test() {
    let result = (0..10).flat_map(|_| {
        let vec: Vec<String> = vec!["abc".into(), "BCDEF".into(), "UserInformation".into()];
        vec.into_iter() // what is the purpose of this?
    }).collect::<Vec<_>>(); // using Flat mapp and into Iter for testing purpose.
    println!("{:?}", result);
}

pub fn get_random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter::<char, _>(rand::distributions::Standard)
        .take(len)
        .collect()
}
pub fn get_random_pwd(len: usize) -> String {
    let mut rng = rand::thread_rng();
    let password: String = (0..len)
        .map(|_| {
        let idx = rng.gen_range(0..CHARSET.len());
        CHARSET[idx] as char
        })
    .collect();
    return password
}
fn get_sample_in_string() -> String {
    let mut rng = rand::thread_rng();
    // let v: Vec<f32> = Standard.sample_iter(&mut rng).take(10).collect(); for number
    return Alphanumeric.sample_iter(&mut rng).take(10).map(char::from).collect();
}



#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use crate::rand_generator;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789)(*&^%$#@!~";

    #[test]
    fn test_password_length() {
        let lengths = [0, 1, 5, 10, 20, 50];
        for &len in &lengths {
            let pwd = rand_generator::get_random_pwd(len);
            assert_eq!(pwd.len(), len, "Password length should match the requested length");
        }
    }

    #[test]
    fn test_password_charset() {
        let pwd = rand_generator::get_random_pwd(1000); // Large length to increase chance of using all characters
        let pwd_chars: HashSet<char> = pwd.chars().collect();
        let charset_chars: HashSet<char> = CHARSET.iter().map(|&c| c as char).collect();

        for &c in CHARSET {
            assert!(pwd_chars.contains(&(c as char)), "Password should only contain characters from CHARSET");
        }

        assert!(pwd_chars.is_subset(&charset_chars), "Password should only contain characters from CHARSET");
    }

    #[test]
    fn test_password_randomness() {
        let pwd1 = rand_generator::get_random_pwd(20);
        let pwd2 = rand_generator::get_random_pwd(20);
        assert_ne!(pwd1, pwd2, "Two generated passwords should not be identical");
    }

    #[test]
    fn test_empty_password() {
        let pwd = rand_generator::get_random_pwd(0);
        assert!(pwd.is_empty(), "Password of length 0 should be empty");
    }

    #[test]
    fn test_long_password() {
        let len = 1_000_000; // Very long password
        let pwd = rand_generator::get_random_pwd(len);
        assert_eq!(pwd.len(), len, "Should be able to generate very long passwords");
    }

    #[test]
    fn test_distribution() {
        let pwd = rand_generator::get_random_pwd(10000);
        let char_counts: HashMap<char, usize> = pwd.chars().fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });

        let expected_count = 10000 / CHARSET.len();
        for &c in CHARSET {
            let count = char_counts.get(&(c as char)).unwrap_or(&0);
            assert!(*count > expected_count / 2 && *count < expected_count * 2,
                    "Character distribution should be roughly even");
        }
    }
}