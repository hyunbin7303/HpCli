pub struct Credentials {
    pub api_key: String,
    pub secret: String,
}
impl Credentials {
    pub fn new(api_key: String, secret: String) -> Self {
        Credentials { api_key, secret }
    }
    pub fn to_string(&self) -> String {
        format!("Credentials({})", &self.api_key)
    }
}