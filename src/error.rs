use thiserror::Error;
#[derive(Error, Debug)]
pub enum SystemError {
    #[error("The file already exists")]
    FileAlredyExists,
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
    #[error("The specified channel does not exist or another error occurred.")]
    ChannelNotFound,
    #[error("The specified campaign does not exist or another error occurred.")]
    CampaignNotFound,
}