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
        add_user, get_user_by_name, OpenApiGetUserResponse, OpenApiNewUserRequest, User,
    },
};

pub struct UserValidator;

impl BasicAuthValidator for UserValidator {
    async fn validate(&self, username: &str, password: &str, _depot: &mut Depot) -> bool {
        if let Ok(Some(user)) = get_user_by_name(username) {
            return user.password == password;
        }
        return false;
    }
}

pub fn router() -> Router {
    let non_auth_router = Router::new().post(new_user);
    let auth_router = Router::with_path("<name>").get(get_user);
    Router::new()
        .push(non_auth_router)
        .push(Router::with_hoop(BasicAuth::new(UserValidator)).push(auth_router))
}

#[handler]
async fn new_user(request: &mut Request, response: &mut Response) -> ServiceResult<()> {
    let req = request.parse_body::<OpenApiNewUserRequest>().await?;
    print!("new user req: {:?}", req);
    info!("new user {req:?}");

    if let Ok(Some(_)) = get_user_by_name(&req.name) {
        return Err(ServiceError::Conflict("user already exists".to_string()));
    }
    let user = User::new(req.name, req.password);
    add_user(&user)?;

    response.status_code(StatusCode::CREATED);
    Ok(())
}

#[handler]
async fn get_user(request: &mut Request) -> ServiceResult<OpenApiGetUserResponse> {
    let name = request.params().get("name").unwrap();
    let user =
        get_user_by_name(name)?.ok_or(ServiceError::NotFound("user not found".to_string()))?;
    Ok(OpenApiGetUserResponse {
        id: user.id,
        name: user.name,
        avatar_url: user.avatar_url,
    })
}
