#[macro_use] extern crate rocket;

use rocket_dyn_templates::{Template, context};

use log::info;

#[catch(404)]
fn error_not_found() -> &'static str {
    "Sorry, I can't find that..."
}

#[catch(500)]
fn fatal_error() -> &'static str {
    "Here be dragons... And they are panicking!"
}

#[get("/oops")]
fn oops() -> &'static str {
    panic!("Here we go...");
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello?<name>")]
fn hello(name: Option<&str>) -> Template {
    Template::render("index", context! {
        title: "Hello",
        name: name.unwrap_or("World")
    })
}


#[launch]
fn rocket() -> _ {
    env_logger::init();
    info!("Ready to launch...");

    rocket::build()
        .mount("/", routes![index, oops, hello])
        .register("/", catchers![error_not_found, fatal_error])
        .attach(Template::fairing())
}
