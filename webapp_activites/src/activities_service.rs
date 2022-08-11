use crate::redis_client::{store_username_password, check_username_password};
use fred::prelude::RedisClient;
use rocket::form::Form;
use rocket::http::Status;
use rocket::State;
use rocket::serde::{Deserialize, Serialize};

pub enum SportType {
    Swim,
    Run,
    Bike,
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
pub struct ActivityInfo {
    pub sport: SportType,
    pub date: String,
    pub intensity: i64,
}

#[post("/create_activity", data = "<form>")]
pub async fn sign_in(form: Form<ActivityInfo>) -> Status {

}
