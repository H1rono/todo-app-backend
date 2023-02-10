#[cfg(test)]
mod model {
    use anyhow::Result;

    use todo_app_backend::model::*;

    #[sqlx::test(migrations = "db/migrations")]
    async fn fetch_test(pool: sqlx::MySqlPool) -> Result<()> {
        let db = Database::new(pool);
        let todos = db.fetch_all_todos().await?;
        assert_eq!(todos.len(), 0);
        Ok(())
    }
}
