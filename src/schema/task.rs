use juniper::{graphql_object, FieldError, FieldResult, GraphQLError, GraphQLInputObject};
use uuid::Uuid;

use crate::model::task::Task;

use super::context::Context;

#[graphql_object(context = Context)]
impl Task {
    fn id(&self) -> String {
        self.id().to_string()
    }

    fn title(&self) -> &str {
        self.title()
    }

    fn desciption(&self) -> Option<&str> {
        self.description().as_deref()
    }

    async fn parent(&self, context: &Context) -> Option<Task> {
        self.parent(&context.pool).await
    }

    async fn children(&self, context: &Context) -> Vec<Task> {
        self.children(&context.pool).await
    }
}

#[derive(GraphQLInputObject)]
struct GetOneTaskInput {
    id: String,
}

pub struct TaskQuery;

#[graphql_object(context = Context)]
impl TaskQuery {
    async fn roots(context: &Context) -> Vec<Task> {
        Task::get_roots(&context.pool).await
    }

    async fn one(context: &Context, input: GetOneTaskInput) -> FieldResult<Task> {
        match Task::get_one(&context.pool, Uuid::parse_str(&input.id).unwrap()).await {
            Some(task) => Ok(task),
            None => Err(format!("Could not find task with id {}", input.id).into()),
        }
    }
}

#[derive(GraphQLInputObject)]
struct CreateTaskInput {
    title: String,
    description: Option<String>,
    parent_id: Option<String>,
}

#[derive(GraphQLInputObject)]
struct DeleteTaskInput {
    id: String,
}

pub struct TaskMutation;

#[graphql_object(context = Context)]
impl TaskMutation {
    async fn create(context: &Context, input: CreateTaskInput) -> Task {
        Task::create(
            &context.pool,
            input.title,
            input.description,
            input
                .parent_id
                .as_deref()
                .map(|id| Uuid::parse_str(id).unwrap()),
        )
        .await
    }

    async fn delete(context: &Context, input: DeleteTaskInput) -> bool {
        if let Some(task) = Task::get_one(&context.pool, Uuid::parse_str(&input.id).unwrap()).await
        {
            task.delete(&context.pool).await;
            true
        } else {
            false
        }
    }
}
