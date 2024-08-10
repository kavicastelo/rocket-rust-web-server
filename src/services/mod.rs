use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::models::{User, NewUser};
use crate::schema::users::dsl::*;

pub fn create_user(conn: &PgConnection, new_user: NewUser) -> QueryResult<User> {
    diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)
}

pub fn get_users(conn: &PgConnection) -> QueryResult<Vec<User>> {
    users.load::<User>(conn).expect("Error loading users")
}

pub fn get_user_by_id(conn: &PgConnection, user_id: uuid::Uuid) -> QueryResult<User> {
    users.filter(id.eq(user_id)).first(conn)
}

pub fn update_user(conn: &PgConnection, user_id: uuid::Uuid, updated_user: NewUser) -> QueryResult<User> {
    diesel::update(users.find(user_id))
        .set((&name.eq(updated_user.name), &email.eq(updated_user.email)))
        .get_result(conn)
}

pub fn delete_user(conn: &PgConnection, user_id: uuid::Uuid) -> QueryResult<usize> {
    diesel::delete(users.find(user_id)).execute(conn)
}
