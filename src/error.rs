use thiserror::Error;
#[derive(Error, Debug)]
pub enum SystemError {
    #[error("The file already exists")]
    FileAlreadyExists,
    #[error("An error occurred during serialization: {0}")]
    SerializationProblem(serde_json::Error),
    #[error("An error occurred during deserialization: {0}")]
    DeserializationProblem(serde_json::Error),
    #[error("File not found")]
    FileNotFound,
    #[error("Failed to retrieve the required headers: {0}")]
    HeadersError(#[from] Box<dyn std::error::Error>),
    #[error("Error creating client: {0}")]
    ClientBuilderError(#[from] reqwest::Error),
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("{0}")]
    TwitchError(#[from] TwitchError),
    #[error("The device authorization code has expired")]
    DeviceTokenExpired
}

impl From<reqwest::Error> for AuthError {
    fn from(e: reqwest::Error) -> Self {
        AuthError::TwitchError(e.into())
    }
}

impl From<serde_json::Error> for AuthError {
    fn from(e: serde_json::Error) -> Self {
        AuthError::TwitchError(e.into())
    }
}

#[derive(Debug, Error)]
pub enum TwitchError {
    #[error("Twitch response JSON is missing the field: {0}")]
    MissingField(String),
    #[error("Error Twitch response JSON: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("HTTP error: {0}")]
    HttpError(u16),
    #[error("Failed reqwest Twitch response: {0}")]
    ReqwestProblem(#[from] reqwest::Error),
    #[error("Twitch error: {0}")]
    TwitchError(String),
}

#[derive(Debug, Error)]
pub enum ClaimDropError {
    #[error("Failed to claim drop: {0}")]
    FailedClaimDrops(String),
    #[error("Drop already claimed")]
    DropAlreadyClaimed,
    #[error("{0}")]
    TwitchError(#[from] TwitchError)
}

impl From<reqwest::Error> for ClaimDropError {
    fn from(e: reqwest::Error) -> Self {
        ClaimDropError::TwitchError(e.into())
    }
}

impl From<serde_json::Error> for ClaimDropError {
    fn from(e: serde_json::Error) -> Self {
        ClaimDropError::TwitchError(e.into())
    }
}

#[derive(Debug, Error)]
pub enum SlugError {
    #[error("Failed to parse game slug from GraphQL response.")]
    GameSlugParsingFailed,
    #[error("{0}")]
    TwitchError(#[from] TwitchError)
}

impl From<reqwest::Error> for SlugError {
    fn from(e: reqwest::Error) -> Self {
        SlugError::TwitchError(e.into())
    }
}

impl From<serde_json::Error> for SlugError {
    fn from(e: serde_json::Error) -> Self {
        SlugError::TwitchError(e.into())
    }
}

#[derive(Debug, Error)]
pub  enum CampaignDetailsError {
    #[error("The specified campaign does not exist or another error occurred.")]
    CampaignNotFound,
    #[error("{0}")]
    TwitchError(#[from] TwitchError)
}

impl From<reqwest::Error> for CampaignDetailsError {
    fn from(e: reqwest::Error) -> Self {
        CampaignDetailsError::TwitchError(e.into())
    }
}

impl From<serde_json::Error> for CampaignDetailsError {
    fn from(e: serde_json::Error) -> Self {
        CampaignDetailsError::TwitchError(e.into())
    }
}

#[derive(Debug, Error)]
pub enum StreamInfoError {
    #[error("The specified channel does not exist or another error occurred.")]
    ChannelNotFound,
    #[error("{0}")]
    TwitchError(#[from] TwitchError)
}

impl From<reqwest::Error> for StreamInfoError {
    fn from(e: reqwest::Error) -> Self {
        StreamInfoError::TwitchError(e.into())
    }
}

impl From<serde_json::Error> for StreamInfoError {
    fn from(e: serde_json::Error) -> Self {
        StreamInfoError::TwitchError(e.into())
    }
}

#[derive(Debug, Error)]
pub enum GameDirectoryError {
    #[error("No streams found for the game slug: {0}")]
    NoStreamsFound(String),
    #[error("{0}")]
    TwitchError(#[from] TwitchError),
}

impl From<reqwest::Error> for GameDirectoryError {
    fn from(e: reqwest::Error) -> Self {
        GameDirectoryError::TwitchError(e.into())
    }
}

impl From<serde_json::Error> for GameDirectoryError {
    fn from(e: serde_json::Error) -> Self {
        GameDirectoryError::TwitchError(e.into())
    }
}

#[derive(Debug, Error)]
pub enum AvailableDropsError {
    #[error("The specified channel does not exist or another error occurred.")]
    ChannelNotFound,
    #[error("{0}")]
    TwitchError(#[from] TwitchError),
}

impl From<reqwest::Error> for AvailableDropsError {
    fn from(e: reqwest::Error) -> Self {
        AvailableDropsError::TwitchError(e.into())
    }
}

impl From<serde_json::Error> for AvailableDropsError {
    fn from(e: serde_json::Error) -> Self {
        AvailableDropsError::TwitchError(e.into())
    }
}