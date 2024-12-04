use salvo::{handler, http::StatusCode, Depot, Request, Response, Router};
use tracing::info;

use crate::{
    error::{ServiceError, ServiceResult},
    model::{
        repo::{get_repo_by_id, OpenApiGetRepoResponse, OpenApiListRepoResponse},
        subscribe::{add_subscribe, check_subscribe, delete_subscribe, fetch_subscribe},
        sync::OpenApiSubscribeLinkRequest,
    },
    router::utils::get_current_user_id,
};

pub fn router() -> Router {
    Router::new()
        .post(new_subscribe)
        .get(list_subscribe)
        .delete(remove_subscribe)
}

#[handler]
async fn new_subscribe(
    req: &mut Request,
    depot: &mut Depot,
) -> ServiceResult<OpenApiGetRepoResponse> {
    let current_user_id = get_current_user_id(depot)?;
    let link = req.parse_body::<OpenApiSubscribeLinkRequest>().await?.link;
    let (user_id, repo_id) = parse_link(link)?;
    if *current_user_id == user_id {
        return Err(ServiceError::BadRequest(
            "should not subscribe self".to_owned(),
        ));
    }

    match get_repo_by_id(&repo_id)? {
        Some(repo) if repo.owner != user_id => {
            Err(ServiceError::NotFound(format!("repo {repo_id} not found")))
        }
        None => Err(ServiceError::NotFound(format!("repo {repo_id} not found"))),
        Some(repo) => {
            if !check_subscribe(current_user_id, &repo_id)? {
                info!(
                    "add subscribe: user_id={}, repo_id={}",
                    current_user_id, repo_id
                );
                add_subscribe(current_user_id, &repo_id)?;
            }
            return Ok(repo.into());
        }
    }
}

#[handler]
async fn list_subscribe(depot: &mut Depot) -> ServiceResult<OpenApiListRepoResponse> {
    let current_user_id = get_current_user_id(depot)?;
    let repo_ids = fetch_subscribe(current_user_id)?;
    let mut repos = Vec::new();
    for repo_id in repo_ids {
        if let Some(repo) = get_repo_by_id(&repo_id)? {
            repos.push(repo);
        }
    }
    Ok(OpenApiListRepoResponse(
        repos.into_iter().map(|repo| repo.into()).collect(),
    ))
}

#[handler]
async fn remove_subscribe(
    req: &mut Request,
    depot: &mut Depot,
    response: &mut Response,
) -> ServiceResult<()> {
    let current_user_id = get_current_user_id(depot)?;
    let Some(repo_id) = req.query("repo") else {
        return Err(ServiceError::BadRequest(
            "not found query param `repo`".to_owned(),
        ));
    };
    if !check_subscribe(current_user_id, repo_id)? {
        return Err(ServiceError::NotFound(
            "not found subscribe info".to_owned(),
        ));
    }
    info!(
        "delete subscribe: user_id={}, repo_id={}",
        current_user_id, repo_id
    );
    delete_subscribe(current_user_id, repo_id)?;
    response.status_code(StatusCode::NO_CONTENT);
    Ok(())
}

fn parse_link(link: String) -> ServiceResult<(String, String)> {
    let parts: Vec<&str> = link.split("://").collect();
    if parts.len() != 2 {
        return Err(ServiceError::BadRequest("link format error".to_owned()));
    }
    let parts: Vec<&str> = parts[1].split('/').collect();
    if parts.len() != 2 {
        return Err(ServiceError::BadRequest("link format error".to_owned()));
    }
    Ok((parts[0].to_owned(), parts[1].to_owned()))
}
