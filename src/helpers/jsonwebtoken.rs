use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, errors::Error, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Serialize,Deserialize};

/// Secret key for signing tokens
const SECRET_KEY: &str = "your-secret-key";

#[derive(Serialize,Deserialize,Debug)]
pub struct Claims {
  sub: String,
  exp: usize
}

pub fn generate_token(data: &str) -> Result<String, Error> {
  let expiration = Utc::now()
    .checked_add_signed(Duration::hours(24))
    .expect("Failed to compute expiration")
    .timestamp() as usize;

  let claims = Claims {
    sub: data.to_owned(),
    exp: expiration,  
  };

  let token = encode(
    &Header::default(), 
    &claims, 
    &EncodingKey::from_secret(SECRET_KEY.as_ref()));
    println!("{}", token.clone().unwrap());
  token
}

pub fn verify_jwt(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
  let token_data = decode::<Claims>(
    token, 
    &DecodingKey::from_secret(SECRET_KEY.as_ref()), 
    &Validation::default()
  );
  token_data
}