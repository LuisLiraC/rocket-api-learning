use rocket::serde::{Serialize, Deserialize};
use crate::models::user::User;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    pub message: String,
    pub id: Option<i32>,
    pub user: User,
    pub hobbies: Vec<String>,
    pub optional_property: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct MessagesList {
    pub messages: Vec<Message>
}

impl MessagesList {
    pub fn get_messages(&mut self) -> Vec<Message> {
        self.messages.clone()
    }

    pub fn save_message(&mut self, message: &Message) -> () {
        let mut new_messages = self.messages.clone();
        new_messages.push(message.clone());
        self.messages = new_messages.clone();
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct MessageError {
    pub error: String
}