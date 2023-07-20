use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: Uuid,
    pub exp: usize,
}

pub fn create_tokens(user_id: Uuid, secret: String) -> (String, String) {
    let header = Header::new(Algorithm::HS256);

    let secret = secret.as_bytes();
    let encoding_key = EncodingKey::from_secret(secret);

    let expiration = 3600; // 1 hour
    let expiration = chrono::Utc::now().naive_utc() + chrono::Duration::seconds(expiration as i64);
    let expiration = expiration.timestamp() as usize;
    let access_claims = Claims {
        user_id,
        exp: expiration,
    };
    let access_token = encode(&header, &access_claims, &encoding_key).unwrap();

    let expiration = 604800; // 1 week
    let expiration = chrono::Utc::now().naive_utc() + chrono::Duration::seconds(expiration as i64);
    let expiration = expiration.timestamp() as usize;
    let refresh_claims = Claims {
        user_id,
        exp: expiration,
    };
    let refresh_token = encode(&header, &refresh_claims, &encoding_key).unwrap();

    (access_token, refresh_token)
}

pub fn decode_access_token(
    access_token: &str,
    secret: String,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = secret.as_bytes();
    let token_data: jsonwebtoken::TokenData<Claims> = decode::<Claims>(
        access_token,
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(token_data.claims)
}
