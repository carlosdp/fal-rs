use std::marker::PhantomData;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::FalError;

#[derive(Debug, Deserialize)]
pub struct FalRequest<Params: Serialize, Response: DeserializeOwned> {
    pub endpoint: String,
    pub params: Params,
    phantom: PhantomData<Response>,
}

impl<Params: Serialize, Response: DeserializeOwned> FalRequest<Params, Response> {
    pub fn new(endpoint: impl Into<String>, params: Params) -> Self {
        Self {
            endpoint: endpoint.into(),
            params,
            phantom: PhantomData,
        }
    }

    pub async fn send(self) -> Result<Response, FalError> {
        let client = reqwest::Client::new();
        let response = client
            .post(format!("https://fal.run/{}", self.endpoint))
            .json(&self.params)
            .header(
                "Authorization",
                format!("Key {}", std::env::var("FAL_API_KEY").unwrap()),
            )
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if response.status() != 200 {
            let error = response.text().await?;
            return Err(error.into());
        }

        Ok(response.error_for_status()?.json().await?)
    }
}
