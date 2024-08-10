use rocket_sync_db_pools::database;
use crate::models::{User, NewUser};
use crate::services;
use diesel::PgConnection;

use rocket::serde::json::Json;
use rocket::serde::uuid;

#[database("postgres_db")]
pub struct DbConn(PgConnection);

#[get("/users")]
pub async fn get_users(conn: DbConn) -> Json<Vec<User>> {
    conn.run(|c| services::get_users(c)).await.map(Json).unwrap()
}

#[post("/users", data = "<new_user>")]
pub async fn create_user(conn: DbConn, new_user: Json<NewUser>) -> Json<User> {
    conn.run(|c| services::create_user(c, new_user.into_inner())).await.map(Json).unwrap()
}

#[get("/users/<id>")]
pub async fn get_user(conn: DbConn, id: uuid::Uuid) -> Json<User> {
    conn.run(move |c| services::get_user_by_id(c, id)).await.map(Json).unwrap()
}

#[put("/users/<id>", data = "<user>")]
pub async fn update_user(conn: DbConn, id: uuid::Uuid, user: Json<NewUser>) -> Json<User> {
    conn.run(move |c| services::update_user(c, id, user.into_inner())).await.map(Json).unwrap()
}

#[delete("/users/<id>")]
pub async fn delete_user(conn: DbConn, id: uuid::Uuid) -> Json<usize> {
    conn.run(move |c| services::delete_user(c, id)).await.map(Json).unwrap()
}

pub fn create_routes() -> Vec<rocket::Route> {
    routes![get_users, create_user, get_user, update_user, delete_user]
}
