use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
pub struct ResponseInfo<T: Serialize> {
    pub errno: String,
    pub data: T,
}

#[derive(Debug)]
pub enum RetError {
    PARAMERR(String),
    DATAERR(String),
    TOKENERR(String),
    DBERR(String),
}

impl IntoResponse for RetError {
    fn into_response(self) -> Response {
        let (errno, errmsg) = match self {
            RetError::PARAMERR(msg) => ("4001", msg),
            RetError::DATAERR(msg) => ("4002", msg),
            RetError::TOKENERR(msg) => ("4003", msg),
            RetError::DBERR(msg) => ("4004", msg),
        };
        let body = Json(json!({
            "errno": errno,
            "errmsg": errmsg,
        }));
        (StatusCode::OK, body).into_response()
    }
}
