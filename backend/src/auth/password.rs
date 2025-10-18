use rocket::http::Status;

use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

/// Hashes password
pub(super) fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

/// Compares a password hash to an unhashed password for verification
pub(super) fn verify_password(hash: &str, password: &str) -> Result<(), Status> {
    let parsed_hash = PasswordHash::new(hash).unwrap();
    if !Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
    {
        return Err(Status::Unauthorized);
    }

    Ok(())
}
