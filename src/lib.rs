use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct HelixDB {
    port: u16,
    client: Client,
}

// This trait allows users to implement their own client if needed
pub trait HelixDBClient {
    fn new(port: Option<u16>) -> Self;
    async fn query<T, R>(
        &self,
        endpoint: &str,
        data: &T,
    ) -> impl std::future::Future<Output = Result<R>> + Send
    where
        T: Serialize,
        R: for<'de> Deserialize<'de>;
}

impl HelixDB {
    pub fn new(port: Option<u16>) -> Self {
        Self {
            port: port.unwrap_or(6969),
            client: Client::new(),
        }
    }

    pub async fn query<T, R>(&self, endpoint: &str, data: &T) -> Result<R>
    where
        T: Serialize,
        R: for<'de> Deserialize<'de>,
    {
        let url = format!("http://localhost:{}/{}", self.port, endpoint);

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
        let client = HelixDB::new(None);

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

        // Note: This test will fail unless HelixDB is running locally
        let _result: UserOutput = client.query("addUser", &input).await.unwrap();
    }
}
