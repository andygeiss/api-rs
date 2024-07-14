use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::{thread_rng, RngCore};
use rusty_paseto::prelude::*;
use std::sync::OnceLock;

pub fn create_hash(password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    password_hash
}

pub fn create_token() -> String {
    let key = PasetoSymmetricKey::<V4, Local>::from(static_random_key().to_owned());
    PasetoBuilder::<V4, Local>::default().build(&key).unwrap()
}

pub fn is_password_valid(password_hash: String, password: String) -> bool {
    let parsed_hash = PasswordHash::new(&password_hash).unwrap();
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

pub fn is_token_valid(token: String) -> bool {
    let key = PasetoSymmetricKey::<V4, Local>::from(static_random_key().to_owned());
    let result = PasetoParser::<V4, Local>::default().parse(&token, &key);
    match result {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn static_random_key() -> &'static Key<32> {
    static KEY: OnceLock<Key<32>> = OnceLock::new();
    KEY.get_or_init(|| {
        let mut data = [0u8; 32];
        thread_rng().fill_bytes(&mut data);
        Key::<32>::from(data)
    })
}
