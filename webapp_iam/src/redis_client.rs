use fred::prelude::*;

pub async fn store_username_password(
    username: String,
    password: String,
    client: &RedisClient,
) -> eyre::Result<()> {
    let exists: Option<String> = client.get(username.clone()).await?;
    println!("{exists:?}");
    match exists {
        Some(_) => Err(eyre::eyre!("Username already exists")),
        None => {
            client
                .set(username.clone(), password, None, None, false)
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
    let password_saved: Option<String> = client.get(username.clone()).await?;

    match password_saved {
        Some(pass) => {
            if password == pass {
                Ok(())
            } else {
                Err(eyre::eyre!("Wrong password"))
            }
        }
        None => Err(eyre::eyre!("Username doesn't exists")),
    }
}

