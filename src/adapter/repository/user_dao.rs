use crate::entity::user::{User, UserID};
use crate::usecase::gateway::repository::user_repository::UserRepository;

pub struct UserRepositoryBySQLite {

}

impl UserRepository for UserRepositoryBySQLite {
    fn find(&self, id: UserID) -> Option<User> {
        Some(
            User {
                id: UserID { id: 1 },
                name: String::from("name")
            }
        )
    }

    fn create(&self) -> Option<User> {
        Some(
            User {
                id: UserID { id: 1 },
                name: String::from("name")
            }
        )
    }

    fn update(&self, name: String) -> bool {
        true
    }
}
