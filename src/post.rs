use rocket::serde::{Serialize, Deserialize};
use diesel::{QueryResult, RunQueryDsl};
use diesel::prelude::*;

use crate::DbConn;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name="posts"]
pub struct Post {
    #[serde(skip_deserializing)]
    id: Option<i32>,
    title: String,
    body: String,
    #[serde(skip_deserializing)]
    published: bool,
}

table! {
    posts (id) {
        id -> Nullable<Integer>,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

impl Post {
    pub async fn all(conn: &DbConn) -> QueryResult<Vec<Post>> {
        conn.run(|c| {
            posts::table.order(posts::id.desc()).load::<Post>(c)
        }).await
    }

    /// Returns the number of affected rows: 1.
    pub async fn insert(post: Post, conn: &DbConn) -> QueryResult<Post> {
        conn.run(|c| {
            let p = Post { id: None, title: post.title, body: post.body, published: false };
            diesel::insert_into(posts::table).values(&p).get_result(c)
        }).await
    }

    pub async fn get(id: i32, conn: &DbConn) -> QueryResult<Post> {
        conn.run(move |c| {
            posts::table.find(id).get_result(c)
        }).await
    }

    pub async fn delete(id: i32, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            diesel::delete(posts::table).filter(posts::id.eq(id)).execute(c)
        }).await
    }
}