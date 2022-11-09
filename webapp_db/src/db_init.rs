use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};
use tokio::time::Duration;

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
