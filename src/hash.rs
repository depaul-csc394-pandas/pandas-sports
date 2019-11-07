use crate::RNG;
use argon2rs::{verifier::Encoded, Argon2, Variant};
use ring::rand::SecureRandom;

/// Generate a random 32-byte salt value.
fn random_salt() -> [u8; 32] {
    let mut salt = [0; 32];
    (&*RNG).fill(&mut salt).unwrap();
    salt
}

fn argon2_session(salt: [u8; 32], password: &str) -> Encoded {
    Encoded::new(
        Argon2::default(Variant::Argon2d),
        password.as_bytes(),
        &salt,
        &*crate::secret::PEPPER,
        b"",
    )
}

pub struct SaltedHash {
    pub salt: [u8; 32],
    pub hash: Vec<u8>,
}

impl SaltedHash {
    /// Generate a random salt, then salt and pepper the password
    pub fn from_password(password: &str) -> SaltedHash {
        let salt = random_salt();
        let session = argon2_session(salt, password);
        let hash = session.to_u8();
        println!("Hash len: {}", hash.len());

        SaltedHash { salt, hash }
    }

    pub fn verify(&self, password: &str) -> bool {
        self.hash == argon2_session(self.salt, password).to_u8()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_verify() {
        let password = "some_other_password";
        let sh = SaltedHash::from_password(password);
        assert_eq!(sh.verify(password), true);
    }
}
