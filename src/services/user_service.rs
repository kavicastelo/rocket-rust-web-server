pub fn fetch_user(id: u32) -> String {
    // For simplicity, return a static user string. Normally, this would query a database.
    format!("User ID: {}", id)
}
