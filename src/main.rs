use clap::{command, Arg};

pub struct Credentials {
    api_key: String,
    secret: String,
}
impl Credentials {
    pub fn new(api_key: String, secret: String) -> Self {
        Credentials { api_key, secret }
    }
    pub fn to_string(&self) -> String {
        format!("Credentials({})", &self.api_key)
    }
}



fn main() {
    let creds = Credentials::new(String::from("SOME_API_KEY"), String::from("SOME_SECRET"));
    println!("{}", creds.to_string());

    let match_result = command!()
        .arg(Arg::new("firstname").short('f').long("first-name"))
        .arg(Arg::new("lastname").short('l').long("last-name"))
        .arg(Arg::new("fluffy").long("fluffy"))
        .get_matches();
}


