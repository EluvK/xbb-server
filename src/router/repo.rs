use salvo::{handler, http::StatusCode, Depot, Request, Response, Router};
use tracing::info;

use crate::{
    error::{ServiceError, ServiceResult},
    model::{
        post::list_posts_by_repo_id,
        repo::{
            add_repo, get_repo_by_id, list_repos_by_owner_id, update_repo, OpenApiGetRepoResponse,
            OpenApiListRepoResponse, OpenApiPushRepoRequest, Repo, RepoStatus,
        },
        sync::OpenApiGetRepoSyncInfoResponse,
    },
    router::utils::{get_current_user_id, get_req_path},
};

pub fn router() -> Router {
    Router::new()
        .get(list_repo)
        .post(push_repo)
        .push(
            Router::with_path("<repo_id>")
                .get(get_repo)
                .delete(delete_repo),
        )
        .push(Router::with_path("<repo_id>/summary").get(repo_summary))
}

#[handler]
async fn list_repo(depot: &mut Depot) -> ServiceResult<OpenApiListRepoResponse> {
    info!("list repo");
    let current_user_id = get_current_user_id(depot)?;
    let repos = list_repos_by_owner_id(current_user_id)?;
    Ok(OpenApiListRepoResponse(
        repos.into_iter().map(|repo| repo.into()).collect(),
    ))
}

#[handler]
async fn push_repo(
    request: &mut Request,
    response: &mut Response,
    depot: &mut Depot,
) -> ServiceResult<OpenApiGetRepoResponse> {
    info!("push repo");
    let current_user_id = get_current_user_id(depot)?;
    let repo: Repo = request.parse_body::<OpenApiPushRepoRequest>().await?.into();
    info!("repo: {:?}", repo);
    if *current_user_id != repo.owner {
        return Err(ServiceError::Forbidden("auth failed".to_owned()));
    }
    match get_repo_by_id(&repo.id)? {
        Some(_old_repo) => {
            if *current_user_id != _old_repo.owner {
                return Err(ServiceError::Forbidden("auth failed".to_owned()));
            }
            info!("update repo");
            update_repo(&repo)?;
            response.status_code(StatusCode::OK);
        }
        None => {
            info!("add repo");
            add_repo(&repo)?;
            response.status_code(StatusCode::CREATED);
        }
    }
    Ok(repo.into())
}

#[handler]
async fn get_repo(req: &mut Request, depot: &mut Depot) -> ServiceResult<OpenApiGetRepoResponse> {
    info!("get repo");
    let repo_id = get_req_path(req, "repo_id")?;
    let current_user_id = get_current_user_id(depot)?;
    let repos = list_repos_by_owner_id(current_user_id)?;
    repos
        .into_iter()
        .find(|repo| repo.id == *repo_id)
        .ok_or(ServiceError::NotFound("repo not found".to_string()))
        .map(|repo| repo.into())
}
#[handler]
async fn delete_repo(
    req: &mut Request,
    depot: &mut Depot,
    response: &mut Response,
) -> ServiceResult<()> {
    info!("get repo");
    let repo_id = get_req_path(req, "repo_id")?;
    let current_user_id = get_current_user_id(depot)?;
    let Some(mut old_repo) = get_repo_by_id(&repo_id)? else {
        return Err(ServiceError::NotFound(format!("{repo_id} not found")));
    };
    if *current_user_id != old_repo.owner {
        return Err(ServiceError::NotFound(format!("{repo_id} not found")));
    }
    old_repo.status = RepoStatus::Deleted;
    update_repo(&old_repo)?;
    response.status_code(StatusCode::NO_CONTENT);
    Ok(())
}

#[handler]
async fn repo_summary(
    request: &mut Request,
    _response: &mut Response,
    depot: &mut Depot,
) -> ServiceResult<OpenApiGetRepoSyncInfoResponse> {
    info!("get repo info");
    let _current_user_id = get_current_user_id(depot)?;
    let repo_id = get_req_path(request, "repo_id")?;
    let repo = get_repo_by_id(&repo_id)?;
    // todo check permission maybe?
    match repo {
        Some(repo) => {
            let posts = list_posts_by_repo_id(&repo_id)?;
            Ok(OpenApiGetRepoSyncInfoResponse::new(repo, posts))
        }
        None => Err(ServiceError::NotFound(format!("repo {repo_id} not found"))),
    }
}
