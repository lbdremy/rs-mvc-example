extern crate iron;

mod routes;
mod controllers;
mod models;
mod helpers;

use app::routes::*;
use self::iron::prelude::Iron;

pub fn run() {
    Iron::new(routes::all()).http("localhost:3000").unwrap();
}
