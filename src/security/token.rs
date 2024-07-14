use rand::{thread_rng, RngCore};
use rusty_paseto::prelude::*;
use std::sync::OnceLock;

pub fn create() -> String {
    let key = PasetoSymmetricKey::<V4, Local>::from(static_random_key().to_owned());
    PasetoBuilder::<V4, Local>::default().build(&key).unwrap()
}

pub fn is_valid(token: String) -> bool {
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
