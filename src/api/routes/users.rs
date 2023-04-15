use actix_web::{get, HttpResponse, Responder};
use serde::{Deserialize, Serialize };
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
struct Address {
    street: String,
    suite: String,
    city: String,
    zipcode: String,
    geo: Geo,
}

#[derive(Debug, Deserialize, Serialize)]
struct Geo {
    lat: String,
    lng: String,
}

#[derive(Debug, Deserialize, Serialize)]
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
    // It's not happy with json coming from actix-web
    // had to add { Serialize } to every struct
    HttpResponse::Ok().json(data)
}