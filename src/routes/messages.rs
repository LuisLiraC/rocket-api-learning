use rocket::serde::json::{Json, Error};
use rocket::response::status;
use std::vec::Vec;
use rocket::http::Status;

use crate::models::message::{Message, MessagesList, MessageError};

static mut MESSAGES: MessagesList = MessagesList { messages: Vec::new() };

#[rocket::get("/")]
fn index() -> Json<Vec<Message>> {
    Json(unsafe { MESSAGES.get_messages() })
}

#[rocket::post("/", data = "<data>")]
fn post(data: Result<Json<Message>, Error>) -> Result<status::Created<Json<Message>>, status::Custom<Json<MessageError>>> {
    match data {
        Ok(mut data) => {
            data.id = Some(unsafe { MESSAGES.messages.len() as i32 });
            unsafe { MESSAGES.save_message(&data) };
            Ok(status::Created::new("/".to_string()).body(Json(data.into_inner())))
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