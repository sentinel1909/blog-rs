// dependencies
use axum::{routing::get, Router};
use blog_rs::domain::Post;
use blog_rs::routes::{index, post};
use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool};
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing::info;

#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    // migrate the database schema, given the empty database that Shuttle created for us
    info!("Migrating database schema...");
    pool.execute(include_str!("../../schema.sql"))
        .await
        .map_err(CustomError::new)?;

    // get all the posts from the database
    let posts = sqlx::query_as::<_, Post>("SELECT post_title, post_date, post_body FROM blogposts")
        .fetch_all(&pool)
        .await
        .unwrap();

    // put the posts into a variable and in an arc for thread-safe referencing
    let shared_state = Arc::new(posts);

    // configure a variable called router to hold all its routes, state, and static asses
    info!("Starting up the router...");
    let router = Router::new()
        .route("/", get(index))
        .route("/post/:query_title", get(post))
        .with_state(shared_state)
        .nest_service("/assets", ServeDir::new("assets"));

    // pass back the router
    Ok(router.into())
}
