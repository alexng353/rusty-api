use rocket::fs::FileServer;

#[macro_use]
extern crate rocket;

mod db;
mod macros;
mod routes;

use routes::*;

#[launch]
fn rocket() -> _ {
    let rocket = mount_routes!(rocket::build(), index::index, user::new_user, test::test)
        .mount("/", FileServer::from("static/"));

    rocket
}
