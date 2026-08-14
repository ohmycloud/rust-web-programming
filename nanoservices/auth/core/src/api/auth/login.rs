use auth_dal::users::GetByEmail;
use glue::{errors::NanoServiceError, errors::NanoServiceErrorStatus, token::HeaderToken};

pub async fn login<T: GetByEmail>(
    email: String,
    password: String,
) -> Result<String, NanoServiceError> {
    let user = T::get_by_email(email).await?;
    let outcome = user.verify_password(password)?;

    if outcome {
        Ok(HeaderToken {
            unique_id: user.unique_id,
        }
        .encode()?)
    } else {
        Err(NanoServiceError::new(
            "Invalid password".to_string(),
            NanoServiceErrorStatus::Unauthorized,
        ))
    }
}
