# TWITCH-GQL-RS

A small, lightweight implementation of a *GraphQL* client for interacting with **Twitch's GraphQL API**.
Designed for simple queries, typed responses, and easy integration into async Rust applications.

## Example

```rust
use std::path::Path;
use twitch_gql_rs::TwitchClient;

async fn example() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("save.json");
    let user_agent = "Dalvik/2.1.0 (Linux; U; Android 16; SM-S911B Build/TP1A.220624.014) tv.twitch.android.app/25.3.0/2503006";

    if !path.exists() {
        let mut client = TwitchClient::new(
            "kd1unb4b3q4t58fwlpcbzcbnm76a8fp",
            user_agent,
            "https://www.twitch.tv",
        ).await?;
        client.auth().await?;
        client.save_file(&path).await?;
    }

    let client = TwitchClient::load_from_file(&path).await?;
    let inv = client.get_inventory().await?;

    for in_progress in inv.inventory.dropCampaignsInProgress {
        for time_based in in_progress.timeBasedDrops {
            if let Some(id) = time_based.self_drop.dropInstanceID {
                println!("{id}");
            }
        }
    }
Ok(())
}
```