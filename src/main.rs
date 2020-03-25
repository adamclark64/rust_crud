#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod post;
// use post::post;

#[get("/<title>/<body>")]
fn poster(title:String, body: String) {
    post::post(title, body)
}

fn main() {
    rocket::ignite().mount("/", routes![poster]).launch();
}