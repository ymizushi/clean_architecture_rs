use crate::adapter::repository::user_dao::TestUserRepository;

use crate::usecase::user_creation::UserCreation;

struct UserController {
}

impl UserController {
    fn render(&self) {
        let user_creation =  UserCreation { ur: TestUserRepository {} };
        user_creation.create();
    }
}

