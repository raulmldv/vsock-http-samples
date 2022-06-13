
#![feature(plugin, decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

#[macro_use]
extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use dotenv::dotenv;
use std::env;
use routes::*;

mod routes;

fn main() {
    rocket::ignite()
        .mount(
            "/api/v1/",
            rocket::routes![add_op, find_op],
        ).launch();
}