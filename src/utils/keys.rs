// keys.rs
use shuttle_runtime::SecretStore;
use std::sync::OnceLock;

static PG_URL: OnceLock<String> = OnceLock::new();
static CRYPTO_KEY: OnceLock<[u8; 32]> = OnceLock::new();
static HOST: OnceLock<String> = OnceLock::new();

pub fn init_secrets(secrets: &SecretStore) {
    // Initialize DATABASE_URL
    let database_url = secrets.get("DATABASE_URL")
        .expect("DATABASE_URL not found in secrets");
    PG_URL.set(database_url.clone())
        .expect("DATABASE_URL already initialized");

    // Initialize CRYPTO_KEY
    let crypto_key = secrets.get("CRYPTO_KEY")
        .expect("CRYPTO_KEY not found in secrets");
    let key_bytes = hex::decode(&crypto_key)
        .expect("CRYPTO_KEY must be valid hex");
    if key_bytes.len() != 32 {
        panic!("CRYPTO_KEY must be 64 hex characters (32 bytes)");
    }
    let mut key = [0u8; 32];
    key.copy_from_slice(&key_bytes);
    CRYPTO_KEY.set(key)
        .expect("CRYPTO_KEY already initialized");

    // Initialize HOST
    let host = secrets.get("HOST")
        .expect("HOST not found in secrets");
    HOST.set(host.clone())
        .expect("HOST already initialized");

}

pub fn get_pg_url() -> &'static str {
    PG_URL.get().expect("DATABASE_URL not initialized")
}

pub fn get_crypto_key() -> &'static [u8; 32] {
    CRYPTO_KEY.get().expect("CRYPTO_KEY not initialized")
}

pub fn get_host() -> &'static str {
    HOST.get().expect("HOST not initialized")
}