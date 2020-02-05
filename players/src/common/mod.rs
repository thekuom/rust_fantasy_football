use serde::Serialize;

#[derive(Serialize)]
pub struct JsonError<T> {
    pub message: String,
    pub data: Option<T>,
}
