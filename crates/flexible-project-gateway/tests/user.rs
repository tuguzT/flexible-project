use std::time::Duration;

use tokio::time::sleep;

#[tokio::test]
async fn get_all_users() {
    let duration = Duration::from_secs_f32(1.0);
    sleep(duration).await
}

#[tokio::test]
async fn find_user_by_id() {
    let duration = Duration::from_secs_f32(2.0);
    sleep(duration).await
}

#[tokio::test]
async fn find_user_by_name() {
    let duration = Duration::from_secs_f32(2.0);
    sleep(duration).await
}

#[tokio::test]
async fn update_user_name() {
    let duration = Duration::from_secs_f32(2.0);
    sleep(duration).await
}

#[tokio::test]
async fn update_user_display_name() {
    let duration = Duration::from_secs_f32(2.0);
    sleep(duration).await
}

#[tokio::test]
async fn update_user_avatar_image() {
    let duration = Duration::from_secs_f32(2.0);
    sleep(duration).await
}

#[tokio::test]
async fn delete_user() {
    let duration = Duration::from_secs_f32(2.0);
    sleep(duration).await
}
