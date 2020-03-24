mod index;
mod webhook;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![index::index, webhook::create]
}
