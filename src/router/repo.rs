use salvo::{handler, Depot, Request, Response, Router};
use tracing::info;

use crate::{
    error::{ServiceError, ServiceResult},
    model::repo::{
        add_repo, get_repo_by_name, list_repos_by_owner_id, OpenApiGetRepoResponse,
        OpenApiListRepoResponse, OpenApiNewRepoRequest, Repo,
    },
    router::utils::{get_current_user_id, get_req_path},
};

pub fn router() -> Router {
    Router::new()
        .get(list_repo)
        .post(new_repo)
        .push(Router::with_path("<repo_id>").get(get_repo))
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
async fn new_repo(
    request: &mut Request,
    response: &mut Response,
    depot: &mut Depot,
) -> ServiceResult<OpenApiGetRepoResponse> {
    info!("new repo");
    let current_user_id = get_current_user_id(depot)?;
    let req = request.parse_body::<OpenApiNewRepoRequest>().await?;
    info!("new repo {req:?}");

    if let Some(_repo) = get_repo_by_name(req.name.as_str())? {
        return Err(ServiceError::Conflict("repo already exists".to_string()));
    }

    // insert new repo
    let repo = Repo::new(req.name, current_user_id.clone(), req.description);
    add_repo(&repo)?;

    response.status_code(salvo::http::StatusCode::CREATED);
    Ok(repo.into())
}
