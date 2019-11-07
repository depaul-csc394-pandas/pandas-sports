use lazy_static::lazy_static;
use std::{env, io::Read};

#[cfg(not(test))]
lazy_static! {
    pub static ref PEPPER: [u8; 32] = {
        dotenv::dotenv().ok();
        let mut s = env::var("PEPPER").expect("PEPPER not set");
        let mut data = [0; 32];
        data.copy_from_slice(s.as_bytes());
        data
    };
    pub static ref COOKIE_KEY: [u8; 32] = {
        dotenv::dotenv().ok();
        let mut s = env::var("COOKIE_KEY").expect("COOKIE_KEY not set");
        let mut data = [0; 32];
        data.copy_from_slice(s.as_bytes());
        data
    };
}

// we don't have access to Docker secrets in the test environment, so we hardcode
// a different set of secrets to be used in test builds.
#[cfg(test)]
lazy_static! {
    pub static ref PEPPER: [u8; 32] = {
        let pepper_str = "NAqdplo5YPcZ84UbCCvWH9OOTJOXAEzr".to_string();
        let mut bytes = [0; 32];
        pepper_str.as_bytes().read(&mut bytes).unwrap();
        bytes
    };
    pub static ref COOKIE_KEY: [u8; 32] = {
        let cookie_key_str = "ChzeqPjoSsrdO5xZ14gMoaW67yMn5Ev1".to_string();
        let mut bytes = [0; 32];
        cookie_key_str.as_bytes().read(&mut bytes).unwrap();
        bytes
    };
}
