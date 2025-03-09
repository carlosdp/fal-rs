use std::marker::PhantomData;

use eventsource_stream::Eventsource;
use futures::{Stream, StreamExt, TryStreamExt};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::FalError;

#[derive(Debug, Serialize, Deserialize)]
pub struct QueueResponse {
    pub request_id: String,
    pub response_url: String,
    pub status_url: String,
    pub cancel_url: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    InQueue,
    InProgress,
    Completed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestLog {
    pub timestamp: String,
    pub level: String,
    pub source: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueueStatus {
    pub status: Status,
    pub queue_position: Option<i64>,
    pub response_url: String,
    pub logs: Option<Vec<RequestLog>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Queue<Response: DeserializeOwned> {
    #[serde(skip)]
    pub client: Option<reqwest::Client>,
    pub endpoint: String,
    pub payload: QueueResponse,
    phantom: PhantomData<Response>,
}

impl<Response: DeserializeOwned> Queue<Response> {
    pub fn new(
        client: reqwest::Client,
        endpoint: impl Into<String>,
        payload: QueueResponse,
    ) -> Self {
        Self {
            client: Some(client),
            endpoint: endpoint.into(),
            payload,
            phantom: PhantomData,
        }
    }

    pub async fn status(&self, show_logs: bool) -> Result<QueueStatus, FalError> {
        let response = self
            .client
            .as_ref()
            .unwrap()
            .get(&self.payload.status_url)
            .query(&[("logs", if show_logs { "1" } else { "0" })])
            .header(
                "Authorization",
                format!("Key {}", std::env::var("FAL_API_KEY").unwrap()),
            )
            .header("Content-Type", "application/json")
            .send()
            .await?;
        println!("status: {:?}", response.status());

        Ok(response.error_for_status()?.json().await?)
    }

    pub async fn response(&self) -> Result<Response, FalError> {
        let response = self
            .client
            .as_ref()
            .unwrap()
            .get(&self.payload.response_url)
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

    pub async fn cancel(&self) -> Result<(), FalError> {
        let response = self
            .client
            .as_ref()
            .unwrap()
            .put(&self.payload.cancel_url)
            .header(
                "Authorization",
                format!("Key {}", std::env::var("FAL_API_KEY").unwrap()),
            )
            .send()
            .await?;

        response.error_for_status()?;

        Ok(())
    }

    pub async fn stream(
        &self,
    ) -> Result<impl Stream<Item = Result<QueueStatus, FalError>>, FalError> {
        let response = self
            .client
            .as_ref()
            .unwrap()
            .get(format!("{}/stream", &self.payload.status_url))
            .header(
                "Authorization",
                format!("Key {}", std::env::var("FAL_API_KEY").unwrap()),
            )
            .send()
            .await?;

        let stream = response.bytes_stream().eventsource();

        Ok(stream
            .map(|event_result| match event_result {
                Ok(event) => {
                    let status: QueueStatus =
                        serde_json::from_str(&event.data).map_err(|e| FalError::from(e))?;
                    Ok(status)
                }
                Err(e) => Err(FalError::from(e)),
            })
            .map_err(FalError::from))
    }
}
