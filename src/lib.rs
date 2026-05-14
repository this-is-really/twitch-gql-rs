//! # TWITCH-GQL-RS
//!
//! [![crates.io](https://img.shields.io/crates/v/twitch-gql-rs.svg)](https://crates.io/crates/twitch-gql-rs)
//! [![Documentation](https://docs.rs/twitch-gql-rs/badge.svg)](https://docs.rs/twitch-gql-rs/0.1.6/twitch_gql_rs)
//! [![github.com](https://img.shields.io/github/stars/this-is-really/twitch-gql-rs.svg?style=social&label=Star)](https://github.com/this-is-really/twitch-gql-rs)
//! 
//! A small, lightweight implementation of a *GraphQL* client for interacting with **Twitch's GraphQL API**.
//! Designed for simple queries, typed responses, and easy integration into async Rust applications.
//!
//! ## Example
//!
//! ```rust
//! use std::{error::Error, path::Path};
//! use twitch_gql_rs::{client_type::ClientType, TwitchClient};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let path = Path::new("save.json");
//!
//!     if !path.exists() {
//!         let client_type = ClientType::android_app();
//!         let mut client = TwitchClient::new(&client_type).await?;
//!         let get_auth = client.request_device_auth().await?;
//!         println!("Please open the following link in your browser:\n{}\nThen enter this code: {}", get_auth.verification_uri, get_auth.user_code);
//!         client.auth(get_auth).await?;
//!         client.save_file(&path).await?;
//!     }
//!
//!     let client = TwitchClient::load_from_file(&path).await?;
//!     let inventory = client.get_inventory().await?;
//!     for in_progress in inventory.inventory.dropCampaignsInProgress {
//!         for time_based in in_progress.timeBasedDrops {
//!             if let Some(id) = time_based.self_drop.dropInstanceID {
//!                 println!("{id}");
//!             }
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```


use std::{error::Error, path::Path};

use reqwest::{Client, ClientBuilder, Proxy, header::{ACCEPT, ACCEPT_LANGUAGE, CACHE_CONTROL, HeaderMap, HeaderValue, ORIGIN, PRAGMA, REFERER, USER_AGENT}};
use serde::{Deserialize, Serialize};
use tokio::{fs};
pub mod error;
use error::*;
mod gql;
mod api;
use gql::*;
use api::*;

use crate::{client_type::ClientType, structs::{AvailableDrops, CampaignDetails, ClaimDrop, CurrentDrop, Drops, GameDirectory, GetInventory, PlaybackAccessToken, StreamInfo}};
/// All data structures used in the project
pub mod structs;
/// Client types
pub mod client_type;

/// Represents a Twitch GraphQL client used to interact with Twitch's API.
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct TwitchClient {
    #[serde(skip)]
    client: Client,
    pub client_id: String,
    pub user_agent: String,
    pub client_url: String,
    pub device_id: String,
    pub user_id: Option<String>,
    pub login: Option<String>,
    pub access_token: Option<String>,
}

async fn get_headers(
    client_id: &str,
    user_agent: &str,
    device_id: &str,
    access_token: &Option<String>,
) -> Result<HeaderMap, Box<dyn Error>> {
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

async fn build_client (twitch_client: &TwitchClient, proxy: &Option<String>) -> Result<Client, SystemError> {
    let headers = get_headers(&twitch_client.client_id, &twitch_client.user_agent, &twitch_client.device_id, &twitch_client.access_token).await?;
    let mut builder = ClientBuilder::new().default_headers(headers);

    if let Some(pr) = &proxy {
        let proxy = Proxy::all(pr).map_err(|e| SystemError::InvalidProxy { proxy_url: pr.to_string(), details: e.to_string() })?;
        builder = builder.proxy(proxy);
    }

    let client = builder.build()?;

    Ok(client)
}

impl TwitchClient {
    /// Saves the current state of the structure to a JSON file at the specified path.
    /// Returns an error if the file already exists or if serialization fails.
    pub async fn save_file(&self, path: &Path) -> Result<(), SystemError> {
        if !path.exists() {
            let info = match serde_json::to_string_pretty(&self) {
                Ok(s) => s,
                Err(e) => return Err(SystemError::SerializationProblem(e)),
            };
            fs::write(&path, info.as_bytes()).await?;
            Ok(())
        } else {
            Err(SystemError::FileAlreadyExists)
        }
    }

    /// Loads the structure from a JSON file at the specified path.
    /// Returns an error if the file is not found or if deserialization fails.
    pub async fn load_from_file(path: &Path, proxy: &Option<String>) -> Result<Self, SystemError> {
        if !path.exists() {
            return Err(SystemError::FileNotFound);
        }

        let load = fs::read_to_string(&path).await?;
        let mut load: TwitchClient = match serde_json::from_str(&load) {
            Ok(s) => s,
            Err(e) => return Err(SystemError::DeserializationProblem(e)),
        };

        let client = build_client(&load, proxy).await?;

        load.client = client;

        Ok(load)
    }

    /// Creates a new `TwitchClient` instance without an access token.
    ///
    /// # Proxy support
    ///
    /// You can optionally pass a proxy URL. Supported formats:
    ///
    /// - HTTP proxy: `"http://127.0.0.1:8080"`
    /// - HTTP proxy with authentication: `"http://username:password@127.0.0.1:8080"`
    /// - SOCKS5 proxy: `"socks5://127.0.0.1:1080"`
    /// - SOCKS5 with authentication: `"socks5://username:password@127.0.0.1:1080"`
    ///
    /// If the proxy string is invalid, [`SystemError::InvalidProxy`] will be returned.
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Without proxy
    /// let client = TwitchClient::new(&client_type, None).await?;
    ///
    /// // With HTTP proxy
    /// let client = TwitchClient::new(&client_type, Some("http://127.0.0.1:8080".to_string())).await?;
    ///
    /// // With authenticated proxy
    /// let client = TwitchClient::new(
    ///     &client_type,
    ///     Some("http://user:pass@192.168.1.50:3128".to_string())
    /// ).await?;
    /// ```
    ///
    pub async fn new(client_type: &ClientType, proxy_str: &Option<String>) -> Result<Self, SystemError> {
        let device_id = uuid::Uuid::new_v4().to_string();
        let temp = TwitchClient {
            client: Client::new(),
            client_id: client_type.client_id.to_string(),
            user_agent: client_type.user_agent.to_string(),
            client_url: client_type.client_url.to_string(),
            device_id: device_id.clone(),
            user_id: None,
            login: None,
            access_token: None
        };

        let client = build_client(&temp, proxy_str).await?;

        Ok(TwitchClient {
            client,
            client_id: client_type.client_id.to_string(),
            user_agent: client_type.user_agent.to_string(),
            client_url: client_type.client_url.to_string(),
            device_id,
            user_id: None,
            login: None,
            access_token: None,
        })
    }

    // API
    /// Requests Device Flow from Twitch and returns a `DeviceAuth` structure.
    pub async fn request_device_auth(&self) -> Result<DeviceAuth, TwitchError> {
        let auth = request_device_auth(&self.client, &self.client_id).await?;
        Ok(auth)
    }

    /// Authenticates the `TwitchClient`.
    /// Starts a token polling cycle via Device Flow using the passed `DeviceAuth`.
    pub async fn auth (&mut self, device_auth: DeviceAuth) -> Result<(), AuthError> {
        let auth = poll_device_auth(&self.client, &self.client_id, device_auth).await?;
        self.access_token = Some(auth.0);
        self.user_id = Some(auth.1);
        self.login = Some(auth.2);
        Ok(())
    } 

    /// Sends a "watch" event for a given channel.
    pub async fn send_watch(&self, channel_login: &str, broadcast_id: &str, channel_id: &str, game_name: Option<&str>, game_id: Option<&str>) -> Result<(), TwitchError> {
        if let Some(user_id) = &self.user_id {
            send_watch_gql(&self.client, user_id, channel_login, channel_id, broadcast_id, game_name, game_id).await?;
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
    pub async fn get_slug (&self, game_name: &str) -> Result<String, SlugError> {
        let slug = slug_redirect(&self.client, game_name).await?;
        Ok(slug)
    }

    /// Retrieves the playback access token for a given Twitch channel.
    pub async fn get_playback_access_token (&self, channel_login: &str) -> Result<PlaybackAccessToken, TwitchError> {
        let playback = playback_access_token(&self.client, channel_login).await?;
        Ok(playback)
    }

    /// Retrieves a list of Twitch streams for a specific game, optionally filtering by drops-enabled streams
    pub async fn get_game_directory(&self, game_slug: &str, limit: u64, drops_enabled: bool) -> Result<Vec<GameDirectory>, GameDirectoryError> {
        let streams = game_directory(&self.client, game_slug, limit, drops_enabled).await?;
        Ok(streams)
    }

    /// Returns a list of available Twitch Drops and their progress for a given channel.
    pub async fn get_available_drops_for_channel (&self, channel_id: &str) -> Result<AvailableDrops, AvailableDropsError> {
        let drops = available_drops(&self.client, channel_id).await?;
        Ok(drops)
    }

    /// Retrieves detailed information about a specific Twitch Drops campaign for a user
    pub async fn get_campaign_details (&self, drop_id: &str) -> Result<CampaignDetails, CampaignDetailsError> {
        if let Some(login) = &self.login {
            let details = campaign_details(&self.client, &login, drop_id).await?;
            return Ok(details)
        } else {
            return Err(CampaignDetailsError::TwitchError(TwitchError::TwitchError("Not found login".into())));
        }
    }

    /// Retrieves the current drop progress for a user on a specific Twitch channel.
    pub async fn get_current_drop_progress_on_channel (&self, channel_login: &str) -> Result<CurrentDrop, TwitchError> {
        let current = current_drop(&self.client, channel_login).await?;
        Ok(current)
    }

    /// Retrieves the current stream information for a given Twitch channel.
    pub async fn get_stream_info (&self, channel_login: &str) -> Result<StreamInfo, StreamInfoError> {
        let stream_info = stream_info(&self.client, channel_login).await?;
        Ok(stream_info)
    }

    /// Claims a Twitch drop for the given drop instance ID
    pub async fn claim_drop (&self, drop_instance_id: &str) -> Result<ClaimDrop, ClaimDropError> {
        let claim = claim_drop(&self.client, drop_instance_id).await?;
        Ok(claim)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

use tokio::time::sleep;

use super::*;

    #[tokio::test]
    async fn test() -> Result<(), Box<dyn Error>> {
        let path = Path::new("save.json");
        if !path.exists() {
            let mut client = TwitchClient::new(&ClientType::web(), &None).await?;
            let auth = client.request_device_auth().await?;
            println!("{} {}", auth.verification_uri, auth.user_code);
            client.auth(auth).await?;
            client.save_file(path).await?;
        }

        let client = TwitchClient::load_from_file(path, &None).await?;
        let slug = client.get_slug("Marvel Rivals").await?;
        let game_directory = client.get_game_directory(&slug, 10, true).await?;
        let first_stream = game_directory.first().unwrap();
        println!("Watching stream: {} (ID: {})", first_stream.broadcaster.login, first_stream.id);
        loop {
            client.send_watch(&first_stream.broadcaster.login, &first_stream.id, &first_stream.broadcaster.id, Some(&first_stream.game.name), Some(&first_stream.game.id)).await?;
            println!("Sent watch event for stream: {} (ID: {})", first_stream.broadcaster.login, first_stream.id);
            sleep(Duration::from_secs(59)).await;
        }
    }
}
