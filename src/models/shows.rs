use actix_web::web;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlQueryResult;
use sqlx::MySqlPool;

#[derive(Deserialize, Serialize, Debug)]
pub struct Show {
    id: Option<i32>,
    city: String,
    venue: String,
    day: i32,
    month: i32,
    year: i32,
    time: Option<String>,
    comments: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ShowDates {
    id: i32,
    user_id: Option<String>,
    status: Option<String>,
    venue: String,
    city: String,
    day: i32,
    month: i32,
    year: i32,
    time: Option<String>,
    comments: Option<String>,
}

pub async fn add_user_show(
    userid: &str,
    show: web::Json<Show>,
    pool: &web::Data<MySqlPool>,
) -> Result<MySqlQueryResult, sqlx::Error> {
    let insert = sqlx::query!(
        r#"
        INSERT INTO user_shows (day,month,year,city,venue,user_id, time, comments)
        VALUES(?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        show.day,
        show.month,
        show.year,
        show.city,
        show.venue,
        userid,
        show.time,
        show.comments
    )
        .execute(pool.get_ref())
        .await;
    match insert {
        Ok(record) => Ok(record),
        Err(e) => {
            log::error!("Failed to execute query: {:?}", e);
            Err(e)
        }
    }
}

pub async fn update_user_show(
    userid: &str,
    show: web::Json<Show>,
    pool: &web::Data<MySqlPool>,
) -> Result<MySqlQueryResult, sqlx::Error> {
    let update = sqlx::query!(
        r#"
        UPDATE user_shows set day=?,month=?,year=?,city=?,venue=?,user_id=?, time=?, comments=?
        WHERE id = ?
        "#,
        show.day,
        show.month,
        show.year,
        show.city,
        show.venue,
        userid,
        show.time,
        show.comments,
        show.id
    )
        .execute(pool.get_ref())
        .await;
    match update {
        Ok(record) => Ok(record),
        Err(e) => {
            log::error!("Failed to execute query: {:?}", e);
            Err(e)
        }
    }
}

pub async fn get_user_shows(
    userid: &str,
    pool: &web::Data<MySqlPool>,
) -> Result<Vec<ShowDates>, sqlx::Error> {
    sqlx::query_as!(
        ShowDates,
        r#"
        SELECT id, user_id, venue, city, day, month, year, status, time, comments
        FROM showdates
        WHERE
            showdate >= now()
            AND user_id = ?
        ORDER BY showdate
        "#,
        userid
    )
        .fetch_all(pool.get_ref())
        .await
}

pub async fn cancel_show(
    show_id: &str,
    user_id: &str,
    pool: &web::Data<MySqlPool>,
) -> Result<MySqlQueryResult, sqlx::Error> {
    let update = sqlx::query!(
        r#"
        UPDATE user_shows SET status = 'CANCELLED'
        WHERE id = ? AND user_id = ?
        "#,
        show_id,
        user_id
    )
        .execute(pool.get_ref())
        .await;
    match update {
        Ok(record) => Ok(record),
        Err(e) => {
            log::error!("Failed to execute query: {:?}", e);
            Err(e)
        }
    }
}
