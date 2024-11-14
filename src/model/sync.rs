use salvo::{writing::Json, Scribe};
use serde::{Deserialize, Serialize};

use crate::model::{
    post::{Post, PostSummary},
    repo::{OpenApiGetRepoResponse, Repo},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenApiGetRepoSyncInfoResponse {
    pub repo: OpenApiGetRepoResponse,
    pub posts_summary: Vec<PostSummary>,
}

impl OpenApiGetRepoSyncInfoResponse {
    pub fn new(repo: Repo, posts: Vec<Post>) -> Self {
        Self {
            repo: repo.into(),
            posts_summary: posts.into_iter().map(Into::into).collect(),
        }
    }
}

impl Scribe for OpenApiGetRepoSyncInfoResponse {
    fn render(self, res: &mut salvo::Response) {
        res.render(Json(&self));
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenApiSubscribeLinkRequest {
    pub link: String,
}
