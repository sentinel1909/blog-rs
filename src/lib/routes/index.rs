// src/lib/routes/index.rs

// dependencies
use crate::domain::{IndexTemplate, Post};
use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use std::sync::Arc;

// index route, displays all the blog post titles
pub async fn index(State(state): State<Arc<Vec<Post>>>) -> impl IntoResponse {
    let mut plinks: Vec<String> = Vec::new();

    for i in 0..state.len() {
        plinks.push(state[i].post_title.clone());
    }

    let template = IndexTemplate {
        index_title: String::from("Test Blog"),
        index_links: &plinks,
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error {}", err),
        )
            .into_response(),
    }
}
