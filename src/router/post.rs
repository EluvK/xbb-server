use salvo::{handler, http::StatusCode, Depot, Request, Response, Router};
use tracing::info;

use crate::{
    error::{ServiceError, ServiceResult},
    model::{
        post::{
            add_post, erase_post, get_post_by_id, list_posts_by_repo_id, update_post,
            OpenApiGetPostResponse, OpenApiListPostResponse, OpenApiPushPostRequest, Post,
        },
        repo::get_repo_by_id,
        subscribe::check_subscribe,
    },
    router::utils::{get_current_user_id, get_req_path},
};

pub fn router() -> Router {
    Router::new().get(list_post).post(push_post).push(
        Router::with_path("<post_id>")
            .get(get_post)
            .delete(delete_post),
    )
}

#[handler]
async fn list_post(req: &mut Request, depot: &mut Depot) -> ServiceResult<OpenApiListPostResponse> {
    let current_user_id = get_current_user_id(depot)?;
    let repo_id = get_req_path(req, "repo_id")?;

    check_owner_or_subscribe(&repo_id, current_user_id)?;
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
    check_owner_or_subscribe(&repo_id, current_user_id)?;
    let post = get_post_by_id(&post_id)?;
    match post {
        Some(post) => Ok(post.into()),
        None => Err(ServiceError::NotFound("post not found".to_owned())),
    }
}

#[handler]
async fn push_post(
    request: &mut Request,
    response: &mut Response,
    depot: &mut Depot,
) -> ServiceResult<()> {
    info!("push post");
    let current_user_id = get_current_user_id(depot)?;
    let repo_id = get_req_path(request, "repo_id")?;
    let post: Post = request.parse_body::<OpenApiPushPostRequest>().await?.into();
    if post.repo_id != *repo_id {
        return Err(ServiceError::NotFound("repo_id not match".to_owned()));
    }
    if get_repo_by_id(&repo_id)?.is_none_or(|repo| repo.owner != *current_user_id) {
        return Err(ServiceError::Forbidden("auth failed".to_owned()));
    }
    match get_post_by_id(&post.id)? {
        Some(_old_post) => {
            info!("update post {}", post.id);
            update_post(&post)?;
            response.status_code(StatusCode::OK);
        }
        None => {
            info!("add post {}", post.id);
            add_post(&post)?;
            response.status_code(StatusCode::CREATED);
        }
    }
    Ok(())
}

#[handler]
async fn delete_post(
    req: &mut Request,
    response: &mut Response,
    depot: &mut Depot,
) -> ServiceResult<()> {
    info!("delete post");
    let current_user_id = get_current_user_id(depot)?;
    let repo_id = get_req_path(req, "repo_id")?;
    check_repo_owner(&repo_id, current_user_id)?;

    let post_id = get_req_path(req, "post_id")?;
    let post = get_post_by_id(&post_id)?;
    let Some(post) = post else {
        return Err(ServiceError::NotFound("post not found".to_owned()));
    };

    info!("do delete post {post_id}");
    erase_post(&post.id)?;
    response.status_code(StatusCode::NO_CONTENT);
    Ok(())
}

fn check_repo_owner(repo_id: &str, current_user_id: &str) -> ServiceResult<()> {
    let repo = get_repo_by_id(repo_id)?;
    let Some(repo) = repo else {
        return Err(ServiceError::NotFound("repo not found".to_owned()));
    };
    if repo.owner != *current_user_id {
        return Err(ServiceError::Forbidden("forbidden".to_owned()));
    }
    Ok(())
}

fn check_owner_or_subscribe(repo_id: &str, current_user_id: &str) -> ServiceResult<()> {
    match (
        check_subscribe(&current_user_id, &repo_id)?,
        check_repo_owner(&repo_id, current_user_id),
    ) {
        (true, _) | (false, Ok(())) => Ok(()),
        (_, Err(e)) => Err(e),
    }
}
