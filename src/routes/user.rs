use rocket::Route;

use crate::controllers::{create_user, delete_user, get_user, update_user};

pub fn routes() -> Vec<Route> {
    routes![
        create_user,
        get_user,
        update_user,
        delete_user,
    ]
}
