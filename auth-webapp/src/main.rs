#[macro_use] extern crate rocket;
use rocket_dyn_templates::Template;

use rocket::form::Context;
use rocket::fs::{FileServer, relative};

#[get("/")]
fn index() -> Template {
    Template::render("index", &Context::default())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("/static")))
}

