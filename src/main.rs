mod handler;

use axum::{routing::get, Router};
use std::net::SocketAddr;

use dotenv;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app = Router::new()
    .route("/kakao/login", get(handler::kakao::login))
    .route("/kakao/redirect", get(handler::kakao::kakao_redirect));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
