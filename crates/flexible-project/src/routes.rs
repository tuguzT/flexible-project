//! Routes of the Flexible Project server.

use actix_web::{get, post, web, Responder};
use fp_data::model::UserData;
use fp_data::repository::ops::{ReadAll, Save};
use tokio::sync::RwLock;

use crate::MockUserRepository;

/// Get all users of the Flexible Project system.
#[get("/users")]
pub async fn all_users(repository: web::Data<RwLock<MockUserRepository>>) -> impl Responder {
    let repository = repository.read().await;
    let all_users = repository.read_all().await;
    web::Json(all_users)
}

/// Save user data in the Flexible Project system.
#[post("/users")]
pub async fn save_user(
    user: web::Json<UserData>,
    repository: web::Data<RwLock<MockUserRepository>>,
) -> impl Responder {
    let mut repository = repository.write().await;
    let user = repository.save(user.into_inner()).await;
    web::Json(user)
}
