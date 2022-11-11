#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use fred::prelude::*;
use rocket::fs::{relative, FileServer};
use webapp_activities::activities_service::{create_activity, get_activities};
use webapp_db::db_init::connect_db;
use webapp_iam::sign_in_sign_up_service::{sign_in, sign_up};

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let redis_client = init_redis().await.unwrap();
    let pool = connect_db().await.unwrap();

    rocket::build()
        .mount(
            "/",
            routes![sign_in, sign_up, create_activity, get_activities],
        )
        .mount(
            "/",
            FileServer::from(relative!("/../webapp_frontend/static")),
        )
        .manage(redis_client.clone())
        .manage(pool)
}

pub async fn init_redis() -> eyre::Result<RedisClient> {
    let config = RedisConfig::default();
    let policy = ReconnectPolicy::default();
    let client = RedisClient::new(config);

    client.connect(Some(policy));
    client.wait_for_connect().await?;
    client.flushall(false).await?;

    Ok(client)
}
