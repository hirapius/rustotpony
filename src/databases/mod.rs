pub mod encrypted;
pub mod json;

use generators::TOTP;

use std::collections::HashMap;

// Database trait
pub trait Database {
    fn get_applications(&self) -> HashMap<String, TOTP>;
    fn save_applications(&self, applications: &HashMap<String, TOTP>);
}
