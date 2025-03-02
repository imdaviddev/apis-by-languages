use rocket::serde::json::Json;
use crate::models::user::User;
use crate::services::user_service;

#[get("/")]
pub fn get_users() -> Json<Vec<User>> {
    Json(user_service::get_all_users())
}

#[post("/", format = "json", data = "<user>")]
pub fn create_user(user: Json<User>) -> Json<User> {
    Json(user_service::create_user(user.into_inner()))
}
