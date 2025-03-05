use rocket::{routes, Rocket, Build};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use crate::dataaccess::database::connector::postgres_connector::establish_connection;
use crate::userinterface::routes::user_routes::{get_users, create_user};

pub fn launch_server() -> Rocket<Build> {
    let db_pool = establish_connection();
    rocket::build()
        .mount("/api", routes![get_users, create_user])
        .mount("/swagger", make_swagger_ui(&get_docs()))
}

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/swagger/openapi.json".to_string(),
        ..Default::default()
    }
}