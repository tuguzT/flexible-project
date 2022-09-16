//! Routes of the Flexible Project server.

use actix_web::{get, post, web, Responder};
use fp_data::model::{IdData, UserData};
use fp_data::repository::ops::{ReadAll, ReadById, Save};
use tokio::sync::RwLock;

use crate::MockUserRepository;

/// Get all users of the Flexible Project system.
#[get("/users")]
pub async fn all_users(repository: web::Data<RwLock<MockUserRepository>>) -> impl Responder {
    let repository = repository.read().await;
    let all_users = repository.read_all().await;
    web::Json(all_users)
}

/// Get user by its identifier of the Flexible Project system.
#[get("/users/{id}")]
pub async fn user_by_id(
    id: web::Path<IdData<UserData>>,
    repository: web::Data<RwLock<MockUserRepository>>,
) -> impl Responder {
    let repository = repository.read().await;
    let id = id.into_inner();
    let user = repository.read_by_id(id).await;
    web::Json(user)
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
