use actix_session::Session;
use actix_web::{HttpResponse, web};
use sqlx::MySqlPool;

use crate::models::*;


pub async fn new_message(
    session: Session,
    new_message: web::Json<NewMessage>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => {
                    let allowed = check_user_is_in_group(&user, &new_message, &pool);
                    if allowed.await {
                        let added = messaging::new_message(&user, &new_message, &pool).await;
                        match added {
                            Ok(_) => {
                                HttpResponse::Ok().json("Message added")
                            }
                            Err(e) => {
                                log::error!("Failed to execute query: {:?}", e);
                                HttpResponse::InternalServerError().finish()
                            }
                        }
                    } else {
                        HttpResponse::Ok().json("You cannot access this group")
                    }
                }
                Err(_) => HttpResponse::Ok().json("not logged_in"),
            }
        }
        Ok(None) => HttpResponse::Ok().json("No Session"),
        Err(_) => HttpResponse::Ok().json("Error"),
    }
}

pub async fn block_contact(
    contact_id: web::Path<String>,
    session: Session,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => match messaging::block_contact(&user, &contact_id, &pool).await {
                    Ok(_) => HttpResponse::Ok().json("Contact blocked"),
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

pub async fn block_contacts(
    session: Session,
    contacts: web::Json<ContactList>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => {
                    for c in &contacts.contacts {
                        match messaging::block_contact(&user, c, &pool).await {
                            Ok(_) => {
                                log::info!("Contact blocked id:{}", c);
                            }
                            Err(e) => {
                                log::error!("Whoops: {:?}", e);
                            }
                        }
                    }
                    HttpResponse::Ok().json("Contacts blocked")
                }
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

pub async fn new_contact(
    contact_id: web::Path<String>,
    session: Session,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => match add_contact(&user, &contact_id, &pool).await {
                    Ok(_) => HttpResponse::Ok().json("Contact added"),
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

pub async fn delete_contact(
    contact_id: web::Path<String>,
    session: Session,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => match messaging::delete_contact(&user, &contact_id, &pool).await {
                    Ok(_) => HttpResponse::Ok().json("Contact removed"),
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

pub async fn delete_contacts(
    session: Session,
    contacts: web::Json<ContactList>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => {
                    for c in &contacts.contacts {
                        match messaging::delete_contact(&user, c, &pool).await {
                            Ok(_) => {
                                log::info!("Contact removed id:{}", c);
                            }
                            Err(e) => {
                                log::error!("Whoops: {:?}", e);
                            }
                        }
                    }
                    HttpResponse::Ok().json("Contacts removed")
                }
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

pub async fn leave_group(
    group_id: web::Path<String>,
    session: Session,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => match messaging::leave_group(&user, &group_id, &pool).await {
                    Ok(_) => HttpResponse::Ok().json("Group removed"),
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

pub async fn get_contacts(session: Session, pool: web::Data<MySqlPool>) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => match fetch_contacts(&user, &pool).await {
                    Ok(contacts) => HttpResponse::Ok().json(contacts),
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

pub async fn get_groups(session: Session, pool: web::Data<MySqlPool>) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => match get_users_groups(&user, &pool).await {
                    Ok(groups) => HttpResponse::Ok().json(groups),
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

pub async fn get_group(
    group_id: web::Path<String>,
    session: Session,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => match messaging::get_group(&group_id, &user, &pool).await {
                    Ok(group) => HttpResponse::Ok().json(group),
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

pub async fn create_group(
    session: Session,
    new_group: web::Json<NewGroup>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => {
                    let group = messaging::create_group(&user, new_group, &pool).await;
                    match group {
                        Ok(group) => HttpResponse::Ok().json(group),
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

pub async fn create_chat(
    session: Session,
    new_group: web::Json<NewGroup>,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => {
                    if new_group.users.len() > 1 {
                        return HttpResponse::Ok().json("Only one user allowed in chat with you");
                    }
                    let group = messaging::create_chat(&user, new_group, &pool).await;
                    match group {
                        Ok(group) => HttpResponse::Ok().json(group),
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

pub async fn unread_messages(session: Session, pool: web::Data<MySqlPool>) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => {
                    let unread = messaging::unread_messages(&user, &pool).await;
                    HttpResponse::Ok().json(unread)
                }
                Err(_) => HttpResponse::Ok().json(""),
            }
        }
        Ok(None) => HttpResponse::Ok().json("No Session"),
        Err(_) => HttpResponse::Ok().json("Error"),
    }
}

pub async fn get_messages(
    group_id: web::Path<String>,
    session: Session,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(_user) => {
                    let group = messaging::get_messages(&group_id, &pool).await;
                    match group {
                        Ok(group) => HttpResponse::Ok().json(group.messages),
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

pub async fn get_users(
    group_id: web::Path<String>,
    session: Session,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(_user) => {
                    let group = messaging::get_messages(&group_id, &pool).await;
                    match group {
                        Ok(group) => HttpResponse::Ok().json(group.users),
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

pub async fn join_group(
    group_id: web::Path<String>,
    session: Session,
    pool: web::Data<MySqlPool>,
) -> HttpResponse {
    let logged_in = session.get::<String>("tk");
    match logged_in {
        Ok(Some(token)) => {
            let userid = check_session_token(&token, &pool).await;
            match userid {
                Ok(user) => {
                    let group = messaging::join_group(&user, &group_id, &pool).await;
                    match group {
                        Ok(_group) => HttpResponse::Ok().json("Group joined"),
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
