use rocket::serde::json::Json;
use rocket::State;
use diesel::pg::PgConnection;
use rocket_sync_db_pools::database;
use crate::services;
use crate::models::{User, NewUser};
use uuid::Uuid;
use rocket::response::status::BadRequest;

#[database("postgres_db")]
pub struct DbConn(PgConnection);

#[get("/users")]
async fn get_users(conn: DbConn) -> Json<Vec<User>> {
    conn.run(|c| services::get_users(c)).await.map(Json).unwrap()
}

#[post("/users", data = "<new_user>")]
async fn create_user(conn: DbConn, new_user: Json<NewUser>) -> Json<User> {
    conn.run(|c| services::create_user(c, new_user.into_inner())).await.map(Json).unwrap()
}

#[get("/users/<id>")]
async fn get_user(conn: DbConn, id: &str) -> Result<Json<User>, BadRequest<String>> {
    // Parse the string into a UUID and map the error to BadRequest<String>
    let uuid = Uuid::parse_str(id).map_err(|_| BadRequest(format!("Invalid UUID: {}", id)))?;

    // Fetch the user by UUID, map the service error to BadRequest<String>
    conn.run(move |c| services::get_user_by_id(c, uuid))
        .await
        .map(Json)
        .map_err(|e| BadRequest(e.to_string()))
}

#[put("/users/<id>", data = "<user>")]
async fn update_user(
    conn: DbConn,
    id: &str,
    user: Json<NewUser>
) -> Result<Json<User>, BadRequest<String>> {
    // Parse the UUID from the string
    let uuid = Uuid::parse_str(id).map_err(|_| BadRequest(format!("Invalid UUID: {}", id)))?;

    // Update the user in the database
    conn.run(move |c| services::update_user(c, uuid, user.into_inner()))
        .await
        .map(Json)
        .map_err(|e| BadRequest(e.to_string()))
}

#[delete("/users/<id>")]
async fn delete_user(
    conn: DbConn,
    id: &str
) -> Result<Json<usize>, BadRequest<String>> {
    // Parse the UUID from the string
    let uuid = Uuid::parse_str(id).map_err(|_| BadRequest(format!("Invalid UUID: {}", id)))?;

    // Delete the user in the database
    conn.run(move |c| services::delete_user(c, uuid))
        .await
        .map(Json)
        .map_err(|e| BadRequest(e.to_string()))
}

pub fn create_routes() -> Vec<rocket::Route> {
    routes![get_users, create_user, get_user, update_user, delete_user]
}
