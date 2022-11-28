use std::{collections::HashMap, fmt, net::SocketAddr};

use axum::{
    extract::Query,
    http::{HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use flutter_blog_backend::{ApiResponse, Article};
use once_cell::sync::Lazy;
use rbdc_pg::driver::PgDriver;
use rbs::to_value;
use tower_http::cors::CorsLayer;

static SQL: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("fetchArticleList", "select a.id, a.title, c.name as category, a.content  from article a left join category c on a.category = c.id where c.id = ?;");
    m
});

async fn fetch_article_list(Query(_params): Query<HashMap<String, usize>>) -> impl IntoResponse {
    // fast_log::init(fast_log::Config::new().console()).unwrap();
    let rb = rbatis::Rbatis::new();
    rb.init(PgDriver {}, "postgres://postgres:1234@localhost:55435/blog")
        .unwrap();
    let v = rb
        .fetch_decode::<Vec<Article>>(SQL["fetchArticleList"], vec![to_value!(1)])
        .await
        .map_err(internal_error)
        .unwrap();
    Json(ApiResponse::new(
        StatusCode::ACCEPTED.as_u16(),
        v,
        String::from("Ok"),
    ))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/http/list", get(fetch_article_list))
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET]),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 5590));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
