extern crate iron;

use app::models::User;
use app::helpers::UserHelper;
use self::iron::prelude::*;

pub struct UserController;

impl UserController {
	pub fn get(_: &mut Request) -> IronResult<Response> {
		User::get();
		UserHelper::pretty();
	    Ok(Response::with((iron::status::Ok, "Hello There")))
	}
	pub fn other(_: &mut Request) -> IronResult<Response> {
	    Ok(Response::with((iron::status::Ok, "Hello Other")))
	}
}
