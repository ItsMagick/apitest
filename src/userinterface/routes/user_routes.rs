use std::any::Any;
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use crate::businesslogic::models::user::User;
use crate::businesslogic::repository::user_repository::UserRepository;


#[openapi]
#[get("/users")]
pub async fn get_users() -> Json<Vec<User>> {
    Json(user_routes.run_get_users())
}

#[openapi]
#[post("/users", format = "json", data = "<user>")]
pub async fn create_user(user: Json<User>) -> Json<User> {
    Json(user_handler::create_user(user.into_inner()).await)
}

