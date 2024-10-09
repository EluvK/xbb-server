use salvo::{basic_auth::BasicAuth, handler, http::StatusCode, Response, Router};

mod post;
mod repo;
mod user;
mod utils;

pub fn router() -> Router {
    let function_router = Router::with_hoop(BasicAuth::new(user::UserValidator))
        .push(Router::with_path("repo").push(repo::router()))
        .push(Router::with_path("repo/<repo_id>/post").push(post::router()));
    let user_router = Router::with_path("user").push(user::router());
    let health_router = Router::with_path("health").get(health);
    Router::new()
        .push(function_router)
        .push(user_router)
        .push(health_router)
}

#[handler]
async fn health(resp: &mut Response) {
    resp.status_code(StatusCode::OK);
    resp.render("OK");
}
