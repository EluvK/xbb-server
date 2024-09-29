use salvo::{handler, Request, Router};

pub fn router() -> Router {
    Router::new().get(list_post).post(post_post).push(
        Router::with_path("<post_id>")
            .get(get_post)
            .put(put_post)
            .delete(delete_post),
    )
}

#[handler]
async fn list_post() -> &'static str {
    "list post"
}

#[handler]
async fn post_post() -> &'static str {
    "post post"
}

#[handler]
async fn get_post(req: &mut Request) -> String {
    let post_id = req.params().get("post_id").unwrap();
    format!("get post {post_id}")
}

#[handler]
async fn put_post(req: &mut Request) -> String {
    let post_id = req.params().get("post_id").unwrap();
    format!("put post {post_id}")
}

#[handler]
async fn delete_post(req: &mut Request) -> String {
    let post_id = req.params().get("post_id").unwrap();
    format!("delete post {post_id}")
}
