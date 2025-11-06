#![allow(dead_code)]
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{error::*, structs::*};

pub const GQL_URL: &'static str = "https://gql.twitch.tv/gql";

#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct GQLOperation {
    operationName: String,
    extensions: Extensions,
    #[serde(skip_serializing_if = "Option::is_none")]
    variables: Option<Value>,
}
#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
struct Extensions {
    persistedQuery: PersistedQuery
}
#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
struct PersistedQuery {
    version: u16,
    sha256Hash: String
}

impl GQLOperation {
    fn new (name: &str, sha256: &str) -> Self {
        let operation = GQLOperation {
            operationName: name.to_string(),
            extensions: Extensions { persistedQuery: PersistedQuery { version: 1, sha256Hash: sha256.to_string() } },
            variables: None,
        };
        operation
    } 
    fn with_variables <V: Serialize>(mut self, vars: V) -> Self {
        self.variables = Some(serde_json::to_value(vars).expect("serialize variables"));
        self
    }
}

async fn check_response_error (response: &Response) -> Result<(), TwitchError> {
    if !response.status().is_success() {
        return Err(TwitchError::HttpError(response.status().as_u16()));
    }
    Ok(())
}

fn get_value (v: Value, name: &str) -> Result<Value, TwitchError> {
    let get = v.get(&name).ok_or_else(|| TwitchError::MissingField(name.into()))?;
    Ok(get.clone())
}

fn get_value_from_vec (v: Value, name_vec: &[&str]) -> Result<Value, TwitchError> {
    let mut current = &v;
    for name in name_vec {
        current = current.get(&name).ok_or_else(|| TwitchError::MissingField(name.to_string()))?;
    }
    Ok(current.clone())
}

pub async fn stream_info (client: &Client, channel_login: &str) -> Result<StreamInfo, StreamInfoError> {
    let gql = GQLOperation::new("VideoPlayerStreamInfoOverlayChannel", "198492e0857f6aedead9665c81c5a06d67b25b58034649687124083ff288597d").with_variables(json!({
        "channel": channel_login
    }));
    let gql = client.post(GQL_URL).json(&gql).send().await?;
    check_response_error(&gql).await?;
    let gql: Value = gql.json().await?;
    let gql = get_value_from_vec(gql, &["data", "user"])?;
    let stream_info: StreamInfo = match serde_json::from_value(gql) {
        Ok(v) => v,
        Err(_) => return Err(StreamInfoError::ChannelNotFound)
    };
    Ok(stream_info)
}
pub async fn claim_drop (client: &Client, drop_instance_id: &str) -> Result<ClaimDrop, ClaimDropError> {
    let gql = GQLOperation::new("DropsPage_ClaimDropRewards", "a455deea71bdc9015b78eb49f4acfbce8baa7ccbedd28e549bb025bd0f751930").with_variables(json!({
        "input": {
            "dropInstanceID": drop_instance_id
        }
    }));
    let gql = client.post(GQL_URL).json(&gql).send().await?;
    check_response_error(&gql).await?;
    let gql: Value = gql.json().await?;
    if let Ok(claim_drop) = get_value_from_vec(gql.clone(), &["data", "claimDropRewards"]) {
        let claim_drop: ClaimDrop = serde_json::from_value(claim_drop)?;
        if claim_drop.status == "ELIGIBLE_FOR_ALL" {
            return Ok(claim_drop)
        } else if claim_drop.status == "DROP_INSTANCE_ALREADY_CLAIMED" {
            return Err(ClaimDropError::DropAlreadyClaimed);
        } else {
            if let Ok(error) = get_value_from_vec(gql, &["data", "error"]) {
                return Err(ClaimDropError::FailedClaimDrops(error.to_string()));
            } else {
                return Err(ClaimDropError::FailedClaimDrops("Missing error field".into()));
            }
        }
    } else {
        return Err(ClaimDropError::FailedClaimDrops("Missing claimDropRewards field".into()));
    }
    
}

pub async fn inventory (client: &Client) -> Result<GetInventory, TwitchError> {
    let gql = GQLOperation::new("Inventory", "d86775d0ef16a63a33ad52e80eaff963b2d5b72fada7c991504a57496e1d8e4b").with_variables(json!({
        "fetchRewardCampaigns": false
    }));
    let gql = client.post(GQL_URL).json(&gql).send().await?;
    check_response_error(&gql).await?;
    let gql: Value = gql.json().await?;
    let gql = get_value_from_vec(gql, &["data", "currentUser"])?;
    let inventory: GetInventory = serde_json::from_value(gql)?;
    Ok(inventory)
}

pub async fn current_drop (client: &Client, channel_login: &str, channel_id: &str) -> Result<CurrentDrop, TwitchError> {
    let gql = GQLOperation::new("DropCurrentSessionContext", "4d06b702d25d652afb9ef835d2a550031f1cf762b193523a92166f40ea3d142b").with_variables(json!({
        "channelID": channel_id,
        "channelLogin": channel_login,
    }));
    let gql = client.post(GQL_URL).json(&gql).send().await?;
    check_response_error(&gql).await?;
    let gql: Value = gql.json().await?;
    let gql = get_value_from_vec(gql, &["data", "currentUser", "dropCurrentSession"])?;
    let current: CurrentDrop = serde_json::from_value(gql)?;
    Ok(current)
}

pub async fn campaign (client: &Client) -> Result<Drops, TwitchError> {
    let gql = GQLOperation::new("ViewerDropsDashboard", "5a4da2ab3d5b47c9f9ce864e727b2cb346af1e3ea8b897fe8f704a97ff017619").with_variables(json!({
        "fetchRewardCampaigns": false
    }));
    let gql = client.post(GQL_URL).json(&gql).send().await?;
    check_response_error(&gql).await?;
    let gql: Value = gql.json().await?;
    let gql = get_value_from_vec(gql, &["data", "currentUser"])?;
    let drops: Drops = serde_json::from_value(gql)?;
    Ok(drops)
}

pub async fn campaign_details (client: &Client, user_login: &str, drop_id: &str) -> Result<CampaignDetails, CampaignDetailsError> {
    let gql = GQLOperation::new("DropCampaignDetails", "039277bf98f3130929262cc7c6efd9c141ca3749cb6dca442fc8ead9a53f77c1").with_variables(json!({
        "channelLogin": user_login,
        "dropID": drop_id,
    }));
    let gql = client.post(GQL_URL).json(&gql).send().await?;
    check_response_error(&gql).await?;
    let gql: Value = gql.json().await?;
    let details = get_value_from_vec(gql, &["data", "user", "dropCampaign"])?;
    let details: CampaignDetails = match serde_json::from_value(details) {
        Ok(v) => v,
        Err(_) => return Err(CampaignDetailsError::CampaignNotFound)
    };
    Ok(details)
}

pub async fn available_drops (client: &Client, channel_id: &str) -> Result<AvailableDrops, AvailableDropsError> {
    let gql = GQLOperation::new("DropsHighlightService_AvailableDrops", "9a62a09bce5b53e26e64a671e530bc599cb6aab1e5ba3cbd5d85966d3940716f").with_variables(json!({
        "channelID": channel_id
    }));
    let gql = client.post(GQL_URL).json(&gql).send().await?;
    check_response_error(&gql).await?;
    let gql: Value = gql.json().await?;
    let channel = match get_value_from_vec(gql, &["data", "channel"]) {
        Ok(value) => value,
        Err(_) => return Err(AvailableDropsError::ChannelNotFound)
    };
    let available_drops: AvailableDrops = serde_json::from_value(channel)?;
    Ok(available_drops)
}

pub async fn playback_access_token (client: &Client, channel_login: &str) -> Result<PlaybackAccessToken, TwitchError> {
    let gql = GQLOperation::new("PlaybackAccessToken", "ed230aa1e33e07eebb8928504583da78a5173989fadfb1ac94be06a04f3cdbe9").with_variables(json!({
        "isLive": true,
        "isVod": false,
        "login": channel_login,
        "platform": "web",
        "playerType": "site",
        "vodID": "",
    }));
    let gql = client.post(GQL_URL).json(&gql).send().await?;
    check_response_error(&gql).await?;
    let gql: Value = gql.json().await?;
    let playback = get_value_from_vec(gql, &["data", "streamPlaybackAccessToken"])?;
    let playback: PlaybackAccessToken = serde_json::from_value(playback)?;
    Ok(playback)
}

pub async fn game_directory (client: &Client, game_slug: &str, limit: u64, drops_enabled: bool) -> Result<Vec<GameDirectory>, GameDirectoryError> {
    let filters = if drops_enabled {
        ["DROPS_ENABLED"]
    } else {
        [""]
    };
    let gql = GQLOperation::new("DirectoryPage_Game", "98a996c3c3ebb1ba4fd65d6671c6028d7ee8d615cb540b0731b3db2a911d3649").with_variables(json!({
        "limit": limit,
        "slug": game_slug,
        "imageWidth": 50,
        "includeCostreaming": false,
        "options": {
            "broadcasterLanguages": [],
            "freeformTags": null,
            "includeRestricted": ["SUB_ONLY_LIVE"],
            "recommendationsContext": {"platform": "web"},
            "sort": "RELEVANCE",
            "systemFilters": filters,
            "tags": [],
            "requestID": "JIRA-VXP-2397",
        },
        "sortTypeIsRecency": false,
    }));
    let gql = client.post(GQL_URL).json(&gql).send().await?;
    check_response_error(&gql).await?;
    let gql: Value = gql.json().await?;
    let streams = match get_value_from_vec(gql, &["data", "game", "streams"]) {
        Ok(value) => value,
        Err(_) => return Err(GameDirectoryError::NoStreamsFound(game_slug.into()))
    };
    let edges_vec = streams.get("edges").and_then(|v| v.as_array()).ok_or_else(|| TwitchError::MissingField("edges".to_string()))?;
    let mut directory_vec = Vec::new();
    for edge in edges_vec {
        let edge = edge.get("node").ok_or_else(|| TwitchError::MissingField("edge".to_string()))?;
        let directory: GameDirectory = serde_json::from_value(edge.clone())?;
        directory_vec.push(directory);
    }
    Ok(directory_vec)
}

pub async fn slug_redirect (client: &Client, game_name: &str) -> Result<String, SlugError> {
    let gql = GQLOperation::new("DirectoryGameRedirect", "1f0300090caceec51f33c5e20647aceff9017f740f223c3c532ba6fa59f6b6cc").with_variables(json!({
        "name": game_name
    }));
    let gql = client.post(GQL_URL).json(&gql).send().await?;
    check_response_error(&gql).await?;
    let gql: Value = gql.json().await?;
    let slug = get_value_from_vec(gql, &["data", "game", "slug"])?;
    let slug = match slug.as_str() {
        Some(s) => s,
        None => return Err(SlugError::GameSlugParsingFailed)
    };
    Ok(slug.to_string())
}