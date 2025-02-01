// src/utils/crypto.rs

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce
};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use crate::utils::keys::get_crypto_key;

// 256-bit key for AES-GCM
pub fn encrypt_url(plaintext: &str) -> Result<String, String> {
    // 1) Create cipher from static 256-bit key
    let key = Key::<Aes256Gcm>::from_slice(get_crypto_key());
    let cipher = Aes256Gcm::new(key);

    // using longer living value
    let rnum = rand::random::<[u8; 12]>();

    // 2) Generate random nonce
    let nonce = Nonce::from_slice(&rnum); // 96-bits
    // 3) Encrypt
    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|e| format!("encrypt error: {e}"))?;

    // 4) Encode: [nonce || ciphertext] in base64 for storing in DB
    let mut combined = Vec::new();
    combined.extend_from_slice(nonce);
    combined.extend_from_slice(&ciphertext);

    Ok(STANDARD.encode(&combined))
}

// Decrypts a base64-encoded ciphertext
pub fn decrypt_url(cipher_b64: &str) -> Result<String, String> {
    // 1) Decode base64
    let combined = STANDARD.decode(cipher_b64).map_err(|e| format!("base64 decode error: {e}"))?;
    if combined.len() < 12 {
        return Err("Ciphertext is too short".to_string());
    }

    // 2) Split out nonce and ciphertext
    let (nonce_bytes, ciphertext) = combined.split_at(12);

    let key = Key::<Aes256Gcm>::from_slice(get_crypto_key());
    let cipher = Aes256Gcm::new(key);

    let nonce = Nonce::from_slice(nonce_bytes);

    // 3) Decrypt
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| format!("decrypt error: {e}"))?;

    Ok(String::from_utf8(plaintext).map_err(|_| "invalid utf8".to_string())?)
}
