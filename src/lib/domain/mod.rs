// src/lib/domain/mod.rs

// contains all the app data types and any associated methods

// dependencies
use askama::Template;
use sqlx::types::time::Date;
use sqlx::FromRow;

// struct type to represent a blog post
#[derive(FromRow, Debug, Clone)]
pub struct Post {
    pub post_title: String,
    pub post_date: Date,
    pub post_body: String,
}

// struct type to represent a list of posts on the index page
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub index_title: String,
    pub index_links: &'a Vec<String>,
}

// struct type to represent a blog post template
#[derive(Debug, Template)]
#[template(path = "posts.html")]
pub struct PostTemplate<'a> {
    pub post_title: &'a str,
    pub post_date: String,
    pub post_body: &'a str,
}
