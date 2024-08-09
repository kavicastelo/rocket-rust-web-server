use crate::services::user_service;

pub fn get_user(id: u32) -> String {
    user_service::fetch_user(id)
}
