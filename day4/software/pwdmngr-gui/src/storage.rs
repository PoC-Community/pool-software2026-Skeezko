use crate::crypto::CryptoManager;
use crate::models::PasswordStore;
use rand::Rng;
use std::usize;
use std::{fs, path::Path};

pub struct Storage {
    pub crypto: CryptoManager,
}

const STORAGE_FILE_NAME: &str = "credentials.encrypted";

impl Storage {
    pub fn load(&self) -> Result<PasswordStore, String> {
        if Path::new(STORAGE_FILE_NAME).exists() {
            let loaded_file =
                fs::read(STORAGE_FILE_NAME).map_err(|e| format!("Failed to read file: {}", e))?;
            let decrypted_file = self.crypto.decrypt(loaded_file.into())?;

            let res = serde_json::from_str(&decrypted_file)
                .map_err(|e| format!("Failed to deserialize: {}", e))?;
            Ok(res)
        } else {
            Ok(PasswordStore {
                entries: Vec::new(),
            })
        }
    }

    pub fn save(&self, pwd: PasswordStore) -> Result<(), String> {
        let serializing =
            serde_json::to_string(&pwd).map_err(|e| format!("Failed to serialize: {}", e))?;
        let encryption = self.crypto.encrypt(&serializing)?;
        let _ = fs::write(STORAGE_FILE_NAME, encryption)
            .map_err(|e| format!("Failed to write: {}", e))?;

        Ok(())
    }

    pub fn generate_password(&self, len: usize) -> String {
        let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()-_=+[]{}|;:,.<>?";
        let mut rng = rand::thread_rng();
        (0..len)
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset[idx] as char
            })
            .collect()
    }
}
