use salvo::{basic_auth::BasicAuth, Router};

mod post;
mod repo;
mod user;

pub fn router() -> Router {
    let function_router = Router::with_hoop(BasicAuth::new(user::UserValidator))
        .push(Router::with_path("repo").push(repo::router()))
        .push(Router::with_path("post").push(post::router()));
    let user_router = Router::with_path("user").push(user::router());
    Router::new().push(function_router).push(user_router)
}
