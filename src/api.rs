use std::time::Duration;

use base64::Engine;
use regex::Regex;
use reqwest::Client;
use serde_json::{json, Value};
use tokio::{sync::{watch::{self, Sender}}, time::sleep};

use crate::{error::TwitchError, gql::{playback_access_token}};

const DEVICE_URL: &'static str = "https://id.twitch.tv/oauth2/device";
const TOKEN_URL: &'static str = "https://id.twitch.tv/oauth2/token";

#[derive(Debug, Clone)]
pub struct DeviceAuth {
    device_code: String,
    pub user_code: String,
    interval: u64,
    pub verification_uri: String
}

async fn validate (client: &Client, oauth: &str) -> Result<(String, String), TwitchError> {
    let url = "https://id.twitch.tv/oauth2/validate";
    let get_validate = client.get(url).header("Authorization", format!("OAuth {}", oauth)).send().await?;
    let get_validate: Value = get_validate.json().await?;
    if let Some(user_id) = get_validate.get("user_id").and_then(|s| s.as_str()) {
        if let Some(login) = get_validate.get("login").and_then(|s| s.as_str()) {
            return Ok((user_id.to_string(), login.to_string()));
        } else {
            return Err(TwitchError::TwitchError("Not found login".into()));
        }
    } else {
        return Err(TwitchError::TwitchError("Not found user_id".into()));
    }
}

pub async fn request_device_auth (client: &Client, client_id: &str) -> Result<DeviceAuth, TwitchError> {
    let payload = [
        ("client_id", client_id),
        ("scopes", "")
    ];
    let response = client.post(DEVICE_URL).form(&payload).send().await?;
    if !response.status().is_success() {
        return Err(TwitchError::HttpError(response.status().as_u16()));
    }
    let response: Value = response.json().await?;
    let device_code = response.get("device_code").and_then(|s| s.as_str()).ok_or_else(|| TwitchError::MissingField("device_code".into()))?;
    let user_code = response.get("user_code").and_then(|s| s.as_str()).ok_or_else(|| TwitchError::MissingField("user_code".into()))?;
    let interval = response.get("interval").and_then(|s| s.as_u64()).ok_or_else(|| TwitchError::MissingField("interval".into()))?;
    let verification_uri = response.get("verification_uri").and_then(|s| s.as_str()).ok_or_else(|| TwitchError::MissingField("verification_uri".into()))?;
    Ok(DeviceAuth { device_code: device_code.to_string(), user_code: user_code.to_string(), interval: interval, verification_uri: verification_uri.to_string() })
}

pub async fn poll_device_auth (client: &Client, client_id: &str, device_auth: DeviceAuth) -> Result<(String, String, String), TwitchError> {
    let payload = [
        ("client_id", client_id),
        ("device_code", &device_auth.device_code),
        ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
    ];
    loop {
        let status = client.post(TOKEN_URL).form(&payload).send().await?;
        if status.status().is_success() && !status.status().as_u16() == 400 {
            return Err(TwitchError::HttpError(status.status().as_u16()));
        }
        let status: Value = status.json().await?;
        if let Some(access_token) = status.get("access_token").and_then(|s| s.as_str()) {
            let user = validate(&client, &access_token).await?;
            return Ok((access_token.to_string(), user.0, user.1));
        } else {
            sleep(Duration::from_secs(device_auth.interval)).await;
        }
    }
}

async fn _watch_stream (client: &Client, channel_login: &str) -> Result<(), TwitchError> {
    let playback = playback_access_token(client, channel_login).await?;
    let url = format!("https://usher.ttvnw.net/api/channel/hls/{}.m3u8", channel_login);
    let get_available_qualities = client.get(url).query(&[("sig", &playback.signature), ("token", &playback.value)]).send().await?;
    let text = get_available_qualities.text().await?;
    if let Ok(json) = serde_json::from_str::<Value>(&text) {
        if let Some(error) = json.get(0).and_then(|s| s.get("error")) {
            return Err(TwitchError::TwitchError(error.to_string()));
        }
    }
    println!("{}", text);
    Ok(())
}

async fn get_spade_url (spade: &str, client: &Client, tx: Sender<String>) -> Result<(), TwitchError> {
    let settings_pattern = Regex::new(r#"src="(https://[\w.]+/config/settings\.[0-9a-f]{32}\.js)""#).unwrap();
    let spade_pattern = Regex::new(r#""spade_?url": ?"(https://video-edge-[.\w\-/]+\.ts(?:\?allow_stream=true)?)""#).unwrap();
    
    if let Some(caps) = spade_pattern.captures(&spade) {
        let spade_url = caps.get(1).unwrap().as_str();
        tx.send(spade_url.to_string()).unwrap();
        return Ok(());
    }
    if let Some(caps) = settings_pattern.captures(&spade) {
        let settings_url = caps.get(1).unwrap().as_str();
        let settings_js = client.get(settings_url).send().await?.text().await?;
        if let Some(caps2) = spade_pattern.captures(&settings_js) {
            let spade_url = caps2.get(1).unwrap().as_str();
            tx.send(spade_url.to_string()).unwrap();
            return Ok(());
        } else {
            return Err(TwitchError::TwitchError("Error while spade_url extraction: step #2".into()));
        }
    } else {
        return Err(TwitchError::TwitchError("Error while spade_url extraction: step #1".into()));
    }
}

pub async fn send_watch (client: &Client, user_id: &str, client_url: &str, channel_login: &str, broadcast_id: &str, channel_id: &str) -> Result<(), TwitchError> {
    let url = format!("{}/{}", client_url, channel_login);
    let get_spade = client.get(url).send().await?;
    let spade = get_spade.text().await?;
    let (tx, mut rx) = watch::channel(String::new());
    get_spade_url(&spade, &client, tx).await?;
    rx.changed().await.unwrap();
    let spade_url = rx.borrow().clone();
    drop(rx);
    let payload = json!([
        {
            "event": "minute-watched",
            "properties": {
                "broadcast_id": broadcast_id,
                "channel_id": channel_id,
                "channel": channel_login,
                "hidden": false,
                "live": true,
                "location": "channel",
                "logged_in": true,
                "muted": false,
                "player": "site",
                "user_id": user_id,
            }
        }
    ]);
    let payload = serde_json::to_string(&payload)?;
    let base64 = base64::engine::general_purpose::STANDARD.encode(&payload);
    let send_watch = client.post(spade_url.to_string()).form(&[("data", base64)]).send().await?;
    let status = send_watch.status();
    if status == 204 {
        return Ok(());
    } else {
        return Err(TwitchError::HttpError(status.as_u16()));
    }
}