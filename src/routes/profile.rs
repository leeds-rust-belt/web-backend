use actix_session::Session;
use actix_web::{HttpResponse, web};
use bcrypt::*;
use sqlx::MySqlPool;

use crate::models::genre::*;
use crate::models::shows::*;
use crate::models::users::*;

pub async fn profile_index(session: Session, pool: web::Data<MySqlPool>) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => match get_profile_data(&user, &pool).await {
                    Ok(profile) => HttpResponse::Ok().json(profile),
                    Err(e) => {
                        log::error!("Whoops: {:?}", e);
                        HttpResponse::Ok().json("Error")
                    }
                },
                Err(e) => {
                    log::error!("Got user from check_session_token : {:?}", e);
                    HttpResponse::Ok().json("not logged_in but have a cookie?")
                }
            }
        }
        Ok(None) => {
            log::info!("Was not able to get tk from session cookie");
            HttpResponse::Ok().json("No Session")
        }
        Err(e) => {
            log::error!("Whoops: {:?}", e);
            HttpResponse::Ok().json("Error")
        }
    }
}

pub async fn get_profile(user_id: web::Path<String>, pool: web::Data<MySqlPool>) -> HttpResponse {
    match get_profile_data(&user_id, &pool).await {
        Ok(profile) => HttpResponse::Ok().json(profile),
        Err(e) => {
            log::error!("Whoops: {:?}", e);
            HttpResponse::Ok().json("Error")
        }
    }
}

pub async fn profile_update(
    session: Session,
    profile: web::Json<UpdateProfileData>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => {
                    let update = update_profile(&user, &profile, &pool).await;
                    match update {
                        Ok(_) => HttpResponse::Ok().json("Profile Updated"),
                        Err(e) => {
                            log::error!("Failed to execute query: {:?}", e);
                            HttpResponse::InternalServerError().finish()
                        }
                    }
                }
                Err(_) => HttpResponse::Ok().json("not logged_in"),
            }
        }
        Ok(None) => HttpResponse::Ok().json("No Session"),
        Err(_) => HttpResponse::Ok().json("Error"),
    }
}

pub async fn bio_update(
    session: Session,
    bio: web::Json<BioUpdate>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => {
                    let result = update_bio(&user, &bio.bio, &pool).await;
                    match result {
                        Ok(_) => HttpResponse::Ok().json("BIO Updated"),
                        Err(_) => HttpResponse::Ok().json("something bad happened"),
                    }
                }
                Err(_) => HttpResponse::Ok().json("not logged_in"),
            }
        }
        Ok(None) => HttpResponse::Ok().json("No Session"),
        Err(_) => HttpResponse::Ok().json("Error"),
    }
}

pub async fn get_genres(pool: web::Data<MySqlPool>) -> HttpResponse {
    let genres = get_genre_list(&pool).await;
    match genres {
        Ok(records) => HttpResponse::Ok().json(records),
        Err(_) => HttpResponse::Ok().json("No Genres found"),
    }
}

pub async fn add_genre(
    session: Session,
    form: web::Json<UserGenre>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => {
                    let insert = add_genre_to_user(&user, form.genre_id, &pool).await;
                    match insert {
                        Ok(_) => HttpResponse::Ok().json("Genre Added"),
                        Err(e) => {
                            log::error!("Failed to execute query: {:?}", e);
                            HttpResponse::InternalServerError().finish()
                        }
                    }
                }
                Err(_) => HttpResponse::Ok().json("not logged_in"),
            }
        }
        Ok(None) => HttpResponse::Ok().json("No Session"),
        Err(_) => HttpResponse::Ok().json("Error"),
    }
}

pub async fn delete_genre(
    session: Session,
    form: web::Json<UserGenre>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => {
                    let delete = delete_genre_from_user(&user, form.genre_id, &pool).await;
                    match delete {
                        Ok(_) => HttpResponse::Ok().json("Genre Removed"),
                        Err(e) => {
                            log::error!("Failed to execute query: {:?}", e);
                            HttpResponse::InternalServerError().finish()
                        }
                    }
                }
                Err(_) => HttpResponse::Ok().json("not logged_in"),
            }
        }
        Ok(None) => HttpResponse::Ok().json("No Session"),
        Err(_) => HttpResponse::Ok().json("Error"),
    }
}

pub async fn get_user_genres(session: Session, pool: web::Data<MySqlPool>) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => {
                    // Need to know the user id.
                    match get_user_genre_list(&user, &pool).await {
                        Ok(records) => HttpResponse::Ok().json(records),
                        Err(_) => HttpResponse::Ok().json("Unable to obtain genres"),
                    }
                }
                Err(_) => HttpResponse::Ok().json("not logged_in"),
            }
        }
        Ok(None) => HttpResponse::Ok().json("No Session"),
        Err(_) => HttpResponse::Ok().json("Error"),
    }
}

pub async fn get_social(user_id: web::Path<String>, pool: web::Data<MySqlPool>) -> HttpResponse {
    // Need to know the user id.
    match get_social_links(&user_id, &pool).await {
        Ok(records) => HttpResponse::Ok().json(records),
        Err(_) => HttpResponse::Ok().json("Unable to obtain shows"),
    }
}

pub async fn get_genres_for_profile(
    user_id: web::Path<String>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    // Need to know the user id.
    match get_user_genre_list(&user_id, &pool).await {
        Ok(records) => HttpResponse::Ok().json(records),
        Err(_) => HttpResponse::Ok().json("Unable to obtain genres"),
    }
}

pub async fn get_shows_for_profile(
    user_id: web::Path<String>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    // Need to know the user id.
    match get_user_shows(&user_id, &pool).await {
        Ok(records) => HttpResponse::Ok().json(records),
        Err(_) => HttpResponse::Ok().json("Unable to obtain shows"),
    }
}

pub async fn add_show(
    session: Session,
    add_show: web::Json<Show>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => match add_user_show(&user, add_show, &pool).await {
                    Ok(_) => HttpResponse::Ok().json("Show added"),
                    Err(_) => HttpResponse::Ok().json("Unable to add show"),
                },
                Err(_) => HttpResponse::Ok().json("not logged_in"),
            }
        }
        Ok(None) => HttpResponse::Ok().json("No Session"),
        Err(_) => HttpResponse::Ok().json("Error"),
    }
}

pub async fn cancel_user_show(
    session: Session,
    show_id: web::Path<String>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => match cancel_show(&show_id, &user, &pool).await {
                    Ok(_) => HttpResponse::Ok().json("Show cancelled"),
                    Err(_) => HttpResponse::Ok().json("Unable to cancel show"),
                },
                Err(_) => HttpResponse::Ok().json("not logged_in"),
            }
        }
        Ok(None) => HttpResponse::Ok().json("No Session"),
        Err(_) => HttpResponse::Ok().json("Error"),
    }
}

pub async fn update_show(
    session: Session,
    update_show: web::Json<Show>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => match update_user_show(&user, update_show, &pool).await {
                    Ok(_) => HttpResponse::Ok().json("Show updated"),
                    Err(_) => HttpResponse::Ok().json("Unable to updated show"),
                },
                Err(_) => HttpResponse::Ok().json("not logged_in"),
            }
        }
        Ok(None) => HttpResponse::Ok().json("No Session"),
        Err(_) => HttpResponse::Ok().json("Error"),
    }
}

pub async fn add_embed_url(
    session: Session,
    add_url: web::Json<AddUrl>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => match add_embed_url_to_user(&user, &add_url, &pool).await {
                    Ok(_) => HttpResponse::Ok().json("Url embedded"),
                    Err(_) => HttpResponse::Ok().json("Unable to embed url"),
                },
                Err(_) => HttpResponse::Ok().json("not logged_in"),
            }
        }
        Ok(None) => HttpResponse::Ok().json("No Session"),
        Err(_) => HttpResponse::Ok().json("Error"),
    }
}

pub async fn delete_embed_url(session: Session, pool: web::Data<MySqlPool>) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => match delete_embed_url_from_user(&user, &pool).await {
                    Ok(_) => HttpResponse::Ok().json("Url unembedded"),
                    Err(_) => HttpResponse::Ok().json("Unable to unembed url"),
                },
                Err(_) => HttpResponse::Ok().json("not logged_in"),
            }
        }
        Ok(None) => HttpResponse::Ok().json("No Session"),
        Err(_) => HttpResponse::Ok().json("Error"),
    }
}

pub async fn add_image_url(
    session: Session,
    add_url: web::Json<AddUrl>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => match add_image_url_to_user(&user, &add_url, &pool).await {
                    Ok(_) => HttpResponse::Ok().json("Image Url added"),
                    Err(_) => HttpResponse::Ok().json("Unable to add image url"),
                },
                Err(_) => HttpResponse::Ok().json("not logged_in"),
            }
        }
        Ok(None) => HttpResponse::Ok().json("No Session"),
        Err(_) => HttpResponse::Ok().json("Error"),
    }
}

pub async fn delete_image_url(session: Session, pool: web::Data<MySqlPool>) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => match delete_image_url_from_user(&user, &pool).await {
                    Ok(_) => HttpResponse::Ok().json("Image Url removed"),
                    Err(_) => HttpResponse::Ok().json("Unable to remove image url"),
                },
                Err(_) => HttpResponse::Ok().json("not logged_in"),
            }
        }
        Ok(None) => HttpResponse::Ok().json("No Session"),
        Err(_) => HttpResponse::Ok().json("Error"),
    }
}

pub async fn add_avatar(
    session: Session,
    add_url: web::Json<AddUrl>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => match add_avatar_url_to_user(&user, &add_url, &pool).await {
                    Ok(_) => HttpResponse::Ok().json("Url avatarded"),
                    Err(_) => HttpResponse::Ok().json("Unable to avatar url"),
                },
                Err(_) => HttpResponse::Ok().json("not logged_in"),
            }
        }
        Ok(None) => HttpResponse::Ok().json("No Session"),
        Err(_) => HttpResponse::Ok().json("Error"),
    }
}

pub async fn delete_avatar(session: Session, pool: web::Data<MySqlPool>) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => match delete_avatar_url_from_user(&user, &pool).await {
                    Ok(_) => HttpResponse::Ok().json("Url unavatarded"),
                    Err(_) => HttpResponse::Ok().json("Unable to unavatar url"),
                },
                Err(_) => HttpResponse::Ok().json("not logged_in"),
            }
        }
        Ok(None) => HttpResponse::Ok().json("No Session"),
        Err(_) => HttpResponse::Ok().json("Error"),
    }
}

pub async fn update_user_password(
    session: Session,
    new_password: web::Json<PasswordUpdate>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => match get_user(&user, &pool).await {
                    Ok(user_data) => {
                        match verify(&new_password.old_password, &user_data.password) {
                            Ok(_) => {
                                match change_password_for_user(&user, &new_password, &pool).await {
                                    Ok(_) => HttpResponse::Ok().json("Password updated"),
                                    Err(_) => HttpResponse::Ok().json("Unable to update password"),
                                }
                            }
                            Err(_) => HttpResponse::Ok().json("Old Password does not match"),
                        }
                    }
                    Err(_) => HttpResponse::Ok().json("Unable to select user record from token"),
                },
                Err(_) => HttpResponse::Ok().json("not logged_in"),
            }
        }
        Ok(None) => HttpResponse::Ok().json("No Session"),
        Err(_) => HttpResponse::Ok().json("Error"),
    }
}

pub async fn delete_account(session: Session, pool: web::Data<MySqlPool>) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => match delete_user_account(&user, &pool).await {
                    Ok(_) => HttpResponse::Ok().json("Account Deleted"),
                    Err(_) => HttpResponse::Ok().json("Unable to delete account"),
                },
                Err(_) => HttpResponse::Ok().json("not logged_in"),
            }
        }
        Ok(None) => HttpResponse::Ok().json("No Session"),
        Err(_) => HttpResponse::Ok().json("Error"),
    }
}

pub async fn social_link(
    session: Session,
    social_link: web::Json<SocialLink>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => match add_social_link_to_user(&user, social_link, &pool).await {
                    Ok(_) => HttpResponse::Ok().json("Social Link added"),
                    Err(_) => HttpResponse::Ok().json("Unable to social link url"),
                },
                Err(_) => HttpResponse::Ok().json("not logged_in"),
            }
        }
        Ok(None) => HttpResponse::Ok().json("No Session"),
        Err(_) => HttpResponse::Ok().json("Error"),
    }
}
