use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::errors::{ErrorMessage, HttpError};

/// JWT signing algorithm used across the application
const ALGORITH_SET: Algorithm = Algorithm::HS256;

/// Claims stored inside the JWT token.
///
/// - `sub`: subject (user identifier)
/// - `iat`: issued-at timestamp (unix seconds)
/// - `exp`: expiration timestamp (unix seconds)
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
}

/// Creates a signed JWT for the given user.
///
/// # Arguments
/// - `user_id` – Unique identifier of the user (stored as `sub`)
/// - `secret` – HMAC secret used to sign the token
/// - `expires_in_minutes` – Token lifetime in minutes
///
/// # Errors
/// Returns an error if:
/// - the user id is empty
/// - token encoding fails
pub fn create_token(
    user_id: &str,
    secret: &[u8],
    expires_in_minutes: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    if user_id.is_empty() {
        return Err(jsonwebtoken::errors::ErrorKind::InvalidSubject.into());
    }

    let now = Utc::now();
    let iat = now.timestamp();
    let exp = (now + Duration::minutes(expires_in_minutes)).timestamp();
    let claims: TokenClaims = TokenClaims {
        sub: user_id.to_string(),
        iat,
        exp,
    };

    let key = &EncodingKey::from_secret(secret);
    let header = &Header::new(ALGORITH_SET);
    encode(header, &claims, key)
}

/// Decodes and validates a JWT token.
///
/// # Arguments
/// - `token` – JWT string received from the client
/// - `secret` – HMAC secret used to verify the token signature
///
/// # Returns
/// - `Ok(user_id)` if the token is valid
/// - `Err(HttpError)` if the token is invalid or expired
///
/// # Security notes
/// - Uses default validation (including `exp` check with leeway)
/// - All JWT errors are intentionally mapped to a generic 401 response
pub fn decode_token<T: Into<String>>(token: T, secret: &[u8]) -> Result<String, HttpError> {
    let decoding_key = &DecodingKey::from_secret(secret);
    let validation = &Validation::new(ALGORITH_SET);
    let decoded = decode::<TokenClaims>(&token.into(), decoding_key, validation);

    match decoded {
        Ok(token) => Ok(token.claims.sub),
        Err(_) => Err(HttpError::new(ErrorMessage::InvalidToken.to_string(), 401)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECRET: &[u8] = b"super-secret-key";

    #[test]
    fn create_token_success() {
        let token = create_token("user123", SECRET, 10);
        assert!(token.is_ok());
    }
    #[test]
    fn create_token_fails_with_emtpy_user_id() {
        let token = create_token("", SECRET, 10);
        assert!(token.is_err());
    }
    #[test]
    fn decode_token_success() {
        let user_id = "user123";
        let token = create_token(user_id, SECRET, 10).unwrap();
        let result = decode_token(&token, SECRET);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), user_id);
    }
    #[test]
    fn decode_token_fails_with_wrong_secret() {
        let token = create_token("user123", SECRET, 10).unwrap();
        let wrong_secret = b"wrong-secret";

        let result = decode_token(token, wrong_secret);

        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err.status, 401);
    }

    #[test]
    fn decode_token_fails_when_expired() {
        // Create token already expired
        let now = Utc::now();
        let claims = TokenClaims {
            sub: "user123".to_string(),
            iat: now.timestamp(),
            exp: (now - Duration::minutes(2)).timestamp(),
        };

        let token = encode(
            &Header::new(ALGORITH_SET),
            &claims,
            &EncodingKey::from_secret(SECRET),
        )
        .unwrap();

        let result = decode_token(token, SECRET);

        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err.status, 401);
    }
}
