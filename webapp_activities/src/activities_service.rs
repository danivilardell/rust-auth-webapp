use rocket::form::Form;
use rocket::http::Status;
use rocket::{post, State};
use sqlx::PgPool;
use webapp_db::activities_queries::{insert_activity, ActivityInfo};

#[post("/create_activity", data = "<form>")]
pub async fn create_activity(form: Form<ActivityInfo>, pool: &State<PgPool>) -> Status {
    println!("{:?}", form);
    let response = insert_activity(form.into_inner(), pool).await;
    match response {
        Ok(_) => Status::Ok,
        Err(e) => Status::Conflict,
    }
}
