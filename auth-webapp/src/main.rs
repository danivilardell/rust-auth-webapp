pub mod redis_client;

#[macro_use]
extern crate rocket;
use crate::redis_client::init_redis;
use fred::prelude::RedisClient;
use rocket::form::Form;
use rocket::http::Status;
use rocket::State;

use rocket::fs::{relative, FileServer};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[post("/sign_in", data = "<form>")]
async fn sign_in(form: Form<LoginInfo>, state: &State<RedisClient>) -> Status {
    let username: String = form.username.clone();
    let password: String = form.password.clone();
    match redis_client::check_username_password(username, password, &state.inner()).await {
        Ok(_) => Status::Ok,
        Err(_) => Status::Conflict
    }
}

#[post("/sign_up", data = "<form>")]
async fn sign_up(form: Form<LoginInfo>, state: &State<RedisClient>) -> Status {
    let username: String = form.username.clone();
    let password: String = form.password.clone();
    match redis_client::store_username_password(username, password, &state.inner()).await {
        Ok(_) => Status::Ok,
        Err(_) => Status::Conflict
    }
}

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().expect("Failed to read .env file");
    let redis_client = init_redis().await.unwrap();
    rocket::build()
        .mount("/", routes![sign_in, sign_up])
        .mount("/", FileServer::from(relative!("/static")))
        .manage(redis_client.clone())
}
