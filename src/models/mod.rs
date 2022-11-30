pub(crate) type FieldString = Option<String>;
pub(crate) type Id = Option<usize>;
pub(crate) type Key = Option<usize>;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Article {
    pub id: Id,
    pub title: FieldString,
    pub category: Key,
    pub content: FieldString,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArticleDto {
    pub id: usize,
    pub title: String,
    pub category: CategoryDto,
    pub content: String,
}

impl ArticleDto {
    pub fn from_with_category(a: Article, c: CategoryDto) -> Self {
        Self {
            id: a.id.unwrap(),
            title: a.title.unwrap(),
            category: c,
            content: a.content.unwrap(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: Id,
    pub name: FieldString,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CategoryDto {
    pub id: usize,
    pub name: String,
}

impl From<Category> for CategoryDto {
    fn from(c: Category) -> Self {
        Self {
            id: c.id.unwrap(),
            name: c.name.unwrap(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ApiResponse<T>
where
    T: DeserializeOwned + Clone,
{
    code: u16,
    data: T,
    message: String,
}

impl<T> ApiResponse<T>
where
    T: DeserializeOwned + Clone,
{
    pub fn new(code: u16, data: T, message: String) -> Self {
        Self {
            code,
            data,
            message,
        }
    }
}
