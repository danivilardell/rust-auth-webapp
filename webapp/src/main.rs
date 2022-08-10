#[macro_use]
extern crate rocket;

use fred::prelude::*;
use rocket::fs::{relative, FileServer};
use webapp_iam::sign_in_sign_up_service::{sign_up, sign_in};

#[launch]
async fn rocket() -> _ {
    let redis_client = init_redis().await.unwrap();
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
