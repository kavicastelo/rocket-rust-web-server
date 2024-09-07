use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};
use crate::schema::users;
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,                   // Maps to diesel::sql_types::Uuid
    pub name: String,               // Maps to diesel::sql_types::Text
    pub email: String,              // Maps to diesel::sql_types::Text
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub id: Uuid,
    pub name: String,
    pub email: String
}
