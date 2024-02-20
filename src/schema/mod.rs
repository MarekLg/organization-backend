use juniper::{graphql_object, EmptySubscription, RootNode};

pub use context::Context;

use self::task::{TaskMutation, TaskQuery};

mod context;
mod task;

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

pub fn schema() -> RootNode<'static, Query, Mutation, EmptySubscription<Context>> {
    RootNode::new(Query, Mutation, EmptySubscription::new())
}
