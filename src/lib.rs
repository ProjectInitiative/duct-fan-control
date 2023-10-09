#![no_std]
#![no_main]
#![macro_use]
#![feature(type_alias_impl_trait)]

use cocoon::{Error, MiniCocoon};

include!("secret.rs");

const MAX_DATA_LEN: usize = 64; // Adjust this size as needed

pub fn encrypt_message(data_str: &str) -> Result<[u8; cocoon::MINI_PREFIX_SIZE], Error> {
    let cocoon = make_cocoon();

    match cocoon.encrypt(&mut msg_to_bytes(data_str)) {
        Ok(detached_prefix) => Ok(detached_prefix),
        Err(e) => return core::prelude::v1::Err(e),
    }
    // assert_ne!(data, b"my secret data");
}

pub fn decrypt_message(detached_prefix: &[u8]) -> Result<[u8; MAX_DATA_LEN], Error> {
    let cocoon = make_cocoon();
    let mut data = [0u8; MAX_DATA_LEN];
    match cocoon.decrypt(&mut data, detached_prefix) {
        Ok(()) => Ok(data),
        Err(e) => return core::prelude::v1::Err(e),
    }
}

pub fn msg_to_bytes(data_str: &str) -> [u8; MAX_DATA_LEN] {
    // let mut data = "my secret data".to_owned().into_bytes();
    let mut data = [0u8; MAX_DATA_LEN];

    // Copy the string's bytes into the fixed-size array
    let data_len = data_str.len().min(MAX_DATA_LEN);
    data[0..data_len].copy_from_slice(data_str.as_bytes());
    data
}

fn make_cocoon() -> MiniCocoon {
    MiniCocoon::from_key(ENCRYPTION_KEY, &[0; 32])
}
