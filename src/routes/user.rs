use rocket::get;

#[get("/user/<id>")]
pub fn get_user(id: u32) -> String {
    format!("User ID: {}", id)
}
