use chrono::Utc;
use salvo::{handler, http::StatusCode, Depot, Request, Response, Router};
use tracing::info;

use crate::{
    error::ServiceResult,
    model::{
        post::{
            add_post, erase_post, get_post_by_id, list_posts_by_repo_id, update_post,
            OpenApiGetPostResponse, OpenApiListPostResponse, OpenApiNewPostRequest, Post,
        },
        repo::check_repo_owner,
    },
    router::utils::{get_current_user_id, get_req_path},
};

pub fn router() -> Router {
    Router::new().get(list_post).post(new_post).push(
        Router::with_path("<post_id>")
            .get(get_post)
            .put(put_post)
            .delete(delete_post),
    )
}

#[handler]
async fn list_post(req: &mut Request, depot: &mut Depot) -> ServiceResult<OpenApiListPostResponse> {
    let current_user_id = get_current_user_id(depot)?;
    let repo_id = get_req_path(req, "repo_id")?;
    check_repo_owner(&repo_id, current_user_id)?;
    info!("list post {repo_id}");

    let post = list_posts_by_repo_id(repo_id.as_str())?;
    Ok(OpenApiListPostResponse(
        post.into_iter().map(|post| post.into()).collect(),
    ))
}

#[handler]
async fn get_post(req: &mut Request, depot: &mut Depot) -> ServiceResult<OpenApiGetPostResponse> {
    let current_user_id = get_current_user_id(depot)?;
    let repo_id = get_req_path(req, "repo_id")?;
    let post_id = get_req_path(req, "post_id")?;
    check_repo_owner(&repo_id, current_user_id)?;
    let post = get_post_by_id(&post_id)?;
    Ok(post.into())
}

#[handler]
async fn new_post(req: &mut Request, depot: &mut Depot, resp: &mut Response) -> ServiceResult<()> {
    let current_user_id = get_current_user_id(depot)?;
    let repo_id = get_req_path(req, "repo_id")?;
    check_repo_owner(&repo_id, current_user_id)?;

    info!("new post for {repo_id}");
    let req = req.parse_body::<OpenApiNewPostRequest>().await?;
    let post = Post::new(req.title, req.content, current_user_id.clone(), repo_id);
    add_post(&post)?;

    resp.status_code(StatusCode::CREATED);
    Ok(())
}

#[handler]
async fn put_post(req: &mut Request, depot: &mut Depot) -> ServiceResult<()> {
    let current_user_id = get_current_user_id(depot)?;
    let repo_id = get_req_path(req, "repo_id")?;
    let post_id = get_req_path(req, "post_id")?;
    check_repo_owner(&repo_id, current_user_id)?;

    info!("update post {post_id}");
    let req = req.parse_body::<OpenApiNewPostRequest>().await?;
    let mut post = get_post_by_id(&post_id)?;
    post.title = req.title;
    post.content = req.content;
    post.updated_at = Utc::now();
    update_post(&post)
}

#[handler]
async fn delete_post(req: &mut Request, depot: &mut Depot) -> ServiceResult<()> {
    let current_user_id = get_current_user_id(depot)?;
    let repo_id = get_req_path(req, "repo_id")?;
    let post_id = get_req_path(req, "post_id")?;
    check_repo_owner(&repo_id, current_user_id)?;

    info!("delete post {post_id}");
    erase_post(&post_id)
}
