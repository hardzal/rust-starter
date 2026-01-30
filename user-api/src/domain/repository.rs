use crate::domain::entity::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: User) -> anyhow::Result<()>;
    async fn find_all(&self) -> anyhow::Result<Vec<User>>;
}