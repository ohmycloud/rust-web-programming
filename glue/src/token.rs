#[cfg(feature = "actix")]
use crate::errors::{NanoServiceError, NanoServiceErrorStatus};
#[cfg(feature = "actix")]
use actix_web::{FromRequest, HttpRequest, dev::Payload};
#[cfg(feature = "actix")]
use futures::future::{Ready, err, ok};

pub struct HeaderToken {
    pub message: String,
}

impl FromRequest for HeaderToken {
    type Error = NanoServiceError;

    type Future = Ready<Result<HeaderToken, NanoServiceError>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let raw_data = match req.headers().get("token") {
            Some(data) => data,
            None => {
                return err(NanoServiceError::new(
                    "token not in header under key 'token'".to_string(),
                    NanoServiceErrorStatus::Unauthorized,
                ));
            }
        };
        let message = match raw_data.to_str() {
            Ok(token) => token.to_string(),
            Err(_) => {
                return err(NanoServiceError::new(
                    "Invalid token string".to_string(),
                    NanoServiceErrorStatus::Unauthorized,
                ));
            }
        };
        return ok(HeaderToken { message });
    }
}
