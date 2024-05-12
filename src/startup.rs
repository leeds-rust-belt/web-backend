use std::net::TcpListener;

use actix_session::{ SessionMiddleware, storage::CookieSessionStore};
use actix_web::{App, HttpServer, middleware, web};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::cookie::Key;
use sqlx::MySqlPool;

use crate::routes::*;

pub fn run(listener: TcpListener, db_pool: MySqlPool) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::DefaultHeaders::new().add(("Access-Control-Allow-Origin", "*"))) // for testing purposes only
            .wrap(SessionMiddleware::new(CookieSessionStore::default(), Key::from(&[0; 64])))
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/discover", web::get().to(discover_index))
            .route("/search", web::post().to(discover_search))
            .route("/profile", web::get().to(profile_index))
            .route("/profile/{user_id}", web::get().to(get_profile))
            .route("/profile_update", web::post().to(profile_update))
            .route("/add_bio", web::post().to(bio_update))
            .route("/social", web::post().to(social_link))
            .route("/social/{user_id}", web::get().to(get_social))
            .route("/get_genres", web::get().to(get_genres))
            .route("/get_user_genres", web::get().to(get_user_genres))
            .route(
                "/get_genres_for_profile/{user_id}",
                web::get().to(get_genres_for_profile),
            )
            .route("/add_genre", web::post().to(add_genre))
            .route("/delete_genre", web::post().to(delete_genre))
            .route(
                "/get_shows_for_profile/{user_id}",
                web::get().to(get_shows_for_profile),
            )
            .route("/add_show", web::post().to(add_show))
            .route("/cancel_show/{show_id}", web::get().to(cancel_user_show))
            .route("/update_show", web::post().to(update_show))
            .route("/add_image", web::post().to(add_image_url))
            .route("/delete_image", web::get().to(delete_image_url))
            .route("/embed_url", web::post().to(add_embed_url))
            .route("/unembed_url", web::get().to(delete_embed_url))
            .route("/add_avatar", web::post().to(add_avatar))
            .route("/delete_avatar", web::get().to(delete_avatar))
            .route("/newmessage", web::post().to(new_message))
            .route("/blockcontact/{contact_id}", web::get().to(block_contact))
            .route("/blockcontacts", web::post().to(block_contacts))
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/reset_password", web::post().to(reset_password))
            .route(
                "/update_user_password",
                web::post().to(update_user_password),
            )
            .route("/delete_account", web::get().to(delete_account))
            .service(web::resource("/update_password").route(web::post().to(update_password)))
            .route("/get_contacts", web::get().to(get_contacts))
            .route("/add_contact/{user_id}", web::get().to(new_contact))
            .route("/delete_contact/{user_id}", web::get().to(delete_contact))
            .route("/delete_contacts", web::post().to(delete_contacts))
            .route("/unread_messages", web::get().to(unread_messages))
            .app_data(db_pool.clone())
            .service(
                web::scope("groups")
                    .route("/", web::post().to(create_group))
                    .route("/chat", web::post().to(create_chat))
                    .route("{id}", web::get().to(get_group))
                    .route("/", web::get().to(get_groups))
                    .route("{id}/leave", web::get().to(leave_group))
                    .route("{id}/join", web::get().to(join_group))
                    .route("{id}/messages", web::post().to(new_message))
                    .route("{id}/messages", web::get().to(get_messages))
                    .route("{id}/users", web::get().to(get_users)),
            )
            .service(
                web::scope("chat")
                    .route("/chat", web::post().to(create_chat))
                    .route("{id}", web::get().to(get_group))
                    .route("{id}/leave", web::get().to(leave_group))
                    .route("{id}/messages", web::post().to(new_message))
                    .route("{id}/messages", web::get().to(get_messages)),
            )
    })
        .listen(listener)?
        .run();
    Ok(server)
}
