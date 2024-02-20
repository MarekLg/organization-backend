use sqlx::{types::Uuid, PgPool};

pub struct Task {
    id: Uuid,
    title: String,
    description: Option<String>,
    parent_id: Option<Uuid>,
}

impl Task {
    pub async fn create(
        pool: &PgPool,
        title: String,
        description: Option<String>,
        parent_id: Option<Uuid>,
    ) -> Self {
        sqlx::query_as!(
            Self,
            r#"
			INSERT INTO task (title, description, parent_id)
				VALUES ($1, $2, $3)
			RETURNING *
			"#,
            title,
            description,
            parent_id
        )
        .fetch_one(pool)
        .await
        .unwrap()
    }

    pub async fn get_roots(pool: &PgPool) -> Vec<Self> {
        sqlx::query_as!(
            Self,
            r#"
			SELECT * FROM task
			WHERE parent_id IS NULL
			"#
        )
        .fetch_all(pool)
        .await
        .unwrap()
    }

    pub async fn get_one(pool: &PgPool, id: Uuid) -> Option<Self> {
        sqlx::query_as!(
            Self,
            r#"
			SELECT * FROM task
			WHERE id = $1
			"#,
            id
        )
        .fetch_optional(pool)
        .await
        .unwrap()
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub async fn parent(&self, pool: &PgPool) -> Option<Self> {
        if let Some(parent_id) = self.parent_id {
            Some(
                sqlx::query_as!(
                    Self,
                    r#"
					SELECT * FROM task
					WHERE id = $1
					"#,
                    parent_id
                )
                .fetch_one(pool)
                .await
                .unwrap(),
            )
        } else {
            None
        }
    }

    pub async fn children(&self, pool: &PgPool) -> Vec<Self> {
        sqlx::query_as!(
            Self,
            r#"
			SELECT * FROM task
			WHERE parent_id = $1
			"#,
            self.id
        )
        .fetch_all(pool)
        .await
        .unwrap()
    }

    pub async fn update_title(self, pool: &PgPool, title: String) -> Self {
        sqlx::query_as!(
            Self,
            r#"
			UPDATE task
				SET title = $1
			WHERE id = $2
			RETURNING *
			"#,
            title,
            self.id
        )
        .fetch_one(pool)
        .await
        .unwrap()
    }

    pub async fn update_description(self, pool: &PgPool, description: Option<String>) -> Self {
        sqlx::query_as!(
            Self,
            r#"
			UPDATE task
				SET description = $1
			WHERE id = $2
			RETURNING *
			"#,
            description,
            self.id
        )
        .fetch_one(pool)
        .await
        .unwrap()
    }

    pub async fn delete(self, pool: &PgPool) {
        sqlx::query!(
            r#"
			DELETE FROM task
			WHERE id = $1
			"#,
            self.id
        )
        .execute(pool)
        .await
        .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use super::Task;

    async fn setup_task(pool: &PgPool, title: String, description: Option<String>) -> Task {
        sqlx::query_as!(
            Task,
            r#"
			INSERT INTO task (title, description)
				VALUES ($1, $2)
			RETURNING *
			"#,
            title,
            description
        )
        .fetch_one(pool)
        .await
        .unwrap()
    }

    #[sqlx::test]
    async fn creates_task(pool: PgPool) -> sqlx::Result<()> {
        let title = String::from("title");

        let task = Task::create(&pool, title.clone(), None, None).await;

        let result = sqlx::query!("SELECT * FROM task").fetch_all(&pool).await?;

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, task.id);
        assert_eq!(result[0].title, task.title);
        assert_eq!(result[0].description, task.description);

        assert_eq!(task.title, title);
        assert_eq!(task.description, None);

        Ok(())
    }

    #[sqlx::test]
    async fn gets_all_root_tasks(pool: PgPool) -> sqlx::Result<()> {
        let tasks = vec![
            setup_task(&pool, "title1".into(), Some("description".into())).await,
            setup_task(&pool, "title2".into(), None).await,
        ];

        let result = Task::get_roots(&pool).await;
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].id, tasks[0].id);
        assert_eq!(result[0].title, tasks[0].title);
        assert_eq!(result[0].description, tasks[0].description);
        assert_eq!(result[1].id, tasks[1].id);
        assert_eq!(result[1].title, tasks[1].title);
        assert_eq!(result[1].description, tasks[1].description);

        Ok(())
    }

    #[sqlx::test]
    async fn updates_task_title(pool: PgPool) -> sqlx::Result<()> {
        let task = setup_task(&pool, "title".into(), None).await;
        let new_title = String::from("new title");

        let result = task.update_title(&pool, new_title.clone()).await;
        assert_eq!(result.title, new_title);
        assert_eq!(result.description, None);

        Ok(())
    }

    #[sqlx::test]
    async fn updates_task_description(pool: PgPool) -> sqlx::Result<()> {
        let task = setup_task(&pool, "title".into(), None).await;
        let new_description = Some(String::from("description"));

        let result = task
            .update_description(&pool, new_description.clone())
            .await;
        assert_eq!(result.title, String::from("title"));
        assert_eq!(result.description, new_description);

        Ok(())
    }

    #[sqlx::test]
    async fn deletes_task(pool: PgPool) -> sqlx::Result<()> {
        let task = setup_task(&pool, "title".into(), None).await;
        let id = task.id;

        task.delete(&pool).await;

        assert_eq!(
            sqlx::query!(
                r#"
			SELECT COUNT(*)
			FROM task
			WHERE id = $1
			"#,
                id
            )
            .fetch_one(&pool)
            .await
            .unwrap()
            .count,
            Some(0)
        );

        Ok(())
    }
}
