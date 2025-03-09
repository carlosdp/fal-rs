Creates a custom endpoint function, compatible with the fal API.

```rust
use fal::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FalResponse {
  pub images: Vec<File>,
}

#[endpoint(endpoint="fal-ai/flux/dev")]
pub fn fal_dev(prompt: String) -> FalResponse;
```

This endpoint function can now be used to call the fal endpoint:

```rust
#[tokio::main]
async fn main() {
    let response = flux_dev("an horse riding an astronaut".to_owned())
    .send()
    .await
    .unwrap();

    println!("Generated image URL: {}", response.images[0].url);
}
```