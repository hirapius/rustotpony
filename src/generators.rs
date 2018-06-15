extern crate base32;
extern crate oath;

// Generator application struct
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TOTP {
    name: String,
    secret: String,
    username: String,
    secret_bytes: Vec<u8>,
}

impl TOTP {
    pub fn new(name: &str, username: &str, secret: &str, secret_bytes: Vec<u8>) -> Self {
        TOTP {
            name: String::from(name),
            secret: String::from(secret),
            username: String::from(username),
            secret_bytes: secret_bytes,
        }
    }

    pub fn new_base32(
        name: &str,
        username: &str,
        base32_secret: &str,
    ) -> Result<TOTP, String> {
        if let Some(secret_bytes) = TOTP::base32_to_bytes(base32_secret) {
            Ok(TOTP::new(name, username, base32_secret, secret_bytes))
        } else {
            Err(String::from("Couldn't decode secret key"))
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn get_secret(&self) -> &str {
        self.secret.as_str()
    }

    pub fn get_username(&self) -> &str {
        self.username.as_str()
    }

    pub fn get_code(&self) -> u64 {
        Self::totp(&self.secret_bytes)
    }

    fn base32_to_bytes(secret: &str) -> Option<Vec<u8>> {
        base32::decode(base32::Alphabet::RFC4648 { padding: false }, secret)
    }

    fn totp(secret_bytes: &[u8]) -> u64 {
        oath::totp_raw_now(&secret_bytes, 6, 0, 30, &oath::HashType::SHA1)
    }
}
