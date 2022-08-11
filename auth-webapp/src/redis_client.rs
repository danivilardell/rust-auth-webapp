use fred::prelude::*;
use std::env;
use sha2::Sha256;
use sha2::Digest;

pub async fn store_username_password(
    username: String,
    password: String,
    client: &RedisClient,
) -> eyre::Result<()> {
    let hashed_password = get_hash(password);
    let exists: Option<String> = client.get(username.clone()).await?;

    match exists {
        Some(_) => Err(eyre::eyre!("Username already exists")),
        None => {
            client
                .set(username.clone(), hashed_password, None, None, false)
                .await?;
            Ok(())
        }
    }
}

pub async fn check_username_password(
    username: String,
    password: String,
    client: &RedisClient,
) -> eyre::Result<()> {
    let hashed_password = get_hash(password);
    let password_saved: Option<String> = client.get(username.clone()).await?;

    match password_saved {
        Some(pass) => {
            if hashed_password == pass { Ok(()) }
            else { Err(eyre::eyre!("Wrong password")) }
        },
        None => Err(eyre::eyre!("Username doesn't exists")),
    }
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

fn get_hash(password: String) -> String {
    let mut hasher = Sha256::new();
    let salt = env::var("PASSWORD_SALT").unwrap();
    let salted_password = format!("{}{}", salt, password);
    hasher.update(salted_password.into_bytes());
    let hashed_password = hasher.finalize();
    base64::encode(hashed_password)
}
