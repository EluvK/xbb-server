use salvo::{
    basic_auth::{BasicAuth, BasicAuthValidator},
    handler,
    http::StatusCode,
    Depot, Request, Response, Router,
};
use tracing::info;

use crate::{
    error::{ServiceError, ServiceResult},
    model::user::{
        add_user, get_user_by_name, OpenApiGetUserResponse, OpenApiNewUserRequest,
        OpenApiValidateUserResponse, User,
    },
    router::utils::SESSION_USER_ID,
};

use super::utils::{get_current_user_id, get_req_path};

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
    let auth_router = Router::new().push(Router::with_path("<name>").get(get_user));
    Router::new()
        .push(non_auth_router)
        .push(Router::with_hoop(BasicAuth::new(UserValidator)).push(auth_router))
}

// #[handler]
// async fn new_user(request: &mut Request, response: &mut Response) -> ServiceResult<()> {
//     let req = request.parse_body::<OpenApiNewUserRequest>().await?;
//     print!("new user req: {:?}", req);
//     info!("new user {req:?}");
//     if let Ok(Some(_)) = get_user_by_name(&req.name) {
//         return Err(ServiceError::Conflict("user already exists".to_string()));
//     }
//     let user = User::new(req.name, req.password);
//     add_user(&user)?;
//     response.status_code(StatusCode::CREATED);
//     Ok(())
// }

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
async fn validate_login(request: &mut Request, response: &mut Response) -> ServiceResult<()> {
    let req = request.parse_body::<OpenApiNewUserRequest>().await?;
    match get_user_by_name(&req.name)? {
        Some(exist_user) => {
            if req.password == exist_user.password {
                response.status_code(StatusCode::OK);
            } else {
                return Err(ServiceError::Unauthorized(
                    "password not correct".to_string(),
                ));
            }
        }
        None => {
            let user = User::new(req.name, req.password);
            add_user(&user)?;
            response.status_code(StatusCode::CREATED);
        }
    };
    response.render("");
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
