use rocket::serde::json::Json;

use crate::models::user::User;

#[rocket::get("/")]
fn index() -> Json<Vec<User>> {
    let user = User {
        name: String::from("Luis Lira"),
        age: 29
    };
    Json(vec![user])
}

pub fn get_routes() -> Vec<rocket::Route> {
    rocket::routes![index]
}