extern crate helix_macros;

pub use helix_macros::helix_node;

use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug)]
pub struct HelixDB {
    port: Option<u16>,
    client: Client,
    endpoint: String,
    api_key: Option<String>,
}

#[derive(Debug, Error)]
pub enum HelixError {
    #[error("Error communicating with server: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Got Error from server: {details}")]
    RemoteError { details: String },
}

// This trait allows users to implement their own client if needed
pub trait HelixDBClient {
    type Err: std::error::Error;

    fn new(endpoint: Option<&str>, port: Option<u16>, api_key: Option<&str>) -> Self;
    fn query<T, R>(
        &self,
        endpoint: &str,
        data: &T,
    ) -> impl std::future::Future<Output = Result<R, Self::Err>> + Send
    where
        T: Serialize + Sync,
        R: for<'de> Deserialize<'de>;
}

impl HelixDBClient for HelixDB {
    type Err = HelixError;
    fn new(endpoint: Option<&str>, port: Option<u16>, api_key: Option<&str>) -> Self {
        Self {
            port: port,
            client: Client::new(),
            endpoint: endpoint.unwrap_or("http://localhost").to_string(),
            api_key: api_key.map(|key| key.to_string()),
        }
    }

    async fn query<T, R>(&self, endpoint: &str, data: &T) -> Result<R, HelixError>
    where
        T: Serialize + Sync,
        R: for<'de> Deserialize<'de>,
    {
        let port = match self.port {
            Some(port) => format!(":{}", port),
            None => "".to_string(),
        };

        let url = format!("{}{}/{}", self.endpoint, port, endpoint);

        let mut request = self.client.post(&url).json(data);

        // Add API key header if provided
        if let Some(ref api_key) = self.api_key {
            request = request.header("x-api-key", api_key);
        }

        let response = request.send().await?;

        match response.status() {
            StatusCode::OK => response.json().await.map_err(Into::into),
            // code => Err(HelixError::RemoteError{details: response.text().await.unwrap_or(code.canonical_reason().unwrap_or(format!()))}),
            code => match response.text().await {
                Ok(t) => Err(HelixError::RemoteError { details: t }),
                Err(_) => match code.canonical_reason() {
                    Some(r) => Err(HelixError::RemoteError {
                        details: r.to_string(),
                    }),
                    None => Err(HelixError::RemoteError {
                        details: format!("unkown error with code: {code}"),
                    }),
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::helix_node;
    use super::*;

    #[helix_node]
    struct User {
        name: String,
        age: i32,
    }

    #[tokio::test]
    async fn test_basic_query() {
        let client = HelixDB::new(None, None, None);

        // Example test structure
        #[derive(Serialize)]
        struct UserInput {
            name: String,
            age: i32,
        }

        #[derive(Deserialize)]
        struct UserOutput {
            id: String,
            name: String,
            age: i32,
        }

        let input = UserInput {
            name: "John".to_string(),
            age: 20,
        };

        #[derive(Deserialize)]
        struct Result {
            pub user: UserOutput,
        }

        // Note: This test will fail unless HelixDB is running locally
        let _result: Result = client.query("add_user", &input).await.unwrap();
    }

    #[tokio::test]
    async fn test_query_with_api_key() {
        let client = HelixDB::new(None, None, Some("test-api-key"));

        // Example test structure
        #[derive(Serialize)]
        struct UserInput {
            name: String,
            age: i32,
        }

        #[derive(Deserialize)]
        struct UserOutput {
            id: String,
            name: String,
            age: i32,
        }

        let input = UserInput {
            name: "Jane".to_string(),
            age: 25,
        };

        #[derive(Deserialize)]
        struct Result {
            pub user: UserOutput,
        }

        // Note: This test will fail unless HelixDB is running locally with API key support
        let _result: Result = client.query("add_user", &input).await.unwrap();
    }
}
