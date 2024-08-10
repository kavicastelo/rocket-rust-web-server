use crate::services::user_service;
use crate::models::user::User;
use rocket_contrib::json::Json;
use rocket::State;
use diesel::PgConnection;
use rocket_codegen::{delete, get, post, put};

#[post("/user", data = "<user>")]
pub fn create_user(conn: State<PgConnection>, user: Json<User>) -> Json<User> {
    Json(user_service::create_user(&conn, &user.into_inner()).unwrap())
}

#[get("/user/<id>")]
pub fn get_user(conn: State<PgConnection>, id: i32) -> Json<User> {
    Json(user_service::get_user(&conn, id).unwrap())
}

#[put("/user/<id>", data = "<user>")]
pub fn update_user(conn: State<PgConnection>, id: i32, user: Json<User>) -> Json<User> {
    Json(user_service::update_user(&conn, id, &user.into_inner()).unwrap())
}

#[delete("/user/<id>")]
pub fn delete_user(conn: State<PgConnection>, id: i32) -> String {
    user_service::delete_user(&conn, id).unwrap().to_string()
}
