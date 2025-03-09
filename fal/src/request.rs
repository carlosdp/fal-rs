use std::marker::PhantomData;

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    queue::{Queue, QueueResponse},
    FalError,
};

#[derive(Debug)]
pub struct FalRequest<Params: Serialize, Response: DeserializeOwned> {
    pub client: reqwest::Client,
    pub endpoint: String,
    pub params: Params,
    phantom: PhantomData<Response>,
}

impl<Params: Serialize, Response: DeserializeOwned> FalRequest<Params, Response> {
    pub fn new(endpoint: impl Into<String>, params: Params) -> Self {
        Self {
            client: reqwest::Client::new(),
            endpoint: endpoint.into(),
            params,
            phantom: PhantomData,
        }
    }

    pub async fn send(self) -> Result<Response, FalError> {
        let response = self
            .client
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

    pub async fn queue(self) -> Result<Queue<Response>, FalError> {
        let response = self
            .client
            .post(format!("https://queue.fal.run/{}", self.endpoint))
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

        let payload: QueueResponse = response.error_for_status()?.json().await?;

        Ok(Queue::new(self.client, self.endpoint, payload))
    }
}
