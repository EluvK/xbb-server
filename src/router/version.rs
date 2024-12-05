use salvo::{handler, writing::Json, Depot, Router, Scribe};
use serde::{Deserialize, Serialize};

use crate::error::{ServiceError, ServiceResult};

pub fn router() -> Router {
    Router::new().get(current_version)
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenApiLastVersionResp {
    version: String,
}

impl Scribe for OpenApiLastVersionResp {
    fn render(self, res: &mut salvo::Response) {
        res.render(Json(&self));
    }
}

#[handler]
async fn current_version(depot: &mut Depot) -> ServiceResult<OpenApiLastVersionResp> {
    let version = depot
        .get::<String>("ClientVersion")
        .map_err(|_err| ServiceError::InternalServerError("no client version".to_owned()))?
        .clone();

    Ok(OpenApiLastVersionResp { version })
}
