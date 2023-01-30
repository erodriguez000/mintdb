mod user;

extern crate ring;
use ring::pbkdf2;
use ring::rand::{SecureRandom, SystemRandom};
use ring::error::Unspecified;
use ring::constant_time::verify_slices_are_equal;

const ARGON2_SALT_LEN: usize = 16;
const ARGON2_OUTPUT_LEN: usize = 32;

pub fn hash_password(password: &str) -> Result<Vec<u8>, Unspecified> {
    let mut salt = [0u8; ARGON2_SALT_LEN];
    let rng = SystemRandom::new();
    rng.fill(&mut salt)?;

    let mut hashed_password = [0u8; ARGON2_OUTPUT_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        // Use a high number of iterations for added security
        std::num::NonZeroU32::new(100_000).unwrap(),
        &salt,
        password.as_bytes(),
        &mut hashed_password,
    );
    let mut hash_salt = vec![];
    hash_salt.extend_from_slice(&salt);
    hash_salt.extend_from_slice(&hashed_password);

    Ok(hash_salt.to_vec())
}

pub fn verify_password(password: &str, hashed_password: &[u8]) -> Result<bool, Unspecified> {
    let mut expected_password = [0u8; ARGON2_OUTPUT_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        std::num::NonZeroU32::new(100_000).unwrap(),
        &hashed_password[..ARGON2_SALT_LEN],
        password.as_bytes(),
        &mut expected_password,
    );

    let result = verify_slices_are_equal(
        &expected_password,
        &hashed_password[ARGON2_SALT_LEN..],
    );

    Ok(result.is_ok())
}