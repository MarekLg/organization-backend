use sqlx::PgPool;

pub struct Context {
    pub pool: PgPool,
}

impl juniper::Context for Context {}
