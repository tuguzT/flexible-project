use actix_web::{web, HttpResponse, Responder};
use fp_data::data_source::user::UserDataSource;
use fp_data::model::{IdData, UserData};
use fp_data::repository::ops::{DeleteById, ReadAll, ReadById, Save};
use fp_data::repository::user::UserRepository;

use crate::RwData;

/// Configuration for user endpoints of the Flexible Project system.
pub fn config<S>(repository: RwData<UserRepository<S>>) -> impl Fn(&mut web::ServiceConfig) + Clone
where
    S: UserDataSource<Item = UserData> + Send + Sync + 'static,
{
    move |cfg| {
        let repository = repository.clone();
        let scope = web::scope("/users")
            .route("", web::get().to(all_users::<S>))
            .route("/{id}", web::get().to(user_by_id::<S>))
            .route("", web::post().to(save_user::<S>))
            .route("/{id}", web::delete().to(delete_user::<S>));
        cfg.service(scope).app_data(repository);
    }
}

/// Get all users of the Flexible Project system.
async fn all_users<S>(repository: RwData<UserRepository<S>>) -> impl Responder
where
    S: UserDataSource<Item = UserData> + Send + Sync + 'static,
{
    let repository = repository.read().await;
    let all_users = repository.read_all().await;
    web::Json(all_users)
}

/// Get user by its identifier of the Flexible Project system.
async fn user_by_id<S>(
    id: web::Path<IdData<UserData>>,
    repository: RwData<UserRepository<S>>,
) -> impl Responder
where
    S: UserDataSource<Item = UserData> + Send + Sync + 'static,
{
    let repository = repository.read().await;
    let user = repository.read_by_id(id.into_inner()).await;
    web::Json(user)
}

/// Save user data in the Flexible Project system.
async fn save_user<S>(
    user: web::Json<UserData>,
    repository: RwData<UserRepository<S>>,
) -> impl Responder
where
    S: UserDataSource<Item = UserData> + Send + 'static,
{
    let mut repository = repository.write().await;
    let user = repository.save(user.into_inner()).await;
    web::Json(user)
}

/// Delete user by its identifier from the Flexible Project system.
async fn delete_user<S>(
    id: web::Path<IdData<UserData>>,
    repository: RwData<UserRepository<S>>,
) -> impl Responder
where
    S: UserDataSource<Item = UserData> + Send + 'static,
{
    let mut repository = repository.write().await;
    repository.delete_by_id(id.into_inner()).await;
    HttpResponse::Ok()
}
