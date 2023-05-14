use std::time::Duration;

use tokio::time::sleep;

#[tokio::test]
async fn sign_in() {
    let duration = Duration::from_secs_f32(1.5);
    sleep(duration).await
}

#[tokio::test]
async fn sign_up() {
    let duration = Duration::from_secs_f32(2.0);
    sleep(duration).await
}

#[tokio::test]
async fn log_out() {
    let duration = Duration::from_secs_f32(1.0);
    sleep(duration).await
}

#[tokio::test]
async fn get_tokens() {
    let duration = Duration::from_secs_f32(2.0);
    sleep(duration).await
}

#[tokio::test]
async fn refresh_token() {
    let duration = Duration::from_secs_f32(2.0);
    sleep(duration).await
}
