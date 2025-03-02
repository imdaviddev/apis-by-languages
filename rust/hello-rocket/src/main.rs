#[macro_use]
extern crate rocket;

mod controllers;
mod models;
mod services;

#[get("/")]
fn check_alive() -> &'static str {
    "I'm alive"
}

#[get("/hello/<name>")]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![check_alive, greet])
        .mount(
            "/users",
            routes![
                controllers::user_controller::get_users,
                controllers::user_controller::create_user
            ],
        )
}
