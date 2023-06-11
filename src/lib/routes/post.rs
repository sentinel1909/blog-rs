// src/lib/routes/post.rs

// dependencies
use crate::domain::{Post, PostTemplate};
use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use std::sync::Arc;

// posts route, displays the blog posts as links
pub async fn post(
    Path(query_title): Path<String>,
    State(state): State<Arc<Vec<Post>>>,
) -> impl IntoResponse {
    let mut template = PostTemplate {
        post_title: "none",
        post_date: "none".to_string(),
        post_body: "none",
    };

    for i in 0..state.len() {
        if query_title == state[i].post_title {
            // We found one so mutate the template variable and
            // populate it with the post that the user requested
            template = PostTemplate {
                post_title: &state[i].post_title,
                post_date: state[i].post_date.to_string(),
                post_body: &state[i].post_body,
            };
            break;
        } else {
            continue;
        }
    }

    if template.post_title == "none" {
        return (StatusCode::NOT_FOUND, "404 not found").into_response();
    }

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "try again later").into_response(),
    }
}
