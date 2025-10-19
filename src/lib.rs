//! # TWITCH-GQL-RS
//!
//! A small, lightweight implementation of a *GraphQL* client for interacting with **Twitch's GraphQL API**.
//! Designed for simple queries, typed responses, and easy integration into async Rust applications.
//!
//! ## Example
//!
//! ```rust
//! use std::path::Path;
//! use twitch_gql_rs::TwitchClient;
//! 
//! async fn example() -> Result<(), Box<dyn std::error::Error>> {
//!     let path = Path::new("save.json");
//!     let user_agent = "Dalvik/2.1.0 (Linux; U; Android 16; SM-S911B Build/TP1A.220624.014) tv.twitch.android.app/25.3.0/2503006";
//!
//!     if !path.exists() {
//!         let mut client = TwitchClient::new(
//!             "kd1unb4b3q4t58fwlpcbzcbnm76a8fp",
//!             user_agent,
//!             "https://www.twitch.tv",
//!         ).await?;
//!         client.auth().await?;
//!         client.save_file(&path).await?;
//!     }
//!
//!     let client = TwitchClient::load_from_file(&path).await?;
//!     let inv = client.get_inventory().await?;
//!
//!     for in_progress in inv.inventory.dropCampaignsInProgress {
//!         for time_based in in_progress.timeBasedDrops {
//!             if let Some(id) = time_based.self_drop.dropInstanceID {
//!                 println!("{id}");
//!             }
//!         }
//!     }
//! Ok(())
//! }
//! ```

use std::{error::Error, path::Path};

use reqwest::{header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, CACHE_CONTROL, ORIGIN, PRAGMA, REFERER, USER_AGENT}, Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use tokio::{fs};
mod error;
use error::*;
mod gql;
mod api;
use gql::*;
use api::*;

use crate::structs::{AvailableDrops, CampaignDetails, ClaimDrop, CurrentDrop, Drops, GameDirectory, GetInventory, PlaybackAccessToken, StreamInfo};
pub mod structs;

#[derive(Deserialize, Serialize)]
pub struct TwitchClient {
    #[serde(skip)]
    client: Client,
    client_id: String,
    user_agent: String,
    client_url: String,
    user_id: Option<String>,
    access_token: Option<String>,
}

async fn get_headers(
    client_id: &str,
    user_agent: &str,
    access_token: Option<&str>,
) -> Result<HeaderMap, Box<dyn Error>> {
    let device_id = uuid::Uuid::new_v4();
    let mut headers = HeaderMap::new();

    headers.insert(ACCEPT, HeaderValue::from_str("application/json")?);
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_str("en-US")?);
    headers.insert(CACHE_CONTROL, HeaderValue::from_str("no-cache")?);
    headers.insert("Client-Id", HeaderValue::from_str(&format!("{}", client_id))?);
    headers.insert(PRAGMA, HeaderValue::from_str("no-cache")?);
    headers.insert(ORIGIN, HeaderValue::from_str("https://www.twitch.tv")?);
    headers.insert(REFERER, HeaderValue::from_str("https://www.twitch.tv")?);
    headers.insert(USER_AGENT, HeaderValue::from_str(&format!("{}", user_agent))?);
    headers.insert("X-Device-Id", HeaderValue::from_str(&format!("{}", device_id))?);

    if let Some(token) = access_token {
        headers.insert("Authorization", HeaderValue::from_str(&format!("OAuth {}", token))?);
    }

    Ok(headers)
}

impl TwitchClient {
    /// Saves the current state of the structure to a JSON file at the specified path.
    /// Returns an error if the file already exists or if serialization fails.
    pub async fn save_file(self, path: &Path) -> Result<Self, SystemError> {
        if !path.exists() {
            let info = match serde_json::to_string_pretty(&self) {
                Ok(s) => s,
                Err(e) => return Err(SystemError::SerializationProblem(e)),
            };
            fs::write(&path, info.as_bytes()).await.unwrap();
            Ok(self)
        } else {
            Err(SystemError::FileAlredyExists)
        }
    }

    /// Loads the structure from a JSON file at the specified path.
    /// Returns an error if the file is not found or if deserialization fails.
    pub async fn load_from_file(path: &Path) -> Result<Self, SystemError> {
        if !path.exists() {
            return Err(SystemError::FileNotFound);
        }

        let load = fs::read_to_string(&path).await.unwrap();
        let mut load: TwitchClient = match serde_json::from_str(&load) {
            Ok(s) => s,
            Err(e) => return Err(SystemError::DeserializationProblem(e)),
        };

        let headers = get_headers(&load.client_id, &load.user_agent, load.access_token.as_deref()).await?;
        let client = ClientBuilder::new().default_headers(headers).build()?;
        load.client = client;

        Ok(load)
    }

    /// Creates a new `TwitchClient` instance without an access token.
    pub async fn new(client_id: &str, user_agent: &str, client_url: &str) -> Result<Self, SystemError> {
        let headers = get_headers(client_id, user_agent, None).await?;
        let client = ClientBuilder::new().default_headers(headers).build()?;

        Ok(TwitchClient {
            client,
            client_id: client_id.to_string(),
            user_agent: user_agent.to_string(),
            client_url: client_url.to_string(),
            user_id: None,
            access_token: None,
        })
    }

    // API
    /// Authenticates the `TwitchClient`.
    /// On success, stores the access token and user ID in the client.
    pub async fn auth(&mut self) -> Result<(), TwitchError> {
        let auth = auth(&self.client, &self.client_id).await?;
        self.access_token = Some(auth.0);
        self.user_id = Some(auth.1);
        Ok(())
    }

    /// Sends a "watch" event for a given channel.
    pub async fn send_watch(&self, channel_login: &str, broadcast_id: &str, channel_id: &str) -> Result<(), TwitchError> {
        if let Some(user_id) = &self.user_id {
            send_watch(&self.client, &user_id, &self.client_url, channel_login, broadcast_id, channel_id).await?;
        } else {
            return Err(TwitchError::TwitchError("Not found user_id".into()));
        }

        Ok(())
    }

    //GQL

    /// Retrieves the user's inventory from Twitch.
    pub async fn get_inventory (&self) -> Result<GetInventory, TwitchError> {
        let inv = inventory(&self.client).await?;
        Ok(inv)
    }

    /// Returns current information about Twitch Drops campaigns.
    pub async fn get_campaign (&self) -> Result<Drops, TwitchError> {
        let drops = campaign(&self.client).await?;
        Ok(drops)
    }

    /// Retrieves the slug for a given game name.
    pub async fn get_slug (&self, game_name: &str) -> Result<String, TwitchError> {
        let slug = slug_redirect(&self.client, game_name).await?;
        Ok(slug)
    }

    /// Retrieves the playback access token for a given Twitch channel.
    pub async fn get_playback_access_token (&self, channel_login: &str) -> Result<PlaybackAccessToken, TwitchError> {
        let playback = playback_access_token(&self.client, channel_login).await?;
        Ok(playback)
    }

    /// Retrieves a list of Twitch streams for a specific game, optionally filtering by drops-enabled streams
    pub async fn get_game_directory(&self, game_slug: &str, drops_enabled: bool) -> Result<Vec<GameDirectory>, TwitchError> {
        let streams = game_directory(&self.client, game_slug, drops_enabled).await?;
        Ok(streams)
    }

    /// Returns a list of available Twitch Drops and their progress for a given channel.
    pub async fn get_available_drops_for_channel (&self, channel_id: &str) -> Result<AvailableDrops, TwitchError> {
        let drops = available_drops(&self.client, channel_id).await?;
        Ok(drops)
    }

    /// Retrieves detailed information about a specific Twitch Drops campaign for a user
    pub async fn get_campaign_details (&self, user_login: &str, drop_id: &str) -> Result<CampaignDetails, TwitchError> {
        let details = campaign_details(&self.client, user_login, drop_id).await?;
        Ok(details)
    }

    /// Retrieves the current drop progress for a user on a specific Twitch channel.
    pub async fn get_current_drop_progress_on_channel (&self, channel_login: &str, channel_id: &str) -> Result<CurrentDrop, TwitchError> {
        let current = current_drop(&self.client, channel_login, channel_id).await?;
        Ok(current)
    }

    /// Retrieves the current stream information for a given Twitch channel.
    pub async fn get_stream_info (&self, channel_login: &str) -> Result<StreamInfo, TwitchError> {
        let stream_info = stream_info(&self.client, channel_login).await?;
        Ok(stream_info)
    }

    /// Claims a Twitch drop for the given drop instance ID
    pub async fn claim_drop (&self, drop_instance_id: &str) -> Result<ClaimDrop, TwitchError> {
        let claim = claim_drop(&self.client, drop_instance_id).await?;
        Ok(claim)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
