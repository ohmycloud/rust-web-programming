use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub unique_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub unique_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrimmedUser {
    pub id: i32,
    pub email: String,
    pub unique_id: String,
}

impl From<User> for TrimmedUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            unique_id: user.unique_id,
        }
    }
}

impl NewUser {
    pub fn new(email: String, password: String) -> Result<NewUser, NanoServiceError> {
        let unique_id = uuid::Uuid::new_v4().to_string();
        let salt = SaltString::generate(&mut rand::thread_rng());
        let argon2_hasher = Argon2::default();
        let hash = argon2_hasher
            .hash_password(password.as_bytes(), &salt)
            .map_err(|err| {
                NanoServiceError::new(
                    format!("Failed to hash password: {}", err),
                    NanoServiceErrorStatus::Unknown,
                )
            })?
            .to_string();

        Ok(Self {
            email,
            password: hash,
            unique_id,
        })
    }
}

impl User {
    pub fn verify_password(&self, password: String) -> Result<bool, NanoServiceError> {
        let argon2_hasher = Argon2::default();
        let parsed_hash = PasswordHash::new(&self.password).map_err(|err| {
            NanoServiceError::new(
                format!("Failed to parse password hash: {}", err),
                NanoServiceErrorStatus::Unknown,
            )
        })?;
        let is_valid = argon2_hasher
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();
        Ok(is_valid)
    }
}
