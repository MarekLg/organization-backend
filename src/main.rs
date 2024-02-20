use std::env;

use juniper_warp::{make_graphql_filter, playground_filter};
use schema::{schema, Context};
use sqlx::PgPool;
use warp::Filter;

mod schema;
mod task;

#[tokio::main]
async fn main() {
    let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let routes = warp::post()
        .and(warp::path("graphql"))
        .and(make_graphql_filter(
            schema(),
            warp::any()
                .map(move || Context { pool: pool.clone() })
                .boxed(),
        ))
        .or(warp::get()
            .and(warp::path("playground"))
            .and(playground_filter("/graphql", None)));

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
