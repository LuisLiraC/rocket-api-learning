#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::response::status;
use std::vec::Vec;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
struct User {
    name: String,
    age: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
struct Message {
    message: String,
    id: Option<i32>,
    user: User,
    hobbies: Vec<String>,
    optional_property: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct MessagesList {
    messages: Vec<Message>
}

impl MessagesList {
    fn get_messages(&mut self) -> Vec<Message> {
        self.messages.clone()
    }

    fn save_message(&mut self, message: &Message) -> Result<(), String> {
        let mut new_messages = self.messages.clone();
        new_messages.push(message.clone());
        self.messages = new_messages.clone();
        Ok(())
    }
}

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