use diesel::prelude::*;

/**
    * The User struct represents a user in the system.
    * It has an ID, a name, and an email.
*/
#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::dataaccess::database::schema::users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl User {
    /**
        * Create a new User instance with ctor.
        *
        * @param id The user's ID.
        * @param name The user's name.
        * @param email The user's email.
        * @return A new User instance.
    */
    pub fn new(id: i32, name: String, email: String) -> Self {
        Self { id, name, email }
    }
}