use rocket::form::Form;
use rocket::http::Status;
use rocket::post;
use rocket::serde::{Deserialize, Serialize};
use rocket::{FromForm, FromFormField};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, FromFormField)]
pub enum SportType {
    Swim,
    Run,
    Bike,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, FromFormField)]
pub enum Intensity {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
pub struct ActivityInfo {
    pub sport: SportType,
    pub date: String,
    pub intensity: Intensity,
}

#[post("/create_activity", data = "<form>")]
pub async fn create_activity(form: Form<ActivityInfo>) -> Status {

    Status::Ok
}
