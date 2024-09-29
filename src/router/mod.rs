use salvo::Router;

mod post;
mod repo;

pub fn router() -> Router {
    let router = Router::new()
        .push(Router::with_path("repo").push(repo::router()))
        .push(Router::with_path("post").push(post::router()));
    router
}
