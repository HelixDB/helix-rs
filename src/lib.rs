use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct HelixDB {
    port: Option<u16>,
    client: Client,
    endpoint: String,
}

// This trait allows users to implement their own client if needed
pub trait HelixDBClient {
    fn new(endpoint: Option<&str>, port: Option<u16>) -> Self;
    fn query<T, R>(
        &self,
        endpoint: &str,
        data: &T,
    ) -> impl std::future::Future<Output = Result<R>> + Send
    where
        T: Serialize + Sync,
        R: for<'de> Deserialize<'de>;
}

impl HelixDBClient for HelixDB {
    fn new(endpoint: Option<&str>, port: Option<u16>) -> Self {
        Self {
            port: port,
            client: Client::new(),
            endpoint: endpoint.unwrap_or("http://localhost").to_string(),
        }
    }

    async fn query<T, R>(&self, endpoint: &str, data: &T) -> Result<R>
    where
        T: Serialize + Sync,
        R: for<'de> Deserialize<'de>,
    {
        let port = match self.port {
            Some(port) => format!(":{}", port),
            None => "".to_string(),
        };

        let url = format!("{}{}/{}", self.endpoint, port, endpoint);

        let response = self.client.post(&url).json(data).send().await?;
        let result = response.json().await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_query() {
        let client = HelixDB::new(None, None);

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
}
