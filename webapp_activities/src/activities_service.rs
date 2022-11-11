use rocket::form::Form;
use rocket::http::{ContentType, Status};
use rocket::{get, post, State};
use sqlx::PgPool;
use webapp_db::activities_queries::{get_activities_query, insert_activity, ActivityInfo};

#[post("/create_activity", data = "<form>")]
pub async fn create_activity(form: Form<ActivityInfo>, pool: &State<PgPool>) -> Status {
    println!("{:?}", form);
    let response = insert_activity(form.into_inner(), pool).await;
    match response {
        Ok(_) => Status::Ok,
        Err(_) => {
            println!("{:?}", response);
            Status::Conflict
        },
    }
}

#[get("/get_activities")]
pub async fn get_activities(pool: &State<PgPool>) -> (Status, (ContentType, String)) {
    let response = get_activities_query(pool).await;
    match response {
        Ok(activities) => (
            Status::Ok,
            (
                ContentType::JSON,
                serde_json::to_string(&activities).unwrap(),
            ),
        ),
        Err(_) => (Status::Ok, (ContentType::JSON, String::from("{}"))),
    }
}
