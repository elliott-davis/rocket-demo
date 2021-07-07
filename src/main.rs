#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate diesel;

use rocket::{Rocket, Build};
use rocket_sync_db_pools::database;
use rocket::fairing::AdHoc;
use rocket::serde::json::{Json, Value, json};

mod post;

use crate::post::Post;
use rocket::response::Debug;
use diesel::QueryResult;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[database("rocket-demo")]
pub struct DbConn(diesel::PgConnection);

#[get("/")]
async fn index() -> &'static str {
    "Hello World"
}

#[get("/posts")]
async fn listPosts(conn: DbConn) -> Result<Json<Vec<Post>>> {
    let posts = Post::all(&conn).await?;

    Ok(Json(posts))
}

#[post("/posts", format = "json", data = "<post>")]
async fn newPost(post: Json<Post>, conn: DbConn) -> Result<Json<Post>> {
    let post = Post::insert(post.into_inner(), &conn).await?;

    Ok(Json(post))
}

#[get("/posts/<id>")]
async fn getPost(id: i32, conn:DbConn) -> Result<Json<Post>> {
    let post = Post::get(id, &conn).await?;

    Ok(Json(post))
}

#[delete("/posts/<id>")]
async fn deletePost(id: i32, conn:DbConn) -> Result<()> {
    Post::delete(id, &conn).await?;

    Ok(())
}

#[rocket::main]
async fn main() {
    rocket::build()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Run Migrations", run_migrations))
        .mount("/", routes![index, listPosts, newPost, getPost, deletePost])
        .launch()
        .await;
}

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    // This macro from `diesel_migrations` defines an `embedded_migrations`
    // module containing a function named `run` that runs the migrations in the
    // specified directory, initializing the database.
    embed_migrations!("migrations");

    let conn = DbConn::get_one(&rocket).await.expect("database connection");
    conn.run(|c| embedded_migrations::run(c)).await.expect("diesel migrations");

    rocket
}