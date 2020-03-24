#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;
extern crate chrono_tz;
extern crate rand;
extern crate reqwest;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod cody_time;
mod magic_8ball;
mod routes;
mod telegram_api;
mod token;

fn main() {
    rocket::ignite().mount("/", routes::routes()).launch();
}
