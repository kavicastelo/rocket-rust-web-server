use rocket::get;
use crate::controllers::user_controller;

#[get("/user/<id>")]
pub fn get_user(id: u32) -> String {
    user_controller::get_user(id)
}
