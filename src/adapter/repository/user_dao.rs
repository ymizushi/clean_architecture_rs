use crate::entity::user::{User, UserID};
use crate::usecase::gateway::repository::user_repository::UserRepository;

pub struct TestUserRepository {}

impl UserRepository for TestUserRepository {
    fn find(&self, id: UserID) -> Option<User> {
        Some(
            User {
                id: UserID { id: 1 },
                name: String::from("name")
            }
        )
    }

    fn create(&self) -> Option<User> {
        None
    }

    fn update(&self, name: String) -> bool {
        true
    }
}
