use super::*;

impl Database {
    pub async fn delete_todo(&self, id: u32) -> Result<()> {
        sqlx::query("DELETE FROM `todos` WHERE `id` = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .with_context(|| format!("Failed to DELETE todo where id={}", id))?;
        Ok(())
    }
}
