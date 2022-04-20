#[macro_use] extern crate rocket;
use rocket::serde::Serialize;
use rocket::serde::json::Json;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, rates])
}

#[derive(Serialize)]
struct Rates { foo: String }

#[get("/rates")]
fn rates() -> Json<Rates> {
    Json(Rates { foo: "bar".to_string() })
}