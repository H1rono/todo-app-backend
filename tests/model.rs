#[cfg(test)]
mod model_test {
    use anyhow::{anyhow, Result};

    use todo_app_backend::model::*;

    const SAMPLE_PARTIAL_TODOS: [(&str, &str, &str, bool); 2] = [
        ("todo 1", "some note", "2023-01-01T22:06:00Z", false),
        ("todo 2", "", "2023-01-01T01:00:00Z", true),
    ];

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn insert_test(pool: sqlx::MySqlPool) -> Result<()> {
        let db = Database::new(pool);
        let mut i = 0;
        for p_todo in SAMPLE_PARTIAL_TODOS {
            i = db.insert_partial_todo(&p_todo.try_into().unwrap()).await?;
        }
        if i as usize != SAMPLE_PARTIAL_TODOS.len() {
            return Err(anyhow!(
                "id is mismatch: expected {i}, got {}",
                SAMPLE_PARTIAL_TODOS.len()
            ));
        }
        Ok(())
    }

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn fetch_test(pool: sqlx::MySqlPool) -> Result<()> {
        let db = Database::new(pool);
        let todos = db.fetch_all_todos().await?;
        assert_eq!(todos.len(), 0);
        Ok(())
    }
}
