use crate::iam_logic::{check_username_password, store_username_password};
use fred::prelude::RedisClient;
use rocket::form::Form;
use rocket::http::{ContentType, Status};
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use sqlx::PgPool;

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdKey {
    id: String,
    username: String,
}

#[post("/sign_in", data = "<form>")]
pub async fn sign_in(
    form: Form<LoginInfo>,
    _redis_client: &State<RedisClient>,
    pool: &State<PgPool>,
) -> (Status, (ContentType, String)) {
    let username: String = form.username.clone();
    let password: String = form.password.clone();

    match check_username_password(username.clone(), password, pool.inner()).await {
        Ok(s) => (
            Status::Ok,
            (
                ContentType::JSON,
                serde_json::to_string(&IdKey { id: s, username }).unwrap(),
            ),
        ),
        Err(_) => (
            Status::Conflict,
            (
                ContentType::JSON,
                serde_json::to_string(&IdKey {
                    id: String::from(""),
                    username: String::from(""),
                })
                .unwrap(),
            ),
        ),
    }
}

#[post("/sign_up", data = "<form>")]
pub async fn sign_up(
    form: Form<LoginInfo>,
    _redis_client: &State<RedisClient>,
    pool: &State<PgPool>,
) -> Status {
    let username: String = form.username.clone();
    let password: String = form.password.clone();
    match store_username_password(username, password, pool.inner()).await {
        Ok(_) => Status::Ok,
        Err(_) => Status::Conflict,
    }
}
