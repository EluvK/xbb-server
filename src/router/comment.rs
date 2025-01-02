use salvo::{handler, http::StatusCode, Depot, Request, Response, Router};
use tracing::info;

use crate::{
    error::{ServiceError, ServiceResult},
    model::comment::{
        add_comment, delete_comment_by_id, get_comment_by_id, list_comments_by_post_id,
        update_comment, Comment, OpenApiGetCommentResponse, OpenApiListCommentResponse,
        OpenApiPushCommentRequest,
    },
    router::utils::{check_owner_or_subscribe, get_current_user_id, get_req_path},
};

pub fn router() -> Router {
    Router::new().get(list_comment).post(push_comment).push(
        Router::with_path("<comment_id>")
            .get(get_comment)
            .delete(delete_comment),
    )
}

#[handler]
async fn list_comment(
    request: &mut Request,
    depot: &mut Depot,
) -> ServiceResult<OpenApiListCommentResponse> {
    let current_user_id = get_current_user_id(depot)?;
    let repo_id = get_req_path(request, "repo_id")?;
    let post_id = get_req_path(request, "post_id")?;
    check_owner_or_subscribe(&repo_id, current_user_id)?;
    info!("list comment in post {post_id}");
    let comments = list_comments_by_post_id(&post_id)?;
    // info!("list comment result: {comments:?}");
    Ok(OpenApiListCommentResponse(
        comments.into_iter().map(|comment| comment.into()).collect(),
    ))
}

#[handler]
async fn get_comment(
    request: &mut Request,
    depot: &mut Depot,
) -> ServiceResult<OpenApiGetCommentResponse> {
    let current_user_id = get_current_user_id(depot)?;
    let repo_id = get_req_path(request, "repo_id")?;
    let post_id = get_req_path(request, "post_id")?;
    let comment_id = get_req_path(request, "comment_id")?;
    check_owner_or_subscribe(&repo_id, current_user_id)?;
    let comment = get_comment_by_id(&comment_id)?;
    info!("get comment {comment:?}");
    match comment {
        Some(comment) => {
            if comment.post_id != post_id || comment.repo_id != repo_id {
                return Err(ServiceError::NotFound("comment not found".to_owned()));
            }
            Ok(comment.into())
        }
        None => Err(ServiceError::NotFound("comment not found".to_owned())),
    }
}

#[handler]
async fn push_comment(
    request: &mut Request,
    response: &mut Response,
    depot: &mut Depot,
) -> ServiceResult<()> {
    info!("push comment");
    let current_user_id = get_current_user_id(depot)?;
    let repo_id = get_req_path(request, "repo_id")?;
    let post_id = get_req_path(request, "post_id")?;
    let comment = request.parse_body::<OpenApiPushCommentRequest>().await?;
    info!("push comment: {comment:?}");
    match comment.id {
        Some(id) => {
            // update
            let comment = Comment {
                id,
                post_id,
                repo_id,
                content: comment.content,
                updated_at: chrono::Utc::now(),
                created_at: chrono::Utc::now(),
                author: current_user_id.clone(),
                parent_id: comment.parent_id,
            };
            update_comment(&comment)?;
            response.status_code(StatusCode::OK);
        }
        None => {
            // insert
            let comment = Comment {
                id: uuid::Uuid::new_v4().to_string(),
                post_id,
                repo_id,
                content: comment.content,
                updated_at: chrono::Utc::now(),
                created_at: chrono::Utc::now(),
                author: current_user_id.clone(),
                parent_id: comment.parent_id,
            };
            add_comment(&comment)?;
            response.status_code(StatusCode::CREATED);
        }
    }
    Ok(())
}

#[handler]
async fn delete_comment(
    request: &mut Request,
    response: &mut Response,
    depot: &mut Depot,
) -> ServiceResult<()> {
    info!("delete comment");
    let current_user_id = get_current_user_id(depot)?;
    let repo_id = get_req_path(request, "repo_id")?;
    let post_id = get_req_path(request, "post_id")?;
    let comment_id = get_req_path(request, "comment_id")?;
    let comment = get_comment_by_id(&comment_id)?;
    let Some(comment) = comment else {
        return Err(ServiceError::NotFound("comment not found".to_owned()));
    };
    if comment.post_id != post_id || comment.repo_id != repo_id {
        return Err(ServiceError::NotFound("comment not found".to_owned()));
    }
    if comment.author != *current_user_id {
        return Err(ServiceError::Forbidden("forbidden".to_owned()));
    }
    info!("do delete comment {comment:?}");
    delete_comment_by_id(&comment_id)?;
    response.status_code(StatusCode::NO_CONTENT);

    Ok(())
}
