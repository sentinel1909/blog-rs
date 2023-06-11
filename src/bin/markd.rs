// src/bin/markd.rs

use sqlx::postgres::PgPoolOptions;
use std::env;
use std::fs::File;
use std::io::Read;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let args: Vec<String> = env::args().collect();

    let inserter;

    match File::open(&args[2]) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            inserter = content;
        }
        Err(_error) => {
            panic!("could not insert into Postgres")
        }
    }

    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect("postgres://postgres:postgres@localhost:16004/postgres")
        .await
        .expect("couldn't create pool");

    let _row: (i64,) = sqlx::query_as(
        "INSERT INTO blogposts (post_title, post_body) VALUES ($1, $2) RETURNING post_id",
    )
    .bind(&args[1])
    .bind(inserter)
    .fetch_one(&pool)
    .await?;

    Ok(())
}
