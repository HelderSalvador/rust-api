#[macro_use]
extern crate diesel;

extern crate dotenv;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;

mod schema;
mod models;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = PgConnection::establish(&database_url).unwrap();

    let Message = models::NewMessage {
        content: String::from("Ceci est un message !"),
        author: String::from("Satoshi Nakamoto"),
        published: true,
    };

    if models::Message::insert(Message, &conn) {
        println!("success");
    } else {
        println!("failed");
    }
}