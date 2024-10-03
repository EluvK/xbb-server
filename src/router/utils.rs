use salvo::{Depot, Request};

use crate::error::{ServiceError, ServiceResult};

pub fn get_current_user_id(depot: &mut Depot) -> ServiceResult<&String> {
    depot
        .get::<String>("current_user_id")
        .map_err(|err| ServiceError::InternalServerError(format!("{err:?}")))
}

pub fn get_req_path(req: &mut Request, key: &str) -> ServiceResult<String> {
    req.params()
        .get(key)
        .map(|v| v.to_string())
        .ok_or(ServiceError::InternalServerError(format!("param {key} not found")))
}