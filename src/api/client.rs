use super::error::ApiError;
use super::types::{Chore, ChoreListResponse, CreateChoreRequest};
use reqwest::header::{HeaderMap, HeaderValue};
use std::time::Duration;

#[derive(Clone)]
pub struct ApiClient {
    client: reqwest::Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: String, token: String) -> Result<Self, ApiError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "secretkey",
            HeaderValue::from_str(&token)
                .map_err(|e| ApiError::InvalidConfig(format!("Invalid token: {}", e)))?,
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build()?;

        Ok(Self { client, base_url })
    }

    pub async fn list_chores(&self) -> Result<Vec<Chore>, ApiError> {
        let url = format!("{}/eapi/v1/chore", self.base_url);
        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_text = response.text().await.unwrap_or_default();
            return Err(ApiError::ServerError(status, error_text));
        }

        // Try to parse as wrapped response first ({"res": [...]})
        let text = response.text().await?;
        if let Ok(wrapped) = serde_json::from_str::<ChoreListResponse>(&text) {
            return Ok(wrapped.res);
        }

        // Fall back to direct array parsing
        let chores: Vec<Chore> = serde_json::from_str(&text)?;
        Ok(chores)
    }

    pub async fn create_chore(&self, name: String, due_date: String) -> Result<Chore, ApiError> {
        let url = format!("{}/eapi/v1/chore", self.base_url);
        let request = CreateChoreRequest {
            name,
            due_date: if due_date.is_empty() {
                None
            } else {
                Some(due_date)
            },
        };

        let response = self.client.post(&url).json(&request).send().await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_text = response.text().await.unwrap_or_default();
            return Err(ApiError::ServerError(status, error_text));
        }

        let chore: Chore = response.json().await?;
        Ok(chore)
    }

    pub async fn complete_chore(&self, id: i64) -> Result<(), ApiError> {
        let url = format!("{}/eapi/v1/chore/{}/complete", self.base_url, id);
        let response = self.client.post(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_text = response.text().await.unwrap_or_default();
            return Err(ApiError::ServerError(status, error_text));
        }

        Ok(())
    }
}
