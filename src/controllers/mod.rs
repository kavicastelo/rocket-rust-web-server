use rocket::serde::json::Json;
use rocket::State;
use diesel::pg::PgConnection;
use rocket_sync_db_pools::database;
use crate::services;
use crate::models::{User, NewUser};
use uuid::Uuid;

#[database("postgres_db")]
struct DbConn(PgConnection);

#[get("/users")]
async fn get_users(conn: DbConn) -> Json<Vec<User>> {
    conn.run(|c| services::get_users(c)).await.map(Json).unwrap()
}

#[post("/users", data = "<new_user>")]
async fn create_user(conn: DbConn, new_user: Json<NewUser>) -> Json<User> {
    conn.run(|c| services::create_user(c, new_user.into_inner())).await.map(Json).unwrap()
}

#[get("/users/<id>")]
async fn get_user(conn: DbConn, id: Uuid) -> Json<User> {
    conn.run(move |c| services::get_user_by_id(c, id)).await.map(Json).unwrap()
}

#[put("/users/<id>", data = "<user>")]
async fn update_user(conn: DbConn, id: Uuid, user: Json<NewUser>) -> Json<User> {
    conn.run(move |c| services::update_user(c, id, user.into_inner())).await.map(Json).unwrap()
}

#[delete("/users/<id>")]
async fn delete_user(conn: DbConn, id: Uuid) -> Json<usize> {
    conn.run(move |c| services::delete_user(c, id)).await.map(Json).unwrap()
}

pub fn create_routes() -> Vec<rocket::Route> {
    routes![get_users, create_user, get_user, update_user, delete_user]
}
