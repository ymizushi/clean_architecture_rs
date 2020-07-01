use crate::adapter::repository::user_dao::UserRepositoryBySQLite;

use crate::usecase::user_creation::UserOperation;

use crate::entity::user::UserID;

use actix_web::{get, web, Responder};

#[get("/users/{id}")]
pub async fn show(info: web::Path<i64>) -> impl Responder {
    let user_operation =  UserOperation { ur: UserRepositoryBySQLite {} };

    if let Some(user) = user_operation.find(UserID { id: info.into_inner() }) {
        format!("Hello {}", user.name)
    } else {
        format!("bye")
    }
    

}
