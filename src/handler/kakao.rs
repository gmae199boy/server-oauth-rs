use axum::{response::{IntoResponse, Redirect}, Json, http::StatusCode, extract::Query};
use serde_derive::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct KakaoOauth {
    pub access_token: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub refresh_token_expires_in: i32,
}

#[derive(Deserialize)]
pub struct Querys {
    pub code: String,
}

#[derive(Serialize, Deserialize)]
pub struct KakaoBody {
    pub grant_type: String,
    pub client_id: String,
    pub code: String,
    pub redirect_uri: String,
}

pub async fn login() -> impl IntoResponse {
    Redirect::permanent(
        &format!(
            "https://kauth.kakao.com/oauth/authorize?client_id={}&redirect_uri={}&response_type=code", 
            std::env::var("KAKAO_API_KEY").expect("no kakao api key"), 
            std::env::var("KAKAO_REDIRECT_URL").expect("no redirect url")
        )
    )
}

pub async fn kakao_redirect(query: Option<Query<Querys>>) -> impl IntoResponse {
    let Query(query) = query.unwrap();
    let client_id = std::env::var("KAKAO_API_KEY").expect("no kakao api key");

    let body = serde_urlencoded::to_string(&KakaoBody{
        grant_type: String::from("authorization_code"),
        client_id,
        code: query.code,
        redirect_uri: std::env::var("KAKAO_REDIRECT_URL").expect("no redirect url"),
    }).expect("serialize fail");

    let client = reqwest::Client::new();
    let res: KakaoOauth = client.post("https://kauth.kakao.com/oauth/token")
        .header("Content-type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await.unwrap().json().await.unwrap();

    (StatusCode::OK, Json(res))
}