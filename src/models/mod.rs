use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};
use chrono::{NaiveDateTime};
use crate::schema::users;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub created_at: (NaiveDateTime),
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}
