use sqlx::PgPool;

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub password: String,
}

pub async fn insert_user(user: User, pool: &PgPool) -> eyre::Result<()> {
    match sqlx::query!(
        r#"INSERT INTO users (username, password) VALUES ('test_username2', 'test_password2')"#
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(_) => Err(eyre::eyre!("Can't insert value to db")),
    }
}
