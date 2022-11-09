use tokio::time::Duration;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};

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
    let res = sqlx::query_as!(User, r#"SELECT * FROM users"#,)
        .fetch_all(&pool)
        .await
        .unwrap();

    println!("USERS: {res:?}");
}

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub password: String,
}
