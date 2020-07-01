use crate::entity::user::{User, UserID};
use super::gateway::repository::user_repository::UserRepository;

pub struct UserOperation<U: UserRepository> {
    pub ur: U
}

impl<U: UserRepository> UserOperation<U> {
    pub fn create(&self) -> Option<User> {
        self.ur.create()
    }
    pub fn find(&self, id: UserID) -> Option<User> {
        self.ur.find(id)
    }
}
