use rocket::serde::json::Json;
use rocket::response::status;
use std::vec::Vec;

use crate::models::message::{Message, MessagesList};
use crate::models::user::User;

static mut MESSAGES: MessagesList = MessagesList { messages: Vec::new() };

#[rocket::get("/")]
pub fn index() -> Json<Vec<Message>> {
    Json(unsafe { MESSAGES.get_messages() })
}

#[rocket::post("/", data = "<data>")]
pub fn post(data: Json<Message>) -> status::Created<Json<Message>> {

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