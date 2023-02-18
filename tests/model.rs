#[cfg(test)]
mod model_test {
    use anyhow::{anyhow, Result};

    use todo_app_backend::model::*;

    const SAMPLE_PARTIAL_TODOS: [(&str, &str, &str, bool); 2] = [
        ("todo 1", "some note", "2023-01-01T22:06:00Z", false),
        ("todo 2", "", "2023-01-01T01:00:00Z", true),
    ];

    fn gen_partial_todos() -> Result<Vec<PartialTodo>> {
        SAMPLE_PARTIAL_TODOS
            .into_iter()
            .map(|p_todo| p_todo.try_into())
            .collect::<Result<Vec<PartialTodo>>>()
    }

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn insert_test(pool: sqlx::MySqlPool) -> Result<()> {
        let db = Database::new(pool);
        for (i, p_todo) in gen_partial_todos()?.iter().enumerate() {
            let j = db.insert_partial_todo(p_todo).await?;
            if (i + 1) as u32 != j {
                return Err(anyhow!("mismatch in id: expected {}, got {}", i + 1, j));
            }
        }
        Ok(())
    }

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn fetch_test(pool: sqlx::MySqlPool) -> Result<()> {
        let db = Database::new(pool);
        let todos = db.fetch_all_todos().await?;
        if !todos.is_empty() {
            return Err(anyhow!(
                "there should be no todos in database, but found: {todos:?}"
            ));
        }
        let p_todos = gen_partial_todos()?;
        for p_todo in p_todos.iter() {
            db.insert_partial_todo(p_todo).await?;
        }
        // fetch_all
        let todos = db.fetch_all_todos().await?;
        if p_todos.len() != todos.len() {
            return Err(anyhow!(
                "inserted {} PartialTodo(s), but fetched {} Todo(s)",
                p_todos.len(),
                todos.len()
            ));
        }
        for (p_todo, todo) in p_todos.iter().zip(todos.into_iter()) {
            let pt: PartialTodo = todo.into();
            if p_todo != &pt {
                return Err(anyhow!("mismatch todo: {p_todo:?} and {pt:?}"));
            }
        }
        // fetch_by_id
        for (i, p_todo) in p_todos.iter().enumerate() {
            let id = (i + 1) as u32;
            let todo = db.fetch_todo_by_id(id).await?;
            let pt: PartialTodo = todo.into();
            if p_todo != &pt {
                return Err(anyhow!("mismatch todo: {p_todo:?} and {pt:?}"));
            }
        }
        match db.fetch_todo_by_id((p_todos.len() + 10) as u32).await {
            Err(DBError::RowNotFound(_)) => Ok(()),
            x => Err(anyhow!(
                "unexpected result {x:?} of fetch by the id that is out of range"
            )),
        }?;
        Ok(())
    }
}
