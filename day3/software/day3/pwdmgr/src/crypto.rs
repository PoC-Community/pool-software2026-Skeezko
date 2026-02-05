use aes_gcm::aead::Aead;
use argon2::Argon2;
use aes_gcm::{Aes256Gcm, KeyInit};
use aes_gcm::AeadCore;
use rand::rngs::OsRng;

pub struct CryptoManager {
    cipher: Aes256Gcm,
}

impl CryptoManager {
    pub fn new(password: &str) -> Self {

        let salt = b"example salt";
        let mut key = [0u8; 32];
        Argon2::default().hash_password_into(password.as_bytes(), salt, &mut key).expect("Key derivation failure");

        let cipher = Aes256Gcm::new_from_slice(&key);
        CryptoManager { cipher: cipher.expect("REASON") }
    }


    pub fn encrypt(&self, data: &str) -> Vec<u8> {

        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = self.cipher.encrypt(&nonce, data.as_ref()).expect("Failed to encrypt");
        let mut res: Vec<u8> = nonce.to_vec();
        res.extend_from_slice(&ciphertext);
        res
    }

    pub fn decrypt(&self, encrypted: Vec<u8>) -> String {

        let data = encrypted.split_at(12);
        let nonce = aes_gcm::Nonce::from_slice(data.0);
        let ciphertext = data.1;
        
        let plaintext = self.cipher.decrypt(nonce, ciphertext.as_ref()).expect("Failed to decrypt");
        let res = match str::from_utf8(&plaintext) {
            Ok(string) => string,
            Err(e)  => panic!("Invalid UTF-8 sequence: {}", e)
        }; 
        res.to_string()
    }
}