#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket!"
}

#[catch(404)]
fn not_found() -> &'static str {
    "404 - Not Found"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .register("/", catchers![not_found])
}
