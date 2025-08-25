# helix-rs

A Rust SDK for interacting with HelixDB - providing a simple, type-safe interface for database operations.

## Features

- Type-safe query interface using Serde serialization/deserialization
- Async/await support
- Configurable port settings
- Custom client implementation support via `HelixDBClient` trait

## Quick Start

```bash
cargo add helix-rs
cargo add serde
cargo add tokio
```

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
async fn main() -> Result<(), HelixError> {
    // Initialize the client
    let client = HelixDB::new(None, None, None); // Uses default port 6969

    // Create a user
    
    let input = AddUserInput {
        name: "John".to_string(),
        age: 20,
    };

    // Define the output structure
    #[derive(Deserialize)]
    struct Result {
        user: AddUserOutput,
    }

    let result = client.query::<UserInput, Result>("add_user", &input).await?;
    println!("Created user with ID: {}", result.user.id);
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
use helix_rs::{HelixDBClient, HelixError};

struct MyCustomClient {
    // Your implementation details
}

impl HelixDBClient for MyCustomClient {
    type Err = HelixError;
    fn new(endpoint: Option<&str>, port: Option<u16>, api_key: Option<&str>) -> Self {
        // Your initialization logic
    }

    async fn query<T, R>(&self, endpoint: &str, data: &T) -> Result<R, HelixError>
    where
        T: Serialize + Sync,
        R: for<'de> Deserialize<'de>
    {
        // Your query implementation
    }
}
```

## Requirements

- Rust 1.88.0 or higher
- Running HelixDB instance
- Tokio runtime for async operations
