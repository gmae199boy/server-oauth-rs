mod handler;

use axum::{routing::{get, post}, Router};
use std::net::SocketAddr;

use dotenv;

use tracing_subscriber::FmtSubscriber;
use tracing::Level;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::INFO)
    .finish();
    tracing::subscriber::set_global_default(subscriber)
    .expect("setting default subscriber failed");

    let app = Router::new()
    .route("/kakao/login", get(handler::kakao::login))
    .route("/kakao/redirect", get(handler::kakao::redirect))
    .route("/kakao/logout", post(handler::kakao::logout))
    .route("/kakao/unlink", post(handler::kakao::unlink_app))
    .route("/kakao/tokenInfo", post(handler::kakao::token_info));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
