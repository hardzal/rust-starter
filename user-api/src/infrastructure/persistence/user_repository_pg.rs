use crate::domain::{entity::user::User, repository::UserRepository};

use async_trait::async_trait;
use sqlx::PgPool;


#[derive(Clone)]
pub struct UserRepositoryPg {
    pool: PgPool,
}

impl UserRepositoryPg {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryPg {
    async fn save(&self, user:User) -> anyhow::Result<()> {
        sqlx::query!(
            "INSERT INTO users(id, name, email) VALUES ($1, $2, $3)",
            user.id,
            user.name,
            user.email,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_all(&self) -> anyhow::Result<Vec<User>> {
        let rows = sqlx::query!(
            "SELECT id, name, email FROM users"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter().map(|row| User {
                id: row.id,
                name: row.name,
                email: row.email,
            }).collect())
    }
}
