# TWITCH-GQL-RS

[![crates.io](https://img.shields.io/crates/v/twitch-gql-rs.svg)](https://crates.io/crates/twitch-gql-rs)
[![Documentation](https://docs.rs/twitch-gql-rs/badge.svg)](https://docs.rs/twitch-gql-rs/0.1.6/twitch_gql_rs)
[![github.com](https://img.shields.io/github/stars/this-is-really/twitch-gql-rs.svg?style=social&label=Star)](https://github.com/this-is-really/twitch-gql-rs)

A small, lightweight implementation of a *GraphQL* client for interacting with **Twitch's GraphQL API**.
Designed for simple queries, typed responses, and easy integration into async Rust applications.

## Example

```rust
use std::{error::Error, path::Path};
use twitch_gql_rs::{client_type::ClientType, TwitchClient};

async fn example() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("save.json");

    if !path.exists() {
        let client_type = ClientType::android_app();
        let mut client = TwitchClient::new(&client_type).await?;
        let get_auth = client.request_device_auth().await?;
        println!("Please open the following link in your browser:\n{}\nThen enter this code: {}", get_auth.verification_uri, get_auth.user_code);
        client.auth(get_auth).await?;
        client.save_file(&path).await?;
    }

    let client = TwitchClient::load_from_file(&path).await?;
    let inventory = client.get_inventory().await?;
    for in_progress in inventory.inventory.dropCampaignsInProgress {
        for time_based in in_progress.timeBasedDrops {
            if let Some(id) = time_based.self_drop.dropInstanceID {
                println!("{id}")
            }
        }
    }
Ok(())
}
```