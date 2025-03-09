use fal::prelude::*;

#[tokio::test]
async fn test_fal_ai_flux_pro() {
    let response = fal::endpoints::fal_ai::flux::dev::dev(DevTextToImageInput {
        prompt: "a horse".to_string(),
        ..Default::default()
    })
    .send()
    .await
    .unwrap();

    assert!(response.images.len() > 0);
}
