extern crate router;

use app::controllers::UserController;
use self::router::Router;

pub fn all() -> Router {
	let mut router = Router::new();  
    router.post("/users", UserController::create);
    router.get("/users/:id", UserController::read);
    router.put("/users/:id", UserController::update);
    router.delete("/users/:id", UserController::delete);
    router.get("/users", UserController::read_all);
    router
}