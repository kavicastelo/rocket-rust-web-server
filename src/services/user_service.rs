use diesel::prelude::*;
use crate::models::user::User;
use crate::schema::users::dsl::*;

pub fn create_user(conn: &PgConnection, new_user: &User) -> QueryResult<User> {
    diesel::insert_into(users)
        .values(new_user)
        .get_result(conn)
}

pub fn get_user(conn: &PgConnection, user_id: i32) -> QueryResult<User> {
    users.find(user_id).get_result::<User>(conn)
}

pub fn update_user(conn: &PgConnection, user_id: i32, updated_user: &User) -> QueryResult<User> {
    diesel::update(users.find(user_id))
        .set(updated_user)
        .get_result(conn)
}

pub fn delete_user(conn: &PgConnection, user_id: i32) -> QueryResult<usize> {
    diesel::delete(users.find(user_id)).execute(conn)
}
