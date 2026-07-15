#[macro_use]
extern crate rocket;

mod controllers;
mod dtos;
mod model_views;
mod models;
mod servicos;

use controllers::{home_controller, recursos_controller};

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![home_controller::index, recursos_controller::index,],
    )
}
