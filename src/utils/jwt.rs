use crate::utils::response_codes::RetError;
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts, TypedHeader},
    headers::Cookie,
};
use jsonwebtoken::{decode, encode, errors::Error, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = "dnwj^&T&*(HNQFhj1i23";
    Keys::new(secret.as_bytes())
});

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub user_id: i32,
    pub exp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthToken {
    access_token: String,
    token_type: String,
}

#[async_trait]
impl<B> FromRequest<B> for AuthUser
where
    B: Send,
{
    type Rejection = RetError;
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        // Bearer 认证, 从header中获取token
        // Extract the token from the authorization header
        // let TypedHeader(Authorization(bearer)) =
        //     TypedHeader::<Authorization<Bearer>>::from_request(req)
        //         .await
        //         .map_err(|_| RetError::GETTOKENERROR)?;
        // println!("token: {}", bearer.token());
        // cookie 认证, 痛cookie中获取token
        // 获取cookie对象
        let cookie = Option::<TypedHeader<Cookie>>::from_request(req)
            .await
            .unwrap();
        // 获取jwt_token
        let jwt_token = cookie.as_ref().and_then(|cookie| cookie.get("jwt_token"));
        let token = jwt_token.ok_or(RetError::TOKENERR("get token error".to_string()))?;
        // Decode the user data
        let token_data = decode::<AuthUser>(token, &KEYS.decoding, &Validation::default())
            .map_err(|_| RetError::TOKENERR("invalid token".to_string()))?;

        Ok(token_data.claims)
    }
}

#[derive(Debug)]
struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey<'static>,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret).into_static(),
        }
    }
}

pub fn jwt_encode(claims: AuthUser) -> Result<String, Error> {
    encode(&Header::default(), &claims, &KEYS.encoding)
}
