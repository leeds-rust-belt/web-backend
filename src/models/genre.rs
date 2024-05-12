use actix_web::web;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlQueryResult;
use sqlx::MySqlPool;

#[derive(Deserialize, Serialize, Debug)]
pub struct Genre {
    genre: String,
    genre_id: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserGenre {
    pub user_id: String,
    pub genre_id: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserGenreList {
    genre: String,
    genre_id: i32,
}

pub async fn get_genre_list(pool: &web::Data<MySqlPool>) -> Result<Vec<Genre>, sqlx::Error> {
    sqlx::query_as!(
        Genre,
        r#"
            SELECT id as genre_id, genre
            FROM genres
            ORDER BY genre
        "#
    )
        .fetch_all(pool.get_ref())
        .await
}

pub async fn add_genre_to_user(
    userid: &str,
    genre: i32,
    pool: &web::Data<MySqlPool>,
) -> Result<MySqlQueryResult, sqlx::Error> {
    let insert = sqlx::query!(
        r#"
        INSERT INTO user_genres (genre_id, user_id)
        VALUES(?, ?)
        "#,
        genre,
        userid,
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

pub async fn delete_genre_from_user(
    userid: &str,
    genre: i32,
    pool: &web::Data<MySqlPool>,
) -> Result<MySqlQueryResult, sqlx::Error> {
    let delete = sqlx::query!(
        r#"
        DELETE FROM user_genres 
        WHERE genre_id = ? AND user_id = ?
        "#,
        genre,
        userid,
    )
        .execute(pool.get_ref())
        .await;
    match delete {
        Ok(record) => Ok(record),
        Err(e) => {
            log::error!("Failed to execute query: {:?}", e);
            Err(e)
        }
    }
}

pub async fn get_user_genre_list(
    userid: &str,
    pool: &web::Data<MySqlPool>,
) -> Result<Vec<UserGenreList>, sqlx::Error> {
    sqlx::query_as!(
        UserGenreList,
        r#"
            SELECT genre, genre_id
            FROM genres, user_genres
            WHERE user_id = ?
            AND genres.id = user_genres.genre_id
            ORDER BY genre
        "#,
        userid,
    )
        .fetch_all(pool.get_ref())
        .await
}
