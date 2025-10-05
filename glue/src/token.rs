#[cfg(feature = "actix")]
use crate::errors::{NanoServiceError, NanoServiceErrorStatus};
#[cfg(feature = "actix")]
use actix_web::{FromRequest, HttpRequest, dev::Payload};
#[cfg(feature = "actix")]
use futures::future::{Ready, err, ok};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderToken {
    pub unique_id: String,
}

impl HeaderToken {
    pub fn get_key() -> Result<String, NanoServiceError> {
        std::env::var("JWT_SECRET").map_err(|err| {
            NanoServiceError::new(err.to_string(), NanoServiceErrorStatus::Unauthorized)
        })
    }

    pub fn encode(self) -> Result<String, NanoServiceError> {
        let jwt_secret = Self::get_key()?;
        let key = EncodingKey::from_secret(jwt_secret.as_ref());

        return match encode(&Header::default(), &self, &key) {
            Ok(token) => Ok(token),
            Err(err) => Err(NanoServiceError::new(
                err.to_string(),
                NanoServiceErrorStatus::Unauthorized,
            )),
        };
    }

    pub fn decode(token: &str) -> Result<Self, NanoServiceError> {
        let jwt_secret = Self::get_key()?;
        let key = DecodingKey::from_secret(jwt_secret.as_ref());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.required_spec_claims.remove("exp");

        match decode::<Self>(token, &key, &validation) {
            Ok(token) => Ok(token.claims),
            Err(err) => Err(NanoServiceError::new(
                err.to_string(),
                NanoServiceErrorStatus::Unauthorized,
            )),
        }
    }
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

        let token = match HeaderToken::decode(&message) {
            Ok(token) => token,
            Err(e) => return err(e),
        };

        return ok(token);
    }
}
