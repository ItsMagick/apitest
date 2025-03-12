use serde::{Serialize, Deserialize};
use diesel::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}
impl User {
    pub fn new(id: i32, name: String, email: String, created_at: chrono::NaiveDateTime) -> Self {
        Self { id, name, email, created_at }
    }
}