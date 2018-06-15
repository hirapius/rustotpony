#![feature(fs_read_write)]
#![feature(extern_prelude)]

extern crate crypto;
extern crate rand;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

pub mod databases;
mod generators;

use databases::Database;
use generators::TOTP;

use std::collections::HashMap;


// Application struct
// Contains database reference and in-memory generators (called «applications»)
pub struct RusTOTPony<DB: Database> {
    database: DB,
    applications: HashMap<String, TOTP>,
}

impl<DB: Database> RusTOTPony<DB> {
    pub fn new(db: DB) -> RusTOTPony<DB> {
        RusTOTPony {
            applications: db.get_applications(),
            database: db,
        }
    }

    pub fn create_application(
        &mut self,
        name: &str,
        username: &str,
        secret: &str,
    ) -> Result<(), String> {
        let new_app = TOTP::new_base32(name, username, secret)?;
        if self.applications.contains_key(name) {
            Err(format!("Application with name '{}' already exists!", name))
        } else {
            &self.applications.insert(String::from(name), new_app);
            Ok(())
        }
    }

    pub fn delete_application(&mut self, name: &str) -> Result<(), String> {
        if let Some(_) = self.applications.remove(name) {
            Ok(())
        } else {
            Err(format!(
                "Application with the name '{}' doesn't exist",
                name
            ))
        }
    }

    pub fn rename_application(&mut self, name: &str, newname: &str) -> Result<(), String> {
        if let Some(app) = self.applications.get_mut(name) {
            app.set_name(newname);
            Ok(())
        } else {
            Err(format!("Application '{}' wasn't found", name))
        }
    }

    pub fn get_applications(&self) -> Result<&HashMap<String, TOTP>, String> {
        if self.applications.len() == 0 {
            Err(String::from("There are no applications"))
        } else {
            Ok(&self.applications)
        }
    }

    pub fn get_application(&self, name: &str) -> Result<&TOTP, String> {
        if let Some(app) = self.applications.get(name) {
            Ok(app)
        } else {
            Err(format!("Application '{}' wasn't found", name))
        }
    }

    pub fn delete_all_applications(&mut self) {
        self.applications = HashMap::new();
    }

    pub fn flush(&self) {
        &self.database.save_applications(&self.applications);
    }
}

// Application → Database (JsonDatabase, EncryptedDatabase)
//     ↓            ↓
//  GeneratorApplication
