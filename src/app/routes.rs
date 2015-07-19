extern crate router;

use app::controllers::UserController;
use self::router::Router;

pub fn all() -> Router {
	let mut router = Router::new();  // Alternative syntax:
    router.get("/", UserController::get);        // let router = router!(get "/" => handler,
    router.get("/:query", UserController::other);
    router
}