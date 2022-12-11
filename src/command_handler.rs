use std::{env, fs};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header, Algorithm};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use warp::http::HeaderMap;
use base64;
use cli_table::{format::Justify, print_stdout, Table, WithTitle};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    iss: usize,
    exp: usize,
    sub: String
}

#[derive(Debug, Deserialize, Serialize)]
struct DeleteClaims {
    iss: usize,
    exp: usize,
    sub: String,
    delete_id: u16
}

const BEARER: &str = "Bearer ";
const TARGET_URL: &str = "http://localhost:1024/";

fn create_jwt() -> String {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(20))
        .expect("valid timestamp")
        .timestamp();
    let request_id = Uuid::new_v4();
    println!("Request ID: {:?}", request_id);

    let claims = Claims {
        exp: expiration as usize,
        iss: Utc::now().timestamp() as usize,
        sub: request_id.hyphenated().to_string()
    };
    let jwt_secret_string = env::var("JWT_SECRET").unwrap();
    let jwt_secret = jwt_secret_string.as_bytes();
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(jwt_secret))
        .map_err(|_| "Failed to encode jwt").unwrap()
}

fn create_delete_jwt(post_id: u16) -> String {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(20))
        .expect("valid timestamp")
        .timestamp();
    let request_id = Uuid::new_v4();
    println!("Request ID: {:?}", request_id);

    let claims = DeleteClaims {
        exp: expiration as usize,
        iss: Utc::now().timestamp() as usize,
        sub: request_id.hyphenated().to_string(),
        delete_id: post_id
    };
    let jwt_secret_string = env::var("JWT_SECRET").unwrap();
    let jwt_secret = jwt_secret_string.as_bytes();
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(jwt_secret))
        .map_err(|_| "Failed to encode jwt").unwrap()
}

#[derive(Debug, Deserialize, Serialize)]
struct AddJson {
    title: String,
    post_body: String,
    ranking: String,
    summary: String,
}

fn create_auth_header(token: String) -> HeaderMap {
    let mut request_headers = HeaderMap::new();
    let value = BEARER.to_owned() + token.as_str();
    request_headers.insert("Authorization", value.parse().unwrap());
    request_headers
}

pub async fn add_post(title: String, file_path: String, rank: String, summary: String) {
    let post_body = fs::read_to_string(file_path).unwrap();
    let encoded_body = base64::encode(post_body);
    let payload = AddJson {
        title,
        post_body: encoded_body,
        ranking: rank,
        summary,
    };
    let http_client = Client::new();
    let token = create_jwt();
    println!("{}", token);
    let call_headers = create_auth_header(token.clone());
    let post_url = TARGET_URL.to_owned() + "add";
    let _response = http_client
        .post(post_url)
        .headers(call_headers)
        .json(&payload)
        .send()
        .await
        .unwrap();

    println!("Success");
}

#[derive(Debug, PartialEq, Eq, Serialize, Clone, Deserialize, Table)]
struct PostRow {
    #[table(title = "ID", justify = "Justify::Right")]
    post_id: u16,

    #[table(title = "Create Date")]
    create_date: String,

    #[table(title = "Title")]
    title: String,

    #[table(title = "Summary")]
    summary: String,

    #[table(title = "Ranking")]
    ranking: String,
}

pub async fn get_post_list() {
    let http_client = Client::new();
    let post_url = TARGET_URL.to_owned() + "posts";
    let mut response = http_client
        .get(post_url)
        .send()
        .await
        .unwrap()
        .json::<Vec<PostRow>>()
        .await
        .unwrap();
    // println!("{:#?}", response);
    response.reverse();
    print_stdout(response.with_title()).unwrap();
}


#[derive(Debug, PartialEq, Eq, Serialize, Clone, Deserialize)]
struct PostBody {
    post_id: u16,
    create_date: String,
    title: String,
    summary: String,
    ranking: String,
    bodytext: String
}

pub async fn get_single_post(post_id: u16, file_path: String) {
    let http_client = Client::new();
    let post_url = format!("{}{}{}", TARGET_URL.to_owned(), "posts/", post_id);
    // let post_url = TARGET_URL.to_owned() + "posts/" ;
    let response = http_client
        .get(post_url)
        .send()
        .await
        .unwrap()
        .json::<PostBody>()
        .await
        .unwrap();
    // println!("{:#?}", response);
    fs::write(file_path, response.bodytext).unwrap();
    println!("Success");
}

pub async fn delete_post(post_id: u16) {
    let http_client = Client::new();
    let token = create_delete_jwt(post_id);
    println!("{}", token);
    let call_headers = create_auth_header(token.clone());
    let delete_url = format!("{}{}{}", TARGET_URL.to_owned(), "delete/", post_id);
    let _response = http_client
        .delete(delete_url.clone())
        .headers(call_headers)
        .send()
        .await
        .unwrap();

    println!("{}", delete_url);
    println!("Success");
}
