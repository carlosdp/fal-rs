use fal::{queue::Status, FalMultiImageResponse};

#[fal::endpoint(endpoint = "fal-ai/flux/dev")]
fn fal_dev(prompt: String) -> FalMultiImageResponse {}

#[tokio::test]
async fn test_queue() {
    let queue = fal_dev("a horse".to_owned()).queue().await.unwrap();

    loop {
        let status = queue.status(false).await.unwrap();

        if status.status == Status::Completed {
            break;
        }

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    let response = queue.response().await.unwrap();

    assert!(response.images.len() > 0)
}
