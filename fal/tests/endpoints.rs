use fal::prelude::FluxProTextToImageInput;

#[tokio::test]
async fn test_fal_ai_flux_pro() {
    let response = fal::endpoints::fal_ai::flux_pro::flux_pro(FluxProTextToImageInput {
        prompt: "a horse".to_string(),
        ..Default::default()
    })
    .send()
    .await
    .unwrap();

    assert!(response.images.len() > 0);
}
