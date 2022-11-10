use crate::iam_queries::check_user_by_hash;
use rocket::serde::{Deserialize, Serialize};
use rocket::{FromForm, FromFormField};
use sqlx::PgPool;
use std::fmt;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, FromFormField)]
pub enum Type {
    Swim,
    Run,
    Bike,
    Movie,
    Convention,
    Concert,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
pub struct ActivityInfo {
    pub activity_type: Type,
    pub date: String,
    pub user: String,
}

pub async fn insert_activity(activity: ActivityInfo, pool: &PgPool) -> eyre::Result<()> {
    let user = check_user_by_hash(activity.user.clone(), pool).await;

    match user {
        Some(user) => {
            match sqlx::query!(
                r#"INSERT INTO activities (activity_type, date, username) VALUES ($1, $2, $3)"#,
                activity.activity_type.to_string(),
                activity.date,
                activity.user
            )
            .execute(pool)
            .await
            {
                Ok(_) => Ok(()),
                Err(_) => Err(eyre::eyre!("Can't insert value to database")),
            }
        }
        None => Err(eyre::eyre!("Can't insert value to database")),
    }
}
