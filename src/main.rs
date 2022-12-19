use rocket_api::routes::{
    users::get_routes as get_user_routes,
    messages::get_routes as get_message_routes,
};

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/v1/messages/", get_message_routes())
        .mount("/api/v1/users/", get_user_routes())
}