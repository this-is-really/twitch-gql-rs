use serde::{Deserialize, Serialize};

//game_directory
/// Represents a Twitch stream in the game directory.
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
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


/// Information about the broadcaster (channel owner) for a stream
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Broadcaster {
    pub id: String,
    pub login: String,
    pub displayName: String,
    pub profileImageURL: String,
}

/// Basic game metadata shown for a stream in the directory.
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Game {
    pub id: String,
    pub name: String,
    pub displayName: Option<String>,
    pub slug: String,
    pub boxArtURL: String
}

//playback_access_token
/// Represents a playback access token for a Twitch stream.
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
//main
pub struct PlaybackAccessToken {
    pub signature: String,
    pub value: String
}

//available_drops
/// Represents the response containing the list of available Drops for a user
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
//main
pub struct AvailableDrops {
    pub id: String,
    pub viewerDropCampaigns: Option<Vec<ViewerDropCampaigns>>
}

/// A single viewer-facing drop campaign (summary) available to watch/claim.
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ViewerDropCampaigns {
    pub id: String,
    pub name: String,
    pub detailsURL: String,
    pub imageURL: String,
    pub endAt: String,
    pub timeBasedDrops: Vec<TimeBasedDrops>,
    pub game: GameDrops
}

/// Minimal game info attached to drops (id + name)
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct GameDrops {
    pub id: String,
    pub name: String
}

/// A time-based drop tier inside a campaign, includes required watch time and benefits
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct TimeBasedDrops {
    pub id: String,
    pub name: String,
    pub startAt: String,
    pub endAt: String,
    pub requiredMinutesWatched: u64,
    pub benefitEdges: Vec<BenefitEdge>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct BenefitEdge {
    pub benefit: Benefit
}


/// A benefit / reward that a viewer can earn from a drop
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Benefit {
    pub id: String,
    pub name: String,
    pub imageAssetURL: String,
    pub game: GameDrops,
}

//get_campaign
/// Represents a user's drops overview - campaigns associated with the user.
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
//main
pub struct Drops {
    #[serde(rename = "id")]
    pub user_id: String,
    pub login: String,
    pub dropCampaigns: Vec<DropCampaigns>
}

/// Summary of a drop campaign
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
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

/// Owner information for a campaign
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Owner {
    pub id: String,
    pub name: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct CampaignGame {
    pub id: String,
    pub displayName: String,
    pub boxArtURL: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct CampaignSelf {
    pub isAccountConnected: bool
}

//get_campaign_details
/// Full details for a specific drop campaign
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
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

/// Flags whether campaign features are allowed/enabled and which channels are permitted.
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Allow {
    pub isEnabled: bool,
    pub channels: Option<Vec<Channels>>
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Channels {
    pub id: String,
    pub name: String,
    pub displayName: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct CampaignDetailsGame {
    pub id: String,
    pub slug: String,
    pub displayName: String
}

/// Time-based drop detail inside the campaign with additional gating like required subs.
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
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
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct CampaignDetailsBenefitsEdges {
    pub entitlementLimit: u64,
    pub benefit: CampaignDetailsBenefits,
}

/// Detailed information about a specific campaign benefit
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
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
/// Shows the current drop progress for the requesting user on a given channel.
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct CurrentDrop {
    pub channel: Option<Channels>,
    pub currentMinutesWatched: u64,
    pub dropID: String,
    pub game: Option<CurrentGame>,
    pub requiredMinutesWatched: u64,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct CurrentGame {
    displayName: String,
    id: String
}

//get_stream_info
/// Detailed stream info for a broadcaster - profile, broadcast settings and live stream data.
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
//main
pub struct StreamInfo {
    pub broadcastSettings: BroadcastSettings,
    pub displayName: String,
    pub id: String,
    pub login: String,
    pub profileImageURL: String,
    pub profileURL: String,
    pub stream: Option<Stream>
}

/// Broadcast settings such as title and game selection for the current broadcast.
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct BroadcastSettings {
    pub id: String,
    pub title: String,
    pub game: StreamGame
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct StreamGame {
    pub displayName: String,
    pub slug: String,
    pub name: String,
    pub id: String
}

/// Lightweight live stream data (viewers count, tags, id).
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Stream {
    pub id: String,
    pub viewersCount: u64,
    pub tags: Vec<String>
}

//get_inventory
/// User inventory response - all drops and progress tied to a user account.
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
//main
pub struct GetInventory {
    pub id: String,
    pub inventory: Inventory
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Inventory {
    pub dropCampaignsInProgress: Vec<DropCampaignsInProgress>
}

/// A campaign that the user is currently participating in (in-progress info).
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct DropCampaignsInProgress {
    pub id: String,
    pub detailsURL: String,
    pub accountLinkURL: String,
    pub startAt: String,
    pub endAt: String,
    pub imageURL: String,
    pub name: String,
    pub status: String,
    #[serde(rename = "self")]
    pub drop_self: CampaignSelf,
    pub game: InventoryGame,
    pub allow: InventoryAllow,
    pub timeBasedDrops: Vec<InventoryTimeBasedDrops>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct InventoryAllow {
    pub channels: Option<Vec<Channels>>
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct InventoryGame {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub boxArtURL: String,
}

/// Progress-specific data for a time-based drop inside the user's inventory.
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct InventoryTimeBasedDrops {
    pub id: String,
    pub name: String,
    pub startAt: String,
    pub endAt: String,
    pub requiredMinutesWatched: u64,
    pub benefitEdges: Vec<InventoryBenefitEdge>,
    pub requiredSubs: u64,
    pub campaign: InventoryCampaign,
    #[serde(rename = "self")]
    pub self_drop: InventorySelf
}


/// Tracks the user's current state for a specific inventory drop (minutes watched, claimed, etc.).
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct InventorySelf {
    pub hasPreconditionsMet: bool,
    pub currentMinutesWatched: u64,
    pub currentSubs: u64,
    pub isClaimed: bool,
    pub dropInstanceID: Option<String>
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct InventoryBenefitEdge {
    pub benefit: InventoryBenefit
}

/// Benefit metadata inside the inventory
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct InventoryBenefit {
    pub id: String,
    pub name: String,
    pub imageAssetURL: String,
    pub distributionType: String
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct InventoryCampaign {
    pub id: String,
    pub detailsURL: String,
    pub accountLinkURL: String,
    #[serde(rename = "self")]
    pub campaign_self: CampaignSelf
}


//claim_drop
/// Response returned when attempting to claim a drop
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
//main
pub struct ClaimDrop {
    pub isUserAccountConnected: bool,
    pub status: String,
    pub dropType: DropType
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct DropType {
    pub campaign: ClaimCampaign,
    pub id: String
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ClaimCampaign {
    pub detailsURL: String,
    pub id: String,
    pub status: Option<String>,
}