use salvo::{async_trait, writing::Text, Depot, Request, Response, Writer};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("400, Bad Request")]
    BadRequest(#[from] salvo::http::ParseError),
    #[error("401, Unauthorized")]
    Unauthorized,
    #[error("403, Forbidden")]
    Forbidden,
    #[error("404, Not Found, {0}")]
    NotFound(String),
    #[error("409, Conflict, {0}")]
    Conflict(String),

    #[error("500, Internal Server Error")]
    InternalServerError(String),
}

pub type ServiceResult<T> = Result<T, ServiceError>;

#[async_trait]
impl Writer for ServiceError {
    async fn write(mut self, _req: &mut Request, depot: &mut Depot, res: &mut Response) {
        match self {
            ServiceError::BadRequest(err) => {
                res.status_code(salvo::http::StatusCode::BAD_REQUEST);
                res.render(Text::Json(format!("{{\"error\": \"{}\"}}", err)));
            }
            ServiceError::Unauthorized => {
                res.status_code(salvo::http::StatusCode::UNAUTHORIZED);
                res.render(Text::Plain("401, Unauthorized"));
            }
            ServiceError::Forbidden => {
                res.status_code(salvo::http::StatusCode::FORBIDDEN);
                res.render(Text::Plain("403, Forbidden"));
            }
            ServiceError::NotFound(err) => {
                res.status_code(salvo::http::StatusCode::NOT_FOUND);
                res.render(Text::Plain(format!("404, Not Found, {}", err)));
            }
            ServiceError::Conflict(err) => {
                res.status_code(salvo::http::StatusCode::CONFLICT);
                res.render(Text::Plain(format!("409, Conflict, {}", err)));
            }
            ServiceError::InternalServerError(err) => {
                res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
                res.render(Text::Plain(format!("500, Internal Server Error, {}", err)));
            }
        }
    }
}

impl From<rusqlite::Error> for ServiceError {
    fn from(err: rusqlite::Error) -> Self {
        ServiceError::InternalServerError(err.to_string())
    }
}
