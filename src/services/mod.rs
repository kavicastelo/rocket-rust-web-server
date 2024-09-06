use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::sql_types::{Uuid as DieselUuid, Text, Timestamptz};
use crate::models::{User, NewUser};
use crate::schema::users::dsl::{users, id, name, email, created_at};
use uuid::Uuid;
use chrono::Utc;

pub fn create_user(conn: &mut PgConnection, new_user: NewUser) -> QueryResult<User> {
    diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)
}

pub fn get_users(conn: &mut PgConnection) -> QueryResult<Vec<User>> {
    users.load::<User>(conn)
}

pub fn get_user_by_id(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<User> {
    users.filter(id.eq(user_id)).first(conn)
}

pub fn update_user(conn: &mut PgConnection, user_id: Uuid, updated_user: NewUser) -> QueryResult<User> {
    diesel::update(users.find(user_id))
        .set((
            name.eq(updated_user.name),
            email.eq(updated_user.email),
        ))
        .get_result(conn)
}

pub fn delete_user(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<usize> {
    diesel::delete(users.find(user_id)).execute(conn)
}
