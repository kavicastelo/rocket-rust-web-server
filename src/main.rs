#[macro_use]
extern crate rocket;

mod controllers;
mod services;
mod models;
mod schema;
mod routes{
    pub mod user;
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(controllers::DbConn::fairing())
        .mount("/", controllers::create_routes())
}
