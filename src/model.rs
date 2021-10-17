use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use schema::messages;
use schema::messages::dsl::messages as all_messages;

#[derive(Queryable)]
pub struct Message {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name = "messages"]
pub struct NewMessage {
    pub title: String,
    pub author: String,
    pub published: bool,
}

impl Message {
    pub fn show(id: i32, conn: &PgConnection) -> Vec<Message> {
        all_messages
            .find(id)
            .load::<Message>(conn)
            .expect("Error loading Message")
    }

    pub fn all(conn: &PgConnection) -> Vec<Message> {
        all_messages
            .order(messages::id.desc())
            .load::<Message>(conn)
            .expect("error loading the messages")
    }

    pub fn update_by_id(id: i32, conn: &PgConnection, Message: NewMessage) -> bool {
        use schema::messages::dsl::{author as a, published as p, title as t};
        let NewMessage {
            title,
            author,
            published,
        } = Message;

        diesel::update(all_messages.find(id))
            .set((a.eq(author), p.eq(published), t.eq(title)))
            .get_result::<Message>(conn)
            .is_ok()
    }

    pub fn insert(Message: NewMessage, conn: &PgConnection) -> bool {
        diesel::insert_into(messages::table)
            .values(&Message)
            .execute(conn)
            .is_ok()
    }

    pub fn delete_by_id(id: i32, conn: &PgConnection) -> bool {
        if Message::show(id, conn).is_empty() {
            return false;
        };
        diesel::delete(all_messages.find(id)).execute(conn).is_ok()
    }

    pub fn all_by_author(author: String, conn: &PgConnection) -> Vec<Message> {
        all_messages
            .filter(messages::author.eq(author))
            .load::<Message>(conn)
            .expect("Error loading messages by author")
    }
}