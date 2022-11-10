use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
}

pub async fn insert_user(user: User, pool: &PgPool) -> eyre::Result<()> {
    match sqlx::query!(
        r#"INSERT INTO users (username, password) VALUES ($1, $2)"#,
        user.username,
        user.password
    )
    .execute(pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(_) => Err(eyre::eyre!("Can't insert value to database")),
    }
}

pub async fn check_user(user: User, pool: &PgPool) -> Option<User> {
    match sqlx::query_as!(
        User,
        r#"SELECT * FROM users WHERE username=$1"#,
        user.username
    )
    .fetch_one(pool)
    .await
    {
        Ok(u) => Some(u),
        Err(_) => None,
    }
}

pub async fn check_user_by_hash(password_hash: String, pool: &PgPool) -> Option<User> {
    match sqlx::query_as!(
        User,
        r#"SELECT * FROM users WHERE password=$1"#,
        password_hash
    )
    .fetch_one(pool)
    .await
    {
        Ok(u) => Some(u),
        Err(_) => None,
    }
}
