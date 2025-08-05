# helix-rs

A Rust SDK for interacting with HelixDB - providing a simple, type-safe interface for database operations.

## Features

- Type-safe query interface using Serde serialization/deserialization
- Async/await support
- Configurable port settings
- Custom client implementation support via `HelixDBClient` trait

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
helix-db= "0.1.0"
```

## Quick Start

```rust
use helix_rs::HelixDB;
use serde::{Serialize, Deserialize};

// Define your data structures
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the client
    let client = HelixDB::new(None, None, None); // Uses default port 6969

    // Create a user
    let input = UserInput {
        name: "John".to_string(),
        age: 20,
    };

    let result = client.query::<UserInput, UserOutput>("add_user", &input).await?;
    println!("Created user with ID: {}", result.id);
    Ok(())
}
```

## Configuration

### Custom Port

You can specify a custom port when initializing the client:

```rust
let client = HelixDB::new(None, Some(8080), None); // Uses port 8080
```

### Custom Client

You can implement your own client by implementing the `HelixDBClient` trait:

```rust
use helix_rs::HelixDBClient;

struct MyCustomClient {
    // Your implementation details
}

impl HelixDBClient for MyCustomClient {
    fn new(endpoint: Option<&str>, port: Option<u16>, api_key: Option<&str>) -> Self {
        // Your initialization logic
    }

    async fn query<T, R>(&self, endpoint: &str, data: &T) -> anyhow::Result<R>
    where
        T: Serialize,
        R: for<'de> Deserialize<'de>
    {
        // Your query implementation
    }
}
```

## Requirements

- Rust 1.56 or higher
- Running HelixDB instance
- Tokio runtime for async operations
