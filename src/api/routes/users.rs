use actix_web::{get, HttpResponse, Responder};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct User {
    id: u64,
    name: String,
    username: String,
    email: String,
    address: Address,
    phone: String,
    website: String,
    company: Company,
}

#[derive(Debug, Deserialize)]
struct Address {
    street: String,
    suite: String,
    city: String,
    zipcode: String,
    geo: Geo,
}

#[derive(Debug, Deserialize)]
struct Geo {
    lat: String,
    lng: String,
}

#[derive(Debug, Deserialize)]
struct Company {
    name: String,
    catchPhrase: String,
    bs: String,
}

#[get("/users")]
async fn get_users() -> impl Responder {
    let response = reqwest::get("https://jsonplaceholder.typicode.com/users")
        .await
        .unwrap()
        .json::<Vec<User>>()
        .await
        .unwrap();
    let mut data = HashMap::new();
    data.insert("users", response);
    HttpResponse::Ok().json(data)
}