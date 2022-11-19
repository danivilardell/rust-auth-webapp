use crate::iam_queries::check_user_by_hash;
use rand::{distributions::Alphanumeric, Rng};
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
    pub id: String,
    pub joined: Vec<String>,
}

pub async fn insert_activity(activity: ActivityInfo, pool: &PgPool) -> eyre::Result<()> {
    let user = check_user_by_hash(activity.username.clone(), pool).await;
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(50)
        .map(char::from)
        .collect();
    println!("{}", s);

    match user {
        Some(user) => {
            match sqlx::query!(
                r#"INSERT INTO activities (activity_type, date, username, id, joined) VALUES ($1, $2, $3, $4, ARRAY[]::TEXT[])"#,
                activity.activity_type as ActivityType,
                activity.date,
                user.username,
                s
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
        r#"SELECT activity_type AS "activity_type: ActivityType", date, username, id, joined FROM activities"#
    )
    .fetch_all(pool)
    .await
    {
        Ok(activities) => Ok(activities),
        Err(_) => Err(eyre::eyre!("Can't get activities from database.")),
    }
}



#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
pub struct JoinActivity {
    pub id: String,
    pub user: String,
    pub key: String,
}

pub async fn join_activity_query(join_activity: JoinActivity, pool: &PgPool) -> eyre::Result<()> {
    let user = check_user_by_hash(join_activity.key.clone(), pool).await;

    match user {
        Some(_) => {
            match sqlx::query!(
                r#"UPDATE activities SET joined = array_append(joined, $1::text) WHERE id = $2"#,
                join_activity.user,
                join_activity.id
            )
                .execute(pool)
                .await
            {
                Ok(_) => Ok(()),
                Err(e) => {
                    println!("{:?}", e);
                    Err(eyre::eyre!("Can't add username to join list"))
                },
            }
        }
        None => Err(eyre::eyre!("You have to log in first!")),
    }
}
