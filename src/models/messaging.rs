use actix_web::web;
use futures::stream::futures_unordered::FuturesUnordered;
use futures::StreamExt;
use guid_create::GUID;

use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlQueryResult;
use sqlx::MySqlPool;
use sqlx::types::chrono::NaiveDateTime;

use crate::models::users::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Contact {
    id: String,
    image_url: Option<String>,
    avatar_url: Option<String>,
    name: String,
    blocked: Option<i8>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ContactList {
    pub contacts: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Group {
    id: String,
    name: String,
    pub messages: Option<Vec<Message>>,
    pub users: Option<Vec<ProfileData>>,
    last_message: Option<NaiveDateTime>,
    unread: Option<i32>,
    chat: Option<i8>,
}

#[derive(Deserialize, Serialize)]
pub struct Grouped {
    id: String,
    name: String,
    last_message: Option<String>,
    last_message_date: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    id: String,
    user_id: String,
    message: String,
    created_at: NaiveDateTime,
    created_time: Option<String>,
    created_date: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewMessage {
    group_id: String,
    message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewGroup {
    name: String,
    pub users: Vec<String>,
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub async fn get_groups(
    user: &str,
    pool: &web::Data<MySqlPool>,
) -> Result<Vec<Grouped>, sqlx::Error> {
    sqlx::query_as!(Grouped,
        r#"
            SELECT 
                id, name, message as last_message,DATE_FORMAT(last_message, "%M %d") as last_message_date
            FROM 
                `groups` 
                JOIN user_groups ON 
                    user_groups.group_id =`groups`.id 
                    AND user_id = ?
                    AND `left` = 0
                LEFT JOIN 
                    (SELECT group_id, message FROM messages LIMIT 1 ) x 
                    ON x.group_id = user_groups.group_id
            ORDER BY `groups`.last_message desc
        "#,
        user
    ).fetch_all(pool.get_ref())
        .await
}

pub async fn new_message(
    user: &str,
    new_message: &web::Json<NewMessage>,
    pool: &web::Data<MySqlPool>,
) -> Result<MySqlQueryResult, sqlx::Error> {
    let mut group = Group::get_group(&new_message.group_id, pool).await;
    if group.chat.unwrap() == 1 {
        log::info!("NEW_MESSAGE:We have a chat");
        group.get_all_users(pool).await;
        match &group.users {
            Some(users) => {
                for (_pos, e) in users.iter().enumerate() {
                    log::info!(
                        "NEW_MESSAGE:User is {} and e.id is {} and GROUP is {}",
                        user,
                        e.id,
                        group.id
                    );
                    if e.id != user {
                        log::info!("NEW_MESSAGE:Checking if we are blocked or not");
                        // check blocked contacts mate
                        let blocked = Group::check_blocked(user, &e.id, pool).await;
                        log::info!("NEW_MESSAGE:Got check back and it is {}", blocked);
                        if blocked {
                            log::info!("NEW_MESSAGE:BLOCKED by recipient {}", &e.id);
                            return group
                                .add_new_message(user, &new_message.message, pool)
                                .await;
                        } else {
                            log::info!("NEW_MESSAGE:MADE GROUP {} all join in", group.id);
                            // make all users members of the group man.
                            sqlx::query!(
                                r#"
                                UPDATE `user_groups`
                                SET `left` = 0, `left_on` = null
                                WHERE group_id = ?
                                "#,
                                group.id,
                            )
                                .execute(pool.get_ref())
                                .await
                                .ok();
                            log::info!("NEW_MESSAGE:Updated user_groups");
                        }
                    } else {
                        log::info!("NEW_MESSAGE:USER=USER");
                    }
                }
            }
            None => {
                log::error!("No users in group")
            }
        }
    }
    group
        .add_new_message(user, &new_message.message, pool)
        .await
}

pub async fn get_users_groups(
    user: &str,
    pool: &web::Data<MySqlPool>,
) -> Result<Vec<Group>, sqlx::Error> {
    let mut rows = sqlx::query!(
        r#"
            SELECT 
                groups.id, 
                groups.name,
                user_groups.unread
            FROM 
                `groups` 
            JOIN user_groups ON group_id = groups.id AND user_id = ? AND `left` = 0
        "#,
        user
    )
        .map(|row| Group {
            id: row.id,
            name: row.name,
            messages: None,
            users: None,
            last_message: None,
            unread: row.unread,
            chat: None,
        })
        .fetch_all(pool.get_ref())
        .await?;
    rows.iter_mut()
        .map(|row| row.fetch_messages(pool))
        .collect::<FuturesUnordered<_>>()
        .collect::<Vec<_>>()
        .await;
    rows.iter_mut()
        .map(|row| row.get_users(pool))
        .collect::<FuturesUnordered<_>>()
        .collect::<Vec<_>>()
        .await;
    rows.iter_mut()
        .map(|row| row.fetch_last_message(pool))
        .collect::<FuturesUnordered<_>>()
        .collect::<Vec<_>>()
        .await;
    rows.sort_by(|a, b| b.last_message.cmp(&a.last_message));
    Ok(rows)
}

pub async fn get_group(
    group_id: &str,
    user: &str,
    pool: &web::Data<MySqlPool>,
) -> Result<Group, sqlx::Error> {
    let mut group = Group::get_group(group_id, pool).await;
    group.fetch_messages(pool).await;
    group.get_users(pool).await;
    group.mark_read(user, pool).await;
    Ok(group)
}

pub async fn fetch_contacts(
    user: &str,
    pool: &web::Data<MySqlPool>,
) -> Result<Vec<Contact>, sqlx::Error> {
    sqlx::query_as!(
        Contact,
        r#"
            SELECT 
                users.id, 
                users.name, 
                users.avatar_url, 
                users.image_url, 
                blocked 
            FROM 
                contacts 
            JOIN users ON contact_id = users.id
            WHERE contacts.user_id = ?
            ORDER BY users.name
        "#,
        user
    )
        .fetch_all(pool.get_ref())
        .await
}

pub async fn add_contact(
    user_id: &str,
    contact_id: &str,
    pool: &web::Data<MySqlPool>,
) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO contacts (user_id, contact_id, blocked)
        VALUES(?, ?, 0)
        "#,
        user_id,
        contact_id,
    )
        .execute(pool.get_ref())
        .await
}

pub async fn delete_contact(
    user_id: &str,
    contact_id: &str,
    pool: &web::Data<MySqlPool>,
) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM contacts 
        WHERE user_id = ? AND contact_id = ?
        "#,
        user_id,
        contact_id,
    )
        .execute(pool.get_ref())
        .await
}

pub async fn block_contact(
    user_id: &str,
    contact_id: &str,
    pool: &web::Data<MySqlPool>,
) -> Result<MySqlQueryResult, sqlx::Error> {
    log::info!("BLOCK::Blocking contact {}", contact_id);
    sqlx::query!(
        r#"
        UPDATE contacts SET blocked = 1
        WHERE (user_id = ? and contact_id = ?) OR
        (user_id = ? and contact_id = ?)
        "#,
        user_id,
        contact_id,
        contact_id,
        user_id,
    )
        .execute(pool.get_ref())
        .await
}

pub async fn create_group(
    user: &str,
    new_group: web::Json<NewGroup>,
    pool: &web::Data<MySqlPool>,
) -> Result<Group, sqlx::Error> {
    let mut group = Group::new_group(&new_group.name, false, pool).await;
    group.add_new_user(user, pool).await?;
    for user_id in &new_group.users {
        let blocked = Group::check_blocked(user_id, user, pool).await;
        if !blocked {
            group.add_new_user(user_id, pool).await?;
        }
    }
    group.get_users(pool).await;

    Ok(group)
}

pub async fn create_chat(
    user: &str,
    new_group: web::Json<NewGroup>,
    pool: &web::Data<MySqlPool>,
) -> Result<Group, sqlx::Error> {
    let mut group = Group::new_group(&new_group.name, true, pool).await;
    group.add_new_user(user, pool).await?;
    for user_id in &new_group.users {
        let blocked = Group::check_blocked(user_id, user, pool).await;
        if !blocked {
            group.add_new_user(user_id, pool).await?;
        }
    }
    group.get_users(pool).await;

    Ok(group)
}

pub async fn leave_group(
    user: &str,
    group: &str,
    pool: &web::Data<MySqlPool>,
) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE `user_groups`
        SET `left` = 1, `left_on` = now()
        WHERE user_id = ?
        AND group_id = ?
        "#,
        user,
        group,
    )
        .execute(pool.get_ref())
        .await
}

pub async fn join_group(
    user: &str,
    group: &str,
    pool: &web::Data<MySqlPool>,
) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO `user_groups`
        (user_id, group_id)
        VALUES(?, ?)
        "#,
        user,
        group,
    )
        .execute(pool.get_ref())
        .await
}

pub async fn unread_messages(user: &str, pool: &web::Data<MySqlPool>) -> i64 {
    let record = sqlx::query!(
        r#"
        SELECT CAST(SUM(unread) AS SIGNED) unread
        FROM user_groups
        WHERE user_id = ? AND `left` = 0
        "#,
        user,
    )
        .fetch_one(pool.get_ref())
        .await;
    match record {
        Ok(record) => record.unread.unwrap(),
        Err(_) => 0,
    }
}

pub async fn check_user_is_in_group(
    user: &str,
    new_message: &web::Json<NewMessage>,
    pool: &web::Data<MySqlPool>,
) -> bool {
    match sqlx::query!(
        r#"
            SELECT * FROM user_groups WHERE user_id = ? and group_id = ? AND `left` = 0
            "#,
        user,
        new_message.group_id
    )
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(_blocked) => true,
        Err(_e) => false,
    }
}

pub async fn get_messages(
    group_id: &str,
    pool: &web::Data<MySqlPool>,
) -> Result<Group, sqlx::Error> {
    let mut group = Group::get_group(group_id, pool).await;
    group.fetch_messages(pool).await;
    Ok(group)
}

pub async fn get_users(group_id: &str, pool: &web::Data<MySqlPool>) -> Result<Group, sqlx::Error> {
    let mut group = Group::get_group(group_id, pool).await;
    group.get_users(pool).await;
    Ok(group)
}

impl Group {
    pub async fn get_group(group_id: &str, pool: &web::Data<MySqlPool>) -> Self {
        let group = sqlx::query!(
            "SELECT name,chat,last_message FROM `groups` WHERE id = ?",
            group_id
        )
            .fetch_one(pool.get_ref())
            .await;
        match group {
            Ok(group) => Self {
                id: group_id.to_string(),
                name: group.name,
                messages: None,
                users: None,
                last_message: group.last_message,
                unread: Some(0),
                chat: group.chat,
            },
            Err(_) => Self {
                id: group_id.to_string(),
                name: "NOTFOUND".to_string(),
                messages: None,
                users: None,
                last_message: None,
                unread: Some(0),
                chat: None,
            },
        }
    }

    pub async fn new_group(name: &str, chat: bool, pool: &web::Data<MySqlPool>) -> Self {
        let guid = GUID::rand();
        sqlx::query!(
            r#"
            INSERT INTO `groups` (id, name, chat)
            VALUES(?, ?, ?)
            "#,
            guid.to_string(),
            name,
            chat,
        )
            .execute(pool.get_ref())
            .await
            .unwrap();
        log::info!("Group id is {}", guid);
        Self {
            id: guid.to_string(),
            name: name.to_string(),
            messages: None,
            users: None,
            last_message: None,
            unread: Some(0),
            chat: Some(chat as i8),
        }
    }
    async fn check_blocked(user: &str, contact: &str, pool: &web::Data<MySqlPool>) -> bool {
        match sqlx::query!(
            r#"
            SELECT * FROM contacts WHERE user_id = ? and contact_id = ? AND blocked = 1
            "#,
            user,
            contact
        )
            .fetch_one(pool.get_ref())
            .await
        {
            Ok(_blocked) => true,
            Err(_e) => false,
        }
    }
    pub async fn add_new_message(
        self,
        user_id: &str,
        message: &str,
        pool: &web::Data<MySqlPool>,
    ) -> Result<MySqlQueryResult, sqlx::Error> {
        let guid = GUID::rand();
        sqlx::query!(
            r#"
            UPDATE `user_groups` SET  `left` = 0
            WHERE group_id = ?
            AND user_id = ?
            "#,
            self.id,
            user_id,
        )
            .execute(pool.get_ref())
            .await?;
        sqlx::query!(
            r#"
            INSERT INTO `messages` (id, group_id, user_id, message, created_at)
            VALUES(?, ?, ?, ?, NOW())
            "#,
            guid.to_string(),
            self.id,
            user_id,
            message,
        )
            .execute(pool.get_ref())
            .await?;
        sqlx::query!(
            r#"
            UPDATE `groups` SET last_message = now()
            WHERE id = ?
            "#,
            self.id
        )
            .execute(pool.get_ref())
            .await?;
        sqlx::query!(
            r#"
            INSERT INTO user_messages
            (user_id, group_id, message_id)
            SELECT user_id, group_id, ? FROM user_groups
            WHERE group_id = ?
            "#,
            guid.to_string(),
            self.id
        )
            .execute(pool.get_ref())
            .await?;
        sqlx::query!(
            r#"
            UPDATE `user_groups` SET unread = unread + 1
            WHERE group_id = ?
            AND user_id != ?
            "#,
            self.id,
            user_id
        )
            .execute(pool.get_ref())
            .await
    }
    pub async fn add_new_user(
        &self,
        user_id: &str,
        pool: &web::Data<MySqlPool>,
    ) -> Result<MySqlQueryResult, sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO `user_groups` (group_id, user_id)
            VALUES(?, ?)
            "#,
            self.id,
            user_id,
        )
            .execute(pool.get_ref())
            .await
    }
    pub async fn fetch_messages(&mut self, pool: &web::Data<MySqlPool>) {
        match sqlx::query_as!(Message,
            r#"
                SELECT id,user_id,message,created_at, 
                    DATE_FORMAT(created_at, "%H:%i") AS created_time, DATE_FORMAT(created_at, "%D %b") AS created_date
                FROM messages
                WHERE
                    group_id = ?
                ORDER BY created_at desc
            "#,
            self.id,
        ).fetch_all(pool.get_ref())
            .await
        {
            Ok(messages) => {
                self.messages = Some(messages);
            }
            Err(_) => {
                self.messages = None;
            }
        }
    }
    pub async fn fetch_last_message(&mut self, pool: &web::Data<MySqlPool>) {
        match sqlx::query_as!(Message,
            r#"
                SELECT id,user_id,message,created_at, 
                    DATE_FORMAT(created_at, "%H:%i") AS created_time, DATE_FORMAT(created_at, "%D %b") AS created_date
                FROM messages
                WHERE
                    group_id = ?
                ORDER BY created_at desc
                LIMIT 1
            "#,
            self.id,
        ).fetch_one(pool.get_ref())
            .await
        {
            Ok(message) => {
                self.last_message = Some(message.created_at);
            }
            Err(_) => {
                self.last_message = None;
            }
        }
    }
    pub async fn mark_read(&self, user: &str, pool: &web::Data<MySqlPool>) {
        let update = sqlx::query!(
            r#"
            UPDATE `user_messages`
            SET `read` = 1, `read_at` = now()
            WHERE group_id =? AND user_id = ?
            "#,
            self.id,
            user
        )
            .execute(pool.get_ref())
            .await;
        match update {
            Ok(_) => {
                log::info!("Cleared group:{} for user:{}", self.id, &user);
            }
            Err(_) => {
                log::error!("Error Cleared group:{} for user:{}", self.id, &user);
            }
        }
        let update = sqlx::query!(
            r#"
            UPDATE `user_groups` SET unread = 0
            WHERE group_id = ? AND user_id = ?
            "#,
            self.id,
            user,
        )
            .execute(pool.get_ref())
            .await;
        match update {
            Ok(_) => {
                log::info!("Cleared group:{} for user:{}", self.id, &user);
            }
            Err(_) => {
                log::error!("Error Cleared group:{} for user:{}", self.id, &user);
            }
        }
    }
    pub async fn get_users(&mut self, pool: &web::Data<MySqlPool>) {
        match sqlx::query_as!(ProfileData,
            r#"
                SELECT users.id, users.name, users.email, users.account_type, users.location, users.embed_url, users.image_url, users.avatar_url, users.bio
                FROM users
                JOIN user_groups ON user_groups.user_id = users.id AND group_id = ? AND `user_groups`.`left` = 0
            "#,
            self.id,
        ).fetch_all(pool.get_ref())
            .await {
            Ok(users) => {
                self.users = Some(users);
            }
            Err(_) => {
                self.users = None
            }
        }
    }
    async fn get_all_users(&mut self, pool: &web::Data<MySqlPool>) {
        match sqlx::query_as!(ProfileData,
            r#"
                SELECT users.id, users.name, users.email, users.account_type, users.location, users.embed_url, users.image_url, users.avatar_url, users.bio
                FROM users
                JOIN user_groups ON user_groups.user_id = users.id AND group_id = ?
            "#,
            self.id,
        ).fetch_all(pool.get_ref())
            .await {
            Ok(users) => {
                self.users = Some(users);
            }
            Err(_) => {
                self.users = None
            }
        }
    }
}
