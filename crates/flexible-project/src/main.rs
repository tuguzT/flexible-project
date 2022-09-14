#![warn(missing_docs)]
#![warn(clippy::all)]

//! Flexible Project server.

use actix_web::{get, post, web, App, HttpServer, Responder};
use fp_data::data_source::mock::user::MockUserDataSource;
use fp_data::model::UserData;
use fp_data::repository::ops::{ReadAll, Save};
use fp_data::repository::user::UserRepository;
use tokio::sync::RwLock;

type MockUserRepository = UserRepository<MockUserDataSource>;

/// Get all users of the Flexible Project system.
#[get("/users")]
async fn all_users(repository: web::Data<RwLock<MockUserRepository>>) -> impl Responder {
    let repository = repository.read().await;
    let all_users = repository.read_all().await;
    web::Json(all_users)
}

/// Save user data in the Flexible Project system.
#[post("/users")]
async fn save_user(
    user: web::Json<UserData>,
    repository: web::Data<RwLock<MockUserRepository>>,
) -> impl Responder {
    let mut repository = repository.write().await;
    let user = repository.save(user.into_inner()).await;
    web::Json(user)
}

/// Entry point of the server.
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let data_source = MockUserDataSource::default();
    let repository = MockUserRepository::new(data_source);
    let repository = web::Data::new(RwLock::new(repository));

    HttpServer::new(move || {
        App::new()
            .app_data(repository.clone())
            .service(all_users)
            .service(save_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
