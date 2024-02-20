use juniper::{graphql_object, EmptySubscription, RootNode};
use sqlx::PgPool;

use crate::task::{TaskMutation, TaskQuery};

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn tasks() -> TaskQuery {
        TaskQuery
    }
}

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    fn tasks() -> TaskMutation {
        TaskMutation
    }
}

pub struct Context {
    pub pool: PgPool,
}

impl juniper::Context for Context {}

pub fn schema() -> RootNode<'static, Query, Mutation, EmptySubscription<Context>> {
    RootNode::new(Query, Mutation, EmptySubscription::new())
}
