pub mod database;

use crate::{ApiResponse, Article, ArticleDto, Category, CategoryDto};
use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use rbs::to_value;
use std::collections::HashMap;

pub use database::*;

pub async fn fetch_article_list(Query(params): Query<HashMap<String, usize>>) -> impl IntoResponse {
    // get the `category_id` from the url path, default is `1`
    let category_id = if params.contains_key("category_id") {
        params["category_id"]
    } else {
        1
    };
    // record the category's id as key and the its category as value
    let mut c_map = HashMap::new();

    // fetch all Category from the database table `category`
    let c_vec = RB
        .fetch_decode::<Vec<Category>>(SQL["fetchArticleCates"], vec![])
        .await
        .unwrap();

    c_vec.iter().for_each(|c| {
        let id = c.id.unwrap();
        let value = CategoryDto::from(c.clone());
        c_map.insert(id, value);
    });

    let v = RB
        .fetch_decode::<Vec<Article>>(SQL["fetchArticleList"], vec![to_value!(category_id)])
        .await
        .map_err(internal_error)
        .map(|vec| {
            vec.into_iter()
                .map(|v| {
                    // c_map use empty entry key
                    ArticleDto::from_with_category(v.clone(), c_map[&v.category.unwrap()].clone())
                })
                .collect::<Vec<_>>()
        })
        .unwrap();
    Json(ApiResponse::new(
        StatusCode::OK.as_u16(),
        v,
        String::from("Ok"),
    ))
}

pub async fn fetch_article_cates() -> impl IntoResponse {
    let v = RB
        .fetch_decode::<Vec<Category>>(SQL["fetchArticleCates"], vec![])
        .await
        .map_err(internal_error)
        .map(|c_vec| c_vec.into_iter().map(CategoryDto::from).collect::<Vec<_>>())
        .unwrap();
    Json(ApiResponse::new(
        StatusCode::OK.as_u16(),
        v,
        String::from("Ok"),
    ))
}

pub async fn fetch_article_detail(
    Query(params): Query<HashMap<String, usize>>,
) -> impl IntoResponse {
    let article_id = if params.contains_key("article_id") {
        params["article_id"]
    } else {
        1
    };
    let article = RB
        .fetch_decode::<Article>(SQL["fetchArticleDetail"], vec![to_value!(article_id)])
        .await
        .map_err(internal_error)
        .unwrap();
    let category_id = article.category.unwrap();
    let category = RB
        .fetch_decode::<Category>(
            SQL["fetchArticleCteSingleById"],
            vec![to_value!(category_id)],
        )
        .await
        .map(|c| c.into())
        .unwrap();
    let v = ArticleDto::from_with_category(article, category);
    Json(ApiResponse::new(
        StatusCode::OK.as_u16(),
        v,
        String::from("Ok"),
    ))
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
