use std::usize;
use std::{fs, path::Path};
use rand::Rng;

use crate::models::PasswordStore;

use crate::crypto::CryptoManager;

pub struct Storage {
    pub cyrpto: CryptoManager
}

const STORAGE_FILE_NAME: &str = ".pwdmgr_store.encrypted";

impl Storage {
    pub fn load(&self) -> PasswordStore {

        if Path::new(STORAGE_FILE_NAME).exists() {
            let loaded_file = fs::read(STORAGE_FILE_NAME).expect("Failed to read file");
            let decrypted_file = self.cyrpto.decrypt(loaded_file.into());

            let res = serde_json::from_str(&decrypted_file).expect("Failed to deserialize");
            res
        } else {
            return PasswordStore {entries: Vec::new()}
        }
    }


    pub fn save(&self, pwd: PasswordStore) {

        let serializing = serde_json::to_string(&pwd).expect("Failed to serialize");
        let encryption = self.cyrpto.encrypt(&serializing);
        let _ = fs::write(STORAGE_FILE_NAME, encryption).expect("Failed to write");
    }

    pub fn generate_password(&self, len: usize) -> String {
        let charset = b"AZERTYUIOPQSDFGHJKLMWXCVBNazertyuiopmlkjhgfdsqwxcvbn1234567890_!$()&-/:*%.;,?^+=}]@`|[{#~";
        let mut rng = rand::thread_rng();
        (0..len).map(|_| {
            let idx= rng.gen_range(0..charset.len());
            charset[idx] as char
        }).collect()
    }
}