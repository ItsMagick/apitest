use crate::businesslogic::models::user::User;
use crate::dataaccess::database::dao::user_dao;

pub struct UserRepository {
    pub pool: DbPool,
}

impl UserRepository {
    // pub fn new(pool: DbPool) -> Self {
    //     Self { pool }
    // }

    pub fn get_all_users(&self) -> Vec<User> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        user_dao::get_users(&mut conn)
    }

    pub fn add_user(&self, user: User) -> User {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        user_dao::create_user(&mut conn, &user)
    }
}
