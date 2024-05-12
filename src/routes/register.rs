use actix_session::Session;
use actix_web::{HttpResponse, web};
use actix_web::http::header::ContentType;
use bcrypt::*;
use sqlx::MySqlPool;

use crate::models::users::*;

pub async fn register(
    session: Session,
    form: web::Json<UserData>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    // check if user already in usage
    match get_login_data(&form.email, &pool).await {
        Ok(_) => {
            let response = format!("{{\"error\" : \"{}\"}}", "User already exists!");
            HttpResponse::Ok()
                .insert_header(ContentType::json())
                .body(response)
        }
        Err(_) => {
            let password_hash = match hash(&form.password, bcrypt::DEFAULT_COST) {
                Ok(hashed_password) => hashed_password,
                Err(_e) => {
                    log::error!("Failed to encrypt password");
                    "".to_string()
                }
            };
            let insert = register_new_user(&form, &pool, &password_hash).await;
            match insert {
                Ok(_) => {
                    // get the user to send back
                    match get_login_data(&form.email, &pool).await {
                        Ok(user_record) => {
                            let token = create_session_token(&user_record.id, &pool).await;
                            match token {
                                Ok(token) => {
                                    let _result = session.insert("tk", &token);
                                    let response = format!(
                                        "{{\"token\" : \"{}\", \"id\" : \"{}\"}}",
                                        token, user_record.id
                                    );
                                    HttpResponse::Ok()
                                        .insert_header(ContentType::json())
                                        .body(response)
                                }
                                Err(_) => HttpResponse::Ok().body("Unable to create session token"),
                            }
                        }
                        Err(_) => HttpResponse::Ok()
                            .json("User registered but unable to get from database."),
                    }
                }
                Err(e) => {
                    log::error!("Failed to execute query: {:?}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
}
