use jsonwebtoken::{
    decode, encode, get_current_timestamp, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use std::error::Error;

const EXPIRY_AFTER: usize = 60 * 60; //60 minutes in seconds

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JWTClaims {
    pub username: String,
    //mandatory field exp to denote the expiry of the token (UTC Timestamp)
    exp: usize,
}

pub fn get_token(username: impl Into<String>) -> Result<String, Box<dyn Error>> {
    let secret_token = std::env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN env var not found!");

    let my_claims = JWTClaims {
        username: username.into(),
        exp: (get_current_timestamp() as usize) + EXPIRY_AFTER,
    };

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(secret_token.as_bytes()),
    )?;

    Ok(token)
}

pub fn validate_token(token: &str) -> Result<JWTClaims, Box<dyn Error>> {
    let secret_token = std::env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN env var not found!");

    let token_data = decode::<JWTClaims>(
        &token,
        &DecodingKey::from_secret(secret_token.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}
