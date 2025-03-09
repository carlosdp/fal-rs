# fal-rs

[![Crates.io](https://img.shields.io/crates/v/fal.svg)](https://crates.io/crates/fal)
[![Documentation](https://docs.rs/fal/badge.svg)](https://docs.rs/fal)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

A Rust client for the [fal.ai](https://fal.ai) API, providing easy access to state-of-the-art AI models for image generation, audio processing, and more.

## Features

- **Type-safe API**: Strongly typed interfaces for all fal.ai endpoints, generated and kept up-to-date straight from the API itself
- **Async/Await Support**: Built on top of `reqwest` for efficient async operations
- **Queue System**: Built-in support for fal.ai's queue system for long-running operations
- **Image Processing**: Optional image processing capabilities with the `image` feature

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fal = "0.1"
```

## Usage
### Using public model endpoints

By default, the `endpoints` feature is enabled, and you can use pre-built, fully-typed endpoint functions to call the API:

```rust,no_run
use fal::prelude::*;

#[tokio::main]
async fn main() {
    // Use the Flux Pro endpoint to generate an image
    let response = fal::endpoints::fal_ai::flux_pro::flux_pro(FluxProTextToImageInput {
        prompt: "a majestic horse in a field".to_string(),
        ..Default::default()
    })
    .with_api_key("fal_api_key_here") // If not provided, the FAL_API_KEY environment variable is used
    .send()
    .await
    .unwrap();

    println!("Generated image URL: {}", response.images[0].url);
}
```

### Using the Queue System

For long-running operations, you can use the [FAL Queue API](https://docs.fal.ai/model-endpoints/queue):

```rust,no_run
use fal::prelude::*;
use futures::StreamExt;

#[tokio::main]
async fn main() {
    let queue = fal::endpoints::fal_ai::flux_pro::flux_pro(FluxProTextToImageInput {
        prompt: "a majestic horse in a field".to_string(),
        ..Default::default()
    })
    .queue()
    .await
    .unwrap();

    // Stream status updates
    while let Some(status) = queue.stream(true).await.unwrap().next().await {
        let status = status.unwrap();
        println!("Status: {:?}", status.status);
        
        if status.status == Status::Completed {
            break;
        }
    }

    let response = queue.response().await.unwrap();
    println!("Generated image URL: {}", response.images[0].url);
}
```

## The `#[endpoint]` macro
You can easily create a custom endpoint function using the provided [endpoint](crate::endpoint) proc macro. This should only be necessary if you disable the `endpoints` feature, or you are using a private model endpoint.

```toml
# in Cargo.toml
[dependencies]
fal = { version = "0.1", default-features = false, features = ["image"] }
```

```rust,no_run
use fal::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FalResponse {
  pub images: Vec<File>,
}

#[endpoint(endpoint="fal-ai/flux/dev")]
pub fn flux_dev(prompt: String) -> FalResponse {}

// This endpoint function can now be used to call the fal endpoint:
#[tokio::main]
async fn main() {
    let response = flux_dev("an horse riding an astronaut".to_owned())
    .send()
    .await
    .unwrap();

    println!("Generated image URL: {}", response.images[0].url);
}
```

## Features

The crate comes with several optional features:

- `image` (enabled by default): Provides image processing capabilities using the `image` crate
- `endpoints` (enabled by default): Includes pre-generated endpoint modules for fal.ai services. FAL has hundreds of endpoints, growing by the day, so disabling this feature will reduce compile time.

## Generating Endpoint Modules

The `generate` package in this repository is used to automatically generate endpoint modules based on the fal.ai API specification. This ensures that the client always has up-to-date type definitions and endpoint implementations.

To generate endpoint modules:

1. Clone the repository
2. Run the generate package:

```bash
cargo run -p generate-endpoints
```

This will update the endpoint modules in the `fal/src/endpoints` directory with the latest API definitions, using the model registry from the FAL API.

## License

Licensed under either of:

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

We welcome contributions! Please feel free to submit a Pull Request.
