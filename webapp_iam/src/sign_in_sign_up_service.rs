use crate::redis_client::{store_username_password, check_username_password};
use fred::prelude::RedisClient;
use rocket::form::Form;
use rocket::http::Status;
use rocket::State;
use rocket::serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[post("/sign_in", data = "<form>")]
pub async fn sign_in(form: Form<LoginInfo>, redis_client: &State<RedisClient>, pool: &State<PgPool>) -> Status {
    let username: String = form.username.clone();
    let password: String = form.password.clone();
    match check_username_password(username, password, &redis_client.inner()).await {
        Ok(_) => Status::Ok,
        Err(_) => Status::Conflict
    }
}

#[post("/sign_up", data = "<form>")]
pub async fn sign_up(form: Form<LoginInfo>, state: &State<RedisClient>) -> Status {
    let username: String = form.username.clone();
    let password: String = form.password.clone();
    match store_username_password(username, password, &state.inner()).await {
        Ok(_) => Status::Ok,
        Err(_) => Status::Conflict
    }
}
