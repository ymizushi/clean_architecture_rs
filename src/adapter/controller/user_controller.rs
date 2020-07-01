use crate::adapter::repository::user_dao::TestUserRepository;

use crate::usecase::user_creation::UserOperation;

use actix_web::{get, web, Responder};

#[get("/users/{id}")]
pub async fn show(info: web::Path<(i32)>) -> impl Responder {
    let user_creation =  UserOperation { ur: TestUserRepository {} };

    if let Some(user) = user_creation.create() {
        format!("Hello {}", user.name)
    } else {
        format!("bye")
    }
    

}
