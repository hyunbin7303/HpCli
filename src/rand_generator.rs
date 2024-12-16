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

fn get_random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter::<char, _>(rand::distributions::Standard)
        .take(len)
        .collect()
}
fn get_random_pwd() -> String {
    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::thread_rng();
    let password: String = (0..PASSWORD_LEN)
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