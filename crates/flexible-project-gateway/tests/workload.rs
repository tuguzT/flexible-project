use futures::{future::join_all, FutureExt};
use reqwest::{get, StatusCode};

#[tokio::test]
async fn workload() {
    const REQUEST_COUNT: usize = 100;

    async fn generate_request() {
        let Ok(response) = get("https://0.0.0.0:8080/graphql").await else {
            return;
        };
        assert_ne!(response.status(), StatusCode::OK)
    }

    let mut futures = Vec::with_capacity(REQUEST_COUNT);
    for _ in 0..REQUEST_COUNT {
        let request = generate_request().boxed();
        futures.push(request)
    }
    let _results = join_all(futures).await;
}
