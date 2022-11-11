use sha2::Digest;
use sha2::Sha256;
use sqlx::PgPool;
use std::env;
use webapp_db::iam_queries;

pub async fn store_username_password(
    username: String,
    password: String,
    pool: &PgPool,
) -> eyre::Result<()> {
    let hashed_password = get_hash(password.clone());
    let user = iam_queries::User {
        username: username.clone(),
        password: hashed_password,
    };

    let exists: Option<iam_queries::User> = iam_queries::check_user(user.clone(), pool).await;
    match exists {
        Some(_) => Err(eyre::eyre!("Username already exists")),
        None => {
            iam_queries::insert_user(user, pool).await.unwrap();

            Ok(())
        }
    }
}

pub async fn check_username_password(
    username: String,
    password: String,
    pool: &PgPool,
) -> eyre::Result<String> {
    let hashed_password = get_hash(password);
    let user = iam_queries::User {
        username: username.clone(),
        password: hashed_password.clone(),
    };
    let saved_user: Option<iam_queries::User> = iam_queries::check_user(user, pool).await;

    match saved_user {
        Some(saved_user) => {
            if hashed_password == saved_user.password {
                Ok(hashed_password)
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
