#[macro_use] extern crate rocket;

mod businesslogic;
mod dataaccess;
mod userinterface;
// mod util;

use userinterface::webserver::launch_server;

#[launch]
fn rocket() -> _ {
    launch_server()
}