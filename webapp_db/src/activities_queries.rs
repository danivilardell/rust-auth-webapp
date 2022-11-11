use crate::iam_queries::check_user_by_hash;
use rocket::serde::{Deserialize, Serialize};
use rocket::{FromForm, FromFormField};
use sqlx::PgPool;
use std::fmt;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, FromFormField, sqlx::Type)]
#[sqlx(type_name = "valid_activity", rename_all = "lowercase")]
pub enum ActivityType {
    Swim,
    Run,
    Bike, //Movie,
          //Convention,
          //Concert,
}

impl fmt::Display for ActivityType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
pub struct ActivityInfo {
    pub activity_type: ActivityType,
    pub date: String,
    pub username: String,
}

pub async fn insert_activity(activity: ActivityInfo, pool: &PgPool) -> eyre::Result<()> {
    let user = check_user_by_hash(activity.username.clone(), pool).await;

    match user {
        Some(user) => {
            match sqlx::query!(
                r#"INSERT INTO activities (activity_type, date, username) VALUES ($1, $2, $3)"#,
                activity.activity_type as ActivityType,
                activity.date,
                user.username
            )
            .execute(pool)
            .await
            {
                Ok(_) => Ok(()),
                Err(e) => {
                    println!("{:?}", e);
                    Err(eyre::eyre!("Can't insert value to database"))
                },
            }
        }
        None => Err(eyre::eyre!("You have to log in first!")),
    }
}

pub async fn get_activities_query(pool: &PgPool) -> eyre::Result<Vec<ActivityInfo>> {
    match sqlx::query_as!(
        ActivityInfo,
        r#"SELECT activity_type AS "activity_type: ActivityType", date, username FROM activities"#
    )
    .fetch_all(pool)
    .await
    {
        Ok(activities) => Ok(activities),
        Err(_) => Err(eyre::eyre!("Can't get activities from database.")),
    }
}
