use salvo::{handler, http::StatusCode, Depot, Request, Response, Router};
use tracing::info;

use crate::{
    error::{ServiceError, ServiceResult},
    model::{
        post::list_posts_by_repo_id,
        repo::{add_repo, get_repo_by_id, OpenApiSyncRepoRequest, Repo},
        sync::OpenApiGetRepoSyncInfoResponse,
    },
    router::utils::{get_current_user_id, get_req_path},
};

pub fn router() -> Router {
    Router::with_path("repo/<repo_id>")
        .get(get_repo_info)
        .put(sync_repo_push)
        .post(sync_repo_pull)
}

#[handler]
async fn get_repo_info(
    req: &mut Request,
    depot: &mut Depot,
) -> ServiceResult<OpenApiGetRepoSyncInfoResponse> {
    info!("get repo info");
    let _current_user_id = get_current_user_id(depot)?;
    let repo_id = get_req_path(req, "repo_id")?;
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

#[handler]
async fn sync_repo_push(
    req: &mut Request,
    response: &mut Response,
    depot: &mut Depot,
) -> ServiceResult<()> {
    let current_user_id = get_current_user_id(depot)?;
    let repo_id = get_req_path(req, "repo_id")?;
    let repo: Repo = req.parse_body::<OpenApiSyncRepoRequest>().await?.into();
    if repo.id != repo_id {
        return Err(ServiceError::NotFound(
            "repo id not equivalent in request".to_owned(),
        ));
    }
    info!("sync repo push {repo:?}");
    match get_repo_by_id(&repo_id)? {
        None => {
            if *current_user_id != repo.owner {
                return Err(ServiceError::Forbidden("auth failed".to_owned()));
            }
            add_repo(&repo)?;
            response.status_code(StatusCode::CREATED);
        }
        Some(repo) => {}
    }
    
    Ok(())
}

#[handler]
async fn sync_repo_pull(
    req: &mut Request,
    response: &mut Response,
    depot: &mut Depot,
) -> ServiceResult<()> {
    Ok(())
}
