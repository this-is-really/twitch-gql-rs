use serde::{Deserialize, Serialize};

//game_directory

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]
//main
pub struct GameDirectory {
    pub id: String,
    pub r#type: String,
    pub viewersCount: u64,
    pub title: String,
    pub previewImageURL: String,
    pub broadcaster: Broadcaster,
    pub game: Game,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Broadcaster {
    pub id: String,
    pub login: String,
    pub displayName: String,
    pub profileImageURL: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Game {
    pub id: String,
    pub name: String,
    pub displayName: String,
    pub slug: String,
    pub boxArtURL: String
}

//playback_access_token

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]
//main
pub struct PlaybackAccessToken {
    pub signature: String,
    pub value: String
}

//available_drops

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]

//main
pub struct AvailableDrops {
    pub id: String,
    pub viewerDropCampaigns: Vec<ViewerDropCampaigns>
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ViewerDropCampaigns {
    pub id: String,
    pub name: String,
    pub detailsURL: String,
    pub imageURL: String,
    pub endAt: String,
    pub timeBasedDrops: Vec<TimeBasedDrops>,
    pub game: GameDrops
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct GameDrops {
    pub id: String,
    pub name: String
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct TimeBasedDrops {
    pub id: String,
    pub name: String,
    pub startAt: String,
    pub endAt: String,
    pub requiredMinutesWatched: u64,
    pub benefitEdges: Vec<BenefitEdge>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct BenefitEdge {
    pub benefit: Benefit
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Benefit {
    pub id: String,
    pub name: String,
    pub imageAssetURL: String,
    pub game: GameDrops,
}

//get_campaign
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]
//main
pub struct Drops {
    #[serde(rename = "id")]
    pub user_id: String,
    pub login: String,
    pub dropCampaigns: Vec<DropCampaigns>
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct DropCampaigns {
    pub id: String,
    pub name: String,
    pub owner: Owner,
    pub game: CampaignGame,
    pub status: String,
    pub startAt: String,
    pub endAt: String,
    pub detailsURL: String,
    pub accountLinkURL: String,
    #[serde(rename = "self")]
    pub connecting: CampaignSelf,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Owner {
    pub id: String,
    pub name: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CampaignGame {
    pub id: String,
    pub displayName: String,
    pub boxArtURL: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CampaignSelf {
    pub isAccountConnected: bool
}

//get_campaign_details

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]
//main
pub struct CampaignDetails {
    pub id: String,
    pub name: String,
    pub description: String,
    pub imageURL: String,
    pub accountLinkURL: String,
    pub detailsURL: String,
    pub status: String,
    pub startAt: String,
    pub endAt: String,
    #[serde(rename = "self")]
    pub self_drop: CampaignSelf,
    pub allow: Allow,
    pub game: CampaignDetailsGame,
    pub owner: Owner,
    pub timeBasedDrops: Vec<TimeBasedDropsCampaignDetails>
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Allow {
    pub isEnabled: bool,
    pub channels: Option<Vec<Channels>>
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Channels {
    pub id: String,
    pub name: String,
    pub displayName: String
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CampaignDetailsGame {
    pub id: String,
    pub slug: String,
    pub displayName: String
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct TimeBasedDropsCampaignDetails {
    pub id: String,
    pub name: String,
    pub startAt: String,
    pub endAt: String,
    pub requiredMinutesWatched: u64,
    pub requiredSubs: u64,
    pub preconditionDrops: Option<String>,
    pub benefitEdges: Vec<CampaignDetailsBenefitsEdges>
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CampaignDetailsBenefitsEdges {
    pub entitlementLimit: u64,
    pub benefit: CampaignDetailsBenefits,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CampaignDetailsBenefits {
    pub id: String,
    pub name: String,
    pub createdAt: String,
    pub distributionType: String,
    pub entitlementLimit: u64,
    pub imageAssetURL: String,
    pub isIosAvailable: bool,
    pub game: GameDrops,
    pub ownerOrganization: Owner
}


//get_current_drop_progress_on_channel
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CurrentDrop {
    pub channel: Option<String>,
    pub currentMinutesWatched: u64,
    pub dropID: String,
    pub game: Option<String>,
    pub requiredMinutesWatched: u64,
}

//get_stream_info
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]
//main
pub struct StreamInfo {
    pub broadcastSettings: BroadcastSettings,
    pub displayName: String,
    pub id: String,
    pub login: String,
    pub profileImageURL: String,
    pub profileURL: String,
    pub stream: Stream
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct BroadcastSettings {
    pub id: String,
    pub title: String,
    pub game: StreamGame
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct StreamGame {
    pub displayName: String,
    pub slug: String,
    pub name: String,
    pub id: String
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Stream {
    pub id: String,
    pub viewersCount: u64,
    pub tags: Vec<String>
}