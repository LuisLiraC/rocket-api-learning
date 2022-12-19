use rocket_api::routes::message_routes::{get_routes as get_message_routes};

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/v1/messages/", get_message_routes())
}