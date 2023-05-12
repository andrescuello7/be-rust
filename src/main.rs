//!Generate event of the rokcet for initializing the server
//! IT is the head of server, in a framework
#[macro_use]
extern crate rocket;

// Here extract the moduls od rokect
// moduls or implement [JSON]
use rocket::serde::json::Json;

// It is other dependencie for template
// TEmplate is for [HANDLEBARS]
use rocket_dyn_templates::{context, Template};

// It is other dependencie for Serialize, Deserialize
// SEREALIZE convert a objet in bytes for conver in ther object
// DESEARIALEZE convert a data serealize in bytes for in object
use serde::{Deserialize, Serialize};

// Here id derive for util serialization and deserialization
// Utility functions for jobs that STRUCT
#[derive(Serialize, Deserialize)]
struct UserModel {
    username: String,
    password: String,
}

// Methods GET for generate Templete
// Templete functions util the index.html.hbs in directory templates
#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        context! {
            title: "Bienvenido"
        },
    )
}

// Methods GET for consult of route "/api/auth"
// Response JSON with UserModel object
#[get("/")]
fn get_user() -> Json<UserModel> {
    let response: UserModel = UserModel {
        username: String::from(""),
        password: String::from(""),
    };
    Json(response)
}

// Methods POST for send data in route "/api/auth"
// Response JSON with UserModel object return JSON(bool)
#[post("/", format = "application/json", data = "<auth>")]
fn post_user(auth: String) -> Json<bool> {
    let response: UserModel = serde_json::from_str(&auth).unwrap();
    if response.password == "admin" && response.username == "conteo" {
        Json(true)
    } else {
        Json(false)
    }
}

// Methods PUT for send data in route "/api/auth"
// Response JSON with UserModel object
#[put("/", format = "application/json", data = "<auth>")]
fn put_user(auth: String) -> Json<UserModel> {
    let response: UserModel = serde_json::from_str(&auth).unwrap();
    Json(response)
}

// Methods DELETE for consult of route "/api/auth"
// Response JSON with UserModel object
#[delete("/")]
fn delete_user() -> Json<UserModel> {
    let response: UserModel = UserModel {
        username: String::from(""),
        password: String::from(""),
    };
    Json(response)
}

// Server initialization here
// Rocket server in listening in "http://127.0.0.1:8000"
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount(
            "/api/auth/",
            routes![get_user, post_user, put_user, delete_user],
        )
        .attach(Template::fairing())
}
