#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    // TODO set up index page
    "Hello, world!"
}

#[post("/parse", data = "<code>")]
fn parse(code. String) -> String {
    unimplemented!()
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
