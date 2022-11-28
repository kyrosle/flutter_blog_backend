pub(crate) type FieldString = Option<String>;
pub(crate) type Id = Option<usize>;
use axum::http::StatusCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Article {
    pub id: Id,
    pub title: FieldString,
    pub category: FieldString,
    pub content: FieldString,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Category {
    id: Id,
    name: FieldString,
}

#[derive(Clone, Debug, Serialize)]
pub struct ApiResponse<T>
where
    T: DeserializeOwned + Clone,
{
    code: u16,
    data: Vec<T>,
    message: String,
}

impl<T> ApiResponse<T>
where
    T: DeserializeOwned + Clone,
{
    pub fn new(code: u16, data: Vec<T>, message: String) -> Self {
        Self {
            code,
            data,
            message,
        }
    }
}
