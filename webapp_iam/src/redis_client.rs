use fred::prelude::*;
use sha2::Digest;
use sha2::Sha256;
use std::env;

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
            if hashed_password == pass {
                Ok(())
            } else {
                Err(eyre::eyre!("Wrong password"))
            }
        }
        None => Err(eyre::eyre!("Username doesn't exists")),
    }
}

fn get_hash(password: String) -> String {
    let mut hasher = Sha256::new();
    let salt = env::var("PASSWORD_SALT").unwrap();
    let salted_password = format!("{}{}", salt, password);
    hasher.update(salted_password.into_bytes());
    let hashed_password = hasher.finalize();
    base64::encode(hashed_password)
}
