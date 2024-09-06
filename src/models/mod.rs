use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};
use crate::schema::users;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,                   // Maps to diesel::sql_types::Uuid
    pub name: String,               // Maps to diesel::sql_types::Text
    pub email: String,              // Maps to diesel::sql_types::Text
    pub created_at: Option<DateTime<Utc>>,  // Maps to diesel::sql_types::Nullable<Timestamptz>
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}
