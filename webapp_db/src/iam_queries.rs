#[derive(Debug)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[tokio::test]
async fn test_insert_user() {
    dotenv().ok();
    let pool = connect_db().await.unwrap();
    sqlx::query!(
        r#"INSERT INTO users (username, password) VALUES ('test_username2', 'test_password2')"#
    )
    .execute(&pool)
    .await
    .unwrap();
    let res = sqlx::query_as!(User, r#"SELECT * FROM users"#,)
        .fetch_all(&pool)
        .await
        .unwrap();
    println!("USERS1: {res:?}");
    sqlx::query!(r#"DELETE FROM users WHERE username = 'test_username2'"#)
        .execute(&pool)
        .await
        .unwrap();
    let res2 = sqlx::query_as!(User, r#"SELECT * FROM users"#,)
        .fetch_all(&pool)
        .await
        .unwrap();
    println!("USERS2: {:?}", res2);
}
