use rocket::get;

#[get("/health")]
pub fn health_check() -> &'static str {
    "Server is running!"
}
