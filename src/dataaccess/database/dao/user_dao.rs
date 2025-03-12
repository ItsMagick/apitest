use diesel::prelude::*;
use crate::businesslogic::models::user::User;
use crate::dataaccess::database::schema::users;

pub fn get_users(conn: &mut PgConnection) -> Vec<User> {
    users::table.load::<User>(conn).expect("Error loading users")
}

pub fn create_user(conn: &mut PgConnection, new_user: &User) -> User {
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(conn)
        .expect("Error inserting user")
}
