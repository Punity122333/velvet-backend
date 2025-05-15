use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
struct Claims {
    sub: String,
    role: String,
    // Add other claims Supabase includes
}

pub fn verify_token(token: &str, public_key: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_rsa_pem(public_key.as_bytes())?,
        &Validation::new(Algorithm::RS256),
    ).map(|data| data.claims)
}
