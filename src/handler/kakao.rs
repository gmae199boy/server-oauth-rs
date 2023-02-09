use axum::{
    response::{IntoResponse, Redirect}, 
    Json, 
    http::{
        StatusCode,
        header::{AUTHORIZATION, HeaderMap}
    }, 
    extract::Query
};
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

pub async fn redirect(query: Option<Query<Querys>>) -> impl IntoResponse {
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

pub async fn logout(headers: HeaderMap) -> impl IntoResponse {
    let client = reqwest::Client::new();
    let res: KakaoOauth = client.post("https://kapi.kakao.com/v1/user/logout")
        .header("Content-type", "application/x-www-form-urlencoded")
        .header("Authorization", headers.get(AUTHORIZATION).expect("not exist Bearer Token"))
        .send()
        .await.unwrap().json().await.unwrap();

    (StatusCode::OK, Json(res))
}

pub async fn unlink_app(headers: HeaderMap) -> impl IntoResponse {
    let client = reqwest::Client::new();
    let res: KakaoOauth = client.post("https://kapi.kakao.com/v1/user/unlink")
        .header("Content-type", "application/x-www-form-urlencoded")
        .header("Authorization", headers.get(AUTHORIZATION).expect("not exist Bearer Token"))
        .send()
        .await.unwrap().json().await.unwrap();

    (StatusCode::OK, Json(res))
}

pub async fn token_info(headers: HeaderMap) -> impl IntoResponse {
    let client = reqwest::Client::new();
    let res: KakaoOauth = client.get("https://kapi.kakao.com/v1/user/access_token_info")
        .header("Authorization", headers.get(AUTHORIZATION).expect("not exist Bearer Token"))
        .send()
        .await.unwrap().json().await.unwrap();

    (StatusCode::OK, Json(res))
}