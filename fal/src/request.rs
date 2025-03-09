use std::marker::PhantomData;

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    queue::{Queue, QueueResponse},
    FalError,
};

#[derive(Debug)]
pub struct FalRequest<Params: Serialize, Response: DeserializeOwned> {
    /// The Reqwest Client to use to make requests
    pub client: reqwest::Client,
    /// The endpoint to make the request to
    pub endpoint: String,
    /// The parameters to send to the endpoint
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

    /// Use a specific Reqwest Client to make requests
    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = client;

        self
    }

    /// Send the request and wait for the response
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

    /// For requests that take longer than several seconds, as it is usually the case with AI applications, we have built a queue system.
    ///
    /// Utilizing our queue system offers you a more granulated control to handle unexpected surges in traffic.
    /// It further provides you with the capability to cancel requests if needed and grants you the observability to monitor your current
    /// position within the queue. Besides that using the queue system spares you from the headache of keeping around long running https requests.
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
