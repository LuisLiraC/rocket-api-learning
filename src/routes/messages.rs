use rocket::serde::json::{Json, Error};
use rocket::response::status;
use std::vec::Vec;
use rocket::http::Status;

use crate::models::message::{Message, MessagesList, MessageError};
use crate::models::user::User;

static mut MESSAGES: MessagesList = MessagesList { messages: Vec::new() };

#[rocket::get("/")]
fn index() -> Json<Vec<Message>> {
    Json(unsafe { MESSAGES.get_messages() })
}

#[rocket::post("/", data = "<data>")]
fn post(data: Result<Json<Message>, Error>) -> Result<status::Created<Json<Message>>, status::Custom<Json<MessageError>>> {
    match data {
        Ok(data) => {
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
            unsafe { MESSAGES.save_message(&new_message).unwrap() };
            Ok(status::Created::new("/".to_string()).body(Json(new_message)))
        },
        Err(error) => {
            let error = MessageError {
                error: error.to_string()
            };
            Err(status::Custom(Status::BadRequest, Json(error)))
        }
    }
}

pub fn get_routes() -> Vec<rocket::Route> {
    rocket::routes![index, post]
}