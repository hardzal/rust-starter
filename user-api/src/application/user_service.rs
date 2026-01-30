use crate::domain::{entity::user::User, repository::UserRepository};

use uuid::Uuid;

#[derive(Clone)]
pub struct UserUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> UserUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_user(&self, name: String, email: String) -> anyhow::Result<()> {
        let user = User {
            id: Uuid::new_v4(),
            name,
            email,
        };

        self.repo.save(user).await
    }

    pub async fn list_users(&self) -> anyhow::Result<Vec<User>> {
        self.repo.find_all().await
    }
}
