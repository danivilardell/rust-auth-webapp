#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use fred::prelude::*;
use rocket::fs::{relative, FileServer};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};
use webapp_iam::sign_in_sign_up_service::{sign_in, sign_up};
use webapp_activities::activities_service::create_activity;
use webapp_db::db_init::connect_db;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let redis_client = init_redis().await.unwrap();
    let pool = connect_db().await.unwrap();

    rocket::build()
        .mount("/", routes![sign_in, sign_up, create_activity])
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

// CREATE DATABASE webapp
//\c webapp
/* CREATE TABLE IF NOT EXISTS users (
username text not null,
password text not null);*/
// CREATE ROLE webapp_test WITH LOGIN PASSWORD 'webapp-test-password';
// GRANT CONNECT ON DATABASE webapp TO webapp_test;
// GRANT ALL PRIVILEGES ON TABLE users TO webapp_test;
// insert into users (username, password) values ('test_user', 'test_password');
