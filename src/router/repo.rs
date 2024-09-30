use salvo::{handler, Request, Router};

pub fn router() -> Router {
    Router::new()
        .get(list_repo)
        .post(new_repo)
        .push(Router::with_path("<repo_id>").get(get_repo))
}

#[handler]
async fn list_repo() -> &'static str {
    "list repo"
}

#[handler]
async fn get_repo(req: &mut Request) -> String {
    let repo_id = req.params().get("repo_id").unwrap();
    format!("get repo {repo_id}")
}

#[handler]
async fn new_repo() -> &'static str {
    "post repo"
}
