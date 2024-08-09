#[macro_use] extern crate rocket;

mod routes;
mod controllers;
mod services;

use routes::health::health_check;
use routes::user::get_user;

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, health_check, get_user])
}