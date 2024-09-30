use salvo::{
    basic_auth::{BasicAuth, BasicAuthValidator},
    handler, Depot, Router,
};

pub struct UserValidator;

impl BasicAuthValidator for UserValidator {
    async fn validate(&self, username: &str, password: &str, _depot: &mut Depot) -> bool {
        username == "salvo" && password == "password"
    }
}

pub fn router() -> Router {
    let new_user_router = Router::new().post(new_user);
    let other_auth_router = Router::with_hoop(BasicAuth::new(UserValidator)).get(get_user);
    Router::new().push(new_user_router).push(other_auth_router)
}

#[handler]
async fn new_user() -> &'static str {
    "new user"
}

#[handler]
async fn get_user() -> &'static str {
    "get user"
}
