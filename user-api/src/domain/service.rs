use crate::domain::user::User;

pub trait UserService {
    fn validate(&self, user: &User) -> bool;
}
