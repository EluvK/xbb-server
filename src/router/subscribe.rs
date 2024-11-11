use salvo::{handler, Depot, Request, Router};

use crate::{
    error::{ServiceError, ServiceResult},
    model::{
        repo::{get_repo_by_id, OpenApiGetRepoResponse},
        sync::OpenApiSubscribeLinkRequest,
    },
    router::utils::get_current_user_id,
};

pub fn router() -> Router {
    Router::with_path("subscribe").post(add_subscribe)
}

#[handler]
async fn add_subscribe(
    req: &mut Request,
    depot: &mut Depot,
) -> ServiceResult<OpenApiGetRepoResponse> {
    let current_user_id = get_current_user_id(depot)?;

    let link = req.parse_body::<OpenApiSubscribeLinkRequest>().await?.link;
    let (user_id, repo_id) = parse_link(link)?;
    match get_repo_by_id(&repo_id)? {
        Some(repo) if repo.owner != user_id => {
            Err(ServiceError::NotFound(format!("repo {repo_id} not found")))
        }
        None => Err(ServiceError::NotFound(format!("repo {repo_id} not found"))),
        Some(repo) => Ok(repo.into()),
        // todo save current user id?
    }
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
