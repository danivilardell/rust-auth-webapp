#[macro_use]
extern crate rocket;

use fred::prelude::*;
use rocket::fs::{relative, FileServer};
use webapp_iam::sign_in_sign_up_service::{sign_up, sign_in};
use tokio::time::Duration;
use dotenv::dotenv;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let redis_client = init_redis().await.unwrap();
    let pool = connect_db().await.unwrap();

    test_query(pool).await;

    rocket::build()
        .mount("/", routes![sign_in, sign_up])
        .mount("/", FileServer::from(relative!("/../webapp_frontend/static")))
        .manage(redis_client.clone())
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

pub async fn connect_db() -> eyre::Result<PgPool> {
    // Read connect options from environment variables, directly using the environment variables
    // specified in sqlx_core::postgres::options::PgConnectOptions
    // (PGHOST, PGPORT, PGUSER, PGPASSWORD, PGDATABASE, PGSSLROOTCERT, PGSSLMODE, PGAPPNAME)
    let connect_options = PgConnectOptions::new();
    println!("{connect_options:?}");

    let pool_options = PgPoolOptions::new()
        .test_before_acquire(true)
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(10 * 60))
        .max_lifetime(Duration::from_secs(30 * 60));

    Ok(pool_options.connect_with(connect_options).await?)
}

pub async fn test_query(pool: PgPool) {
    let res = sqlx::query_as!(
        User,
        r#"SELECT * FROM users"#,
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    println!("USERS: {res:?}");
}

#[derive(Debug)]
pub struct User {
    username: String,
    password: String,
}

// CREATE DATABASE webapp
//\c webapp
// CREATE TABLE IF NOT EXISTS users (
// username text not null,
// password text not null);
// CREATE ROLE webapp_test WITH LOGIN PASSWORD 'webapp-test-password'
// GRANT CONNECT ON DATABASE webapp TO webapp_test;
// GRANT ALL PRIVILEGES ON TABLE users TO webapp_test;
// insert into users (username, password) values ('test_user', 'test_password');