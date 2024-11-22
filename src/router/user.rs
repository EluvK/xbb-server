use chrono::Utc;
use salvo::{
    basic_auth::{BasicAuth, BasicAuthValidator},
    handler,
    http::StatusCode,
    Depot, Request, Response, Router,
};

use crate::{
    error::{ServiceError, ServiceResult},
    model::user::{
        add_user, get_user_by_id, get_user_by_name, update_exist_user, OpenApiGetUserResponse,
        OpenApiNewUserRequest, OpenApiUpdateUserRequest, OpenApiValidateUserResponse, User,
    },
    router::utils::SESSION_USER_ID,
};

use super::utils::get_req_path;

pub struct UserValidator;

impl BasicAuthValidator for UserValidator {
    async fn validate(&self, username: &str, password: &str, depot: &mut Depot) -> bool {
        if let Ok(Some(user)) = get_user_by_name(username) {
            if user.password == password {
                depot.insert(SESSION_USER_ID, user.id);
                return true;
            }
        }
        false
    }
}

pub fn router() -> Router {
    let non_auth_router = Router::new()
        // .post(new_user)
        .push(Router::with_path("validate-name/<name>").get(validate_user_name))
        .push(Router::with_path("validate-login").post(validate_login));
    let auth_router = Router::new()
        .push(Router::with_path("<name>").get(get_user))
        .push(Router::with_path("<id>").put(update_user));
    Router::new()
        .push(non_auth_router)
        .push(Router::with_hoop(BasicAuth::new(UserValidator)).push(auth_router))
}

#[handler]
async fn validate_user_name(request: &mut Request) -> ServiceResult<OpenApiValidateUserResponse> {
    let name = get_req_path(request, "name")?;
    match get_user_by_name(&name)? {
        Some(_) => Ok(OpenApiValidateUserResponse { exist: true }),
        None => Ok(OpenApiValidateUserResponse { exist: false }),
    }
}

/// login if user exist, or register a new one.
#[handler]
async fn validate_login(
    request: &mut Request,
    response: &mut Response,
) -> ServiceResult<OpenApiGetUserResponse> {
    let req = request.parse_body::<OpenApiNewUserRequest>().await?;
    let user;
    match get_user_by_name(&req.name)? {
        Some(exist_user) => {
            if req.password == exist_user.password {
                response.status_code(StatusCode::OK);
                user = exist_user;
            } else {
                return Err(ServiceError::Unauthorized(
                    "password not correct".to_string(),
                ));
            }
        }
        None => {
            user = User::new(req.name, req.password);
            add_user(&user)?;
            response.status_code(StatusCode::CREATED);
        }
    };
    Ok(OpenApiGetUserResponse {
        id: user.id,
        name: user.name,
        avatar_url: user.avatar_url,
    })
}

#[handler]
async fn update_user(request: &mut Request) -> ServiceResult<()> {
    let id = get_req_path(request, "id")?;
    let mut user =
        get_user_by_id(&id)?.ok_or(ServiceError::NotFound("user not found".to_string()))?;
    let req = request.parse_body::<OpenApiUpdateUserRequest>().await?;
    if get_user_by_name(&req.name)?.is_some() {
        return Err(ServiceError::Conflict(format!("name {:?} exist", req.name)));
    }
    user = User {
        name: req.name,
        password: req.password,
        avatar_url: req.avatar_url,
        updated_at: Utc::now(),
        ..user
    };
    update_exist_user(&user)?;
    Ok(())
}

#[handler]
async fn get_user(request: &mut Request) -> ServiceResult<OpenApiGetUserResponse> {
    let name = get_req_path(request, "name")?;
    let user =
        get_user_by_name(&name)?.ok_or(ServiceError::NotFound("user not found".to_string()))?;
    Ok(OpenApiGetUserResponse {
        id: user.id,
        name: user.name,
        avatar_url: user.avatar_url,
    })
}
