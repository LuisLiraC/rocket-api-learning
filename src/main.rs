#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::response::status;
use std::vec::Vec;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct User {
    name: String,
    age: u8,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Message {
    message: String,
    id: i32,
    user: User,
    hobbies: Vec<String>,
    optional_property: Option<String>
}

#[get("/")]
// fn index() -> status::Created<Json<Vec<Message>>> {
fn index() -> Json<Vec<Message>> {
    let mut messages: Vec<Message> = Vec::new();
    let message1 = Message {
        id: 1,
        message: "Hello, world!".to_string(),
        user: User {
            name: "John".to_string(),
            age: 20
        },
        hobbies: vec![
            "Programming".to_string(),
            "Reading".to_string()
        ],
        optional_property: Some("Some".to_string())
    };
    let message2 = Message {
        id: 2,
        message: "Hello, world! 2".to_string(),
        user: User {
            name: "John".to_string(),
            age: 20
        },
        hobbies: vec![
            "Programming".to_string(),
            "Reading".to_string()
        ],
        optional_property: None
    };

    messages.push(message1);
    messages.push(message2);
    Json(messages)
    // status::Created::new("/".to_string()).body(Json(message))
    // status::Accepted(Some(Json(message)))
}

#[post("/", data = "<data>")]
fn post(data: Json<Message>) -> status::Created<Json<Message>> {
    println!("Message: {}", data.message);
    status::Created::new("/".to_string()).body(data)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![post])
}