use rocket_api::routes::message_routes::{index, post};

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, post])
}