use crate::entity::user::{User, UserID};

pub trait UserRepository {
    fn find(&self, id: UserID) -> Option<User>;
    fn create(&self) -> Option<User>;
    fn update(&self, name: String) -> bool;
}
