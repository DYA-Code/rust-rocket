#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// #[get("/wasans")]
// fn wasans() -> &'static str {
//     "SANS"
// }

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
