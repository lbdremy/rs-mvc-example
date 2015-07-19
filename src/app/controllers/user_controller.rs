extern crate iron;
extern crate router;
extern crate rustc_serialize;

use app::models::User;
use app::helpers::UserHelper;
use self::iron::prelude::*;
use self::router::Router;
use self::rustc_serialize::json;

pub struct UserController;

impl UserController {
	pub fn create(_: &mut Request) -> IronResult<Response> {
	    Ok(Response::with((iron::status::Ok)))
	}
	pub fn read(req: &mut Request) -> IronResult<Response> {
		let id = req.extensions.get::<Router>().unwrap().find("id").unwrap().parse::<i64>().unwrap();
		let user = User::get_by_id(id);
		let encoded_user = json::encode(&user).unwrap();
	    Ok(Response::with((iron::status::Ok, encoded_user)))
	}
	pub fn update(_: &mut Request) -> IronResult<Response> {
	    Ok(Response::with((iron::status::Ok)))
	}
	pub fn delete(_: &mut Request) -> IronResult<Response> {
	    Ok(Response::with((iron::status::Ok)))
	}
	pub fn read_all(_: &mut Request) -> IronResult<Response> {
	    Ok(Response::with((iron::status::Ok)))
	}
}