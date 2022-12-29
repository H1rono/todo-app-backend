use sqlx::types::time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub note: String,
    pub due_to: OffsetDateTime,
    pub done: i8,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}
