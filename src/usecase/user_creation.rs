use crate::entity::user::User;
use super::gateway::repository::user_repository::UserRepository;

pub struct UserOperation<U: UserRepository> {
    pub ur: U
}

impl<U: UserRepository> UserOperation<U> {
    pub fn create(&self) -> Option<User> {
        self.ur.create()
    }
}
