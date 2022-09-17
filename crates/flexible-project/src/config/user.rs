use actix_web::{web, HttpResponse, Responder};
use fp_data::model::{IdData, UserData};
use fp_data::repository::user::UserRepository;

use crate::RwData;

/// Provides configuration for user endpoints of the Flexible Project system.
///
/// Creates a factory for [`actix-web`](actix_web) [application](actix_web::App) configuration.
pub fn config<R>(repository: RwData<R>) -> impl Fn(&mut web::ServiceConfig) + Clone
where
    R: UserRepository + Send + Sync + 'static,
{
    move |cfg| {
        let repository = repository.clone();
        let scope = web::scope("/users")
            .route("", web::get().to(all_users::<R>))
            .route("/{id}", web::get().to(user_by_id::<R>))
            .route("", web::post().to(save_user::<R>))
            .route("/{id}", web::delete().to(delete_user::<R>));
        cfg.service(scope).app_data(repository);
    }
}

/// Get all users of the Flexible Project system.
async fn all_users<R>(repository: RwData<R>) -> impl Responder
where
    R: UserRepository + Send + Sync,
{
    let repository = repository.read().await;
    let all_users = repository.read_all().await;
    web::Json(all_users)
}

/// Get user by its identifier of the Flexible Project system.
async fn user_by_id<R>(id: web::Path<IdData<UserData>>, repository: RwData<R>) -> impl Responder
where
    R: UserRepository + Send + Sync,
{
    let repository = repository.read().await;
    let user = repository.read_by_id(id.into_inner()).await;
    web::Json(user)
}

/// Save user data in the Flexible Project system.
async fn save_user<R>(user: web::Json<UserData>, repository: RwData<R>) -> impl Responder
where
    R: UserRepository + Send,
{
    let mut repository = repository.write().await;
    let user = repository.save(user.into_inner()).await;
    web::Json(user)
}

/// Delete user by its identifier from the Flexible Project system.
async fn delete_user<R>(id: web::Path<IdData<UserData>>, repository: RwData<R>) -> impl Responder
where
    R: UserRepository + Send,
{
    let mut repository = repository.write().await;
    repository.delete_by_id(id.into_inner()).await;
    HttpResponse::Ok()
}
