use actix_web::web;
use serde::Serialize;

#[derive(Serialize)]
pub struct JsonError<T> {
    pub message: String,
    pub data: Option<T>,
}

pub trait DeserializeErrorHandler {
    fn handle_deserialize(cfg: web::JsonConfig) -> web::JsonConfig;
}
