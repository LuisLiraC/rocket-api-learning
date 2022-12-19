#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rocket::response::status;
use std::vec::Vec;

use rocket_api::models::user::User;
use rocket_api::models::message::{Message, MessagesList};

static mut MESSAGES: MessagesList = MessagesList { messages: Vec::new() };

#[get("/")]
fn index() -> Json<Vec<Message>> {
    Json(unsafe { MESSAGES.get_messages() })
}

#[post("/", data = "<data>")]
fn post(data: Json<Message>) -> status::Created<Json<Message>> {

    let new_message = Message {
        message: data.message.clone(),
        id: Some(unsafe { MESSAGES.messages.len() as i32 }),
        user: User {
            name: data.user.name.clone(),
            age: data.user.age.clone()
        },
        hobbies: data.hobbies.clone(),
        optional_property: data.optional_property.clone()
    };

    unsafe { MESSAGES.save_message(&new_message).unwrap(); }
    status::Created::new("/".to_string()).body(Json(new_message))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, post])
}