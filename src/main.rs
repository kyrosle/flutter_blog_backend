use std::net::SocketAddr;

use axum::{
    http::{HeaderValue, Method},
    routing::get,
    Router,
};
use flutter_blog_backend::{fetch_article_cates, fetch_article_detail, fetch_article_list};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/http/list", get(fetch_article_list))
        .route("/http/get_news_categories", get(fetch_article_cates))
        .route("/http/detail", get(fetch_article_detail))
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
