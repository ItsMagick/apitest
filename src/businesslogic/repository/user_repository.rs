use crate::businesslogic::models::user::User;
use crate::dataaccess::database::dao::user_dao;

/**
    * The UserRepository struct is used to interact with the database.
    * It has a connection pool to the database.
*/
pub struct UserRepository {
    // pub pool: DbPool,
}

impl UserRepository {
    // pub fn new(pool: DbPool) -> Self {
    //     Self { pool }
    // }

    /**
        * Get all users from the database.
        *
        * @return A vector of all users in the database.
    */
    pub fn get_all_users(&self) -> Vec<User> {
        // let mut conn = self.pool.get().expect("Failed to get DB connection");
        // user_dao::get_users(&mut conn)
        vec![
            User::new(1, "John@mail.com".to_string(), "Doe".to_string())
            ]
    }

    /**
        * Add a user to the database.
        *
        * @param user The user to add.
        * @return The added user.
    */
    pub fn add_user(&self, user: User) -> User {
        // let mut conn = self.pool.get().expect("Failed to get DB connection");
        // user_dao::create_user(&mut conn, &user)
        User::new(1, "John@mail.com".to_string(), "Doe".to_string())
    }
}
