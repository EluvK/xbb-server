use salvo::{Depot, Request};

use crate::{
    error::{ServiceError, ServiceResult},
    model::{repo::get_repo_by_id, subscribe::check_subscribe},
};

pub const SESSION_USER_ID: &str = "current_user_id";

pub fn get_current_user_id(depot: &mut Depot) -> ServiceResult<&String> {
    depot
        .get::<String>(SESSION_USER_ID)
        .map_err(|err| ServiceError::InternalServerError(format!("{err:?}")))
}

pub fn get_req_path(req: &mut Request, key: &str) -> ServiceResult<String> {
    req.params()
        .get(key)
        .map(|v| v.to_string())
        .ok_or(ServiceError::InternalServerError(format!(
            "param {key} not found"
        )))
}

pub fn check_repo_owner(repo_id: &str, current_user_id: &str) -> ServiceResult<()> {
    let repo = get_repo_by_id(repo_id)?;
    let Some(repo) = repo else {
        return Err(ServiceError::NotFound("repo not found".to_owned()));
    };
    if repo.owner != *current_user_id {
        return Err(ServiceError::Forbidden("forbidden".to_owned()));
    }
    Ok(())
}

pub fn check_owner_or_subscribe(repo_id: &str, current_user_id: &str) -> ServiceResult<()> {
    match (
        check_subscribe(current_user_id, repo_id)?,
        check_repo_owner(repo_id, current_user_id),
    ) {
        (true, _) | (false, Ok(())) => Ok(()),
        (_, Err(e)) => Err(e),
    }
}
