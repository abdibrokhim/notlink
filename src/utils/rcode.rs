// src/utils/rcode.rs
use rand::{Rng, thread_rng, distributions::Alphanumeric};

pub fn random_short_code(len: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}
