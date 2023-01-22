use super::*;

impl Database {
    pub async fn insert_todo(&self, todo: Todo) -> Result<()> {
        let Todo {
            id,
            title,
            note,
            due_to,
            done,
            created_at,
            updated_at,
            deleted_at,
        } = todo.clone();
        sqlx::query("INSERT INTO `todos` (`id`, `title`, `note`, `due_to`, `done`, `created_at`, `updated_at`) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(id)
            .bind(title)
            .bind(note)
            .bind(due_to)
            .bind(done)
            .bind(created_at)
            .bind(updated_at)
            .bind(deleted_at)
            .execute(&self.pool)
            .await
            .with_context(|| format!("Failed to INSERT a todo {:?}", todo))?;
        Ok(())
    }

    pub async fn insert_partial_todo(
        &self,
        title: &str,
        due_to: TimeStamp,
        note: Option<&str>,
    ) -> Result<()> {
        let note = note.unwrap_or_default();
        sqlx::query("INSERT INTO `todos` (`title`, `note`, `due_to`) VALUES (?, ?, ?)")
            .bind(title)
            .bind(note)
            .bind(due_to)
            .execute(&self.pool)
            .await
            .with_context(|| {
                format!(
                    "Failed to INSERT a todo with title=`{}`, due_to=`{}`, note=`{}`",
                    title, due_to, note
                )
            })?;
        Ok(())
    }
}
