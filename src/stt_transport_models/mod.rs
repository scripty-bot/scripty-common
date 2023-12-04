use serde::{Deserialize, Serialize};
use uuid::{serde::compact, Uuid};

#[repr(u8)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ClientToServerMessage {
    InitializeStreaming(InitializeStreaming) = 0x00,
    AudioData(AudioData) = 0x01,
    FinalizeStreaming = 0x02,
    CloseConnection = 0x03,
    ConvertToStatus = 0x04,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InitializeStreaming {
    pub verbose: bool,
    pub language: String,
    #[serde(with = "compact")]
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioData {
    pub data: Vec<i16>,
    #[serde(with = "compact")]
    pub id: Uuid,
}

#[repr(u8)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServerToClientMessage {
    InitializationComplete(InitializationComplete) = 0x00,
    InitializationFailed(InitializationFailed) = 0x01,
    SttResult(SttResult) = 0x02,
    SttVerboseResult(SttVerboseResult) = 0x03,
    SttError(SttError) = 0x04,
    ShuttingDown = 0x05,
    StatusConnectionOpen(StatusConnectionOpen) = 0x06,
    StatusConnectionData(StatusConnectionData) = 0x07,
    FatalIoError(FatalIoError) = 0xFD,
    FatalUnknownError(FatalUnknownError) = 0xFF,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InitializationComplete {
    #[serde(with = "compact")]
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InitializationFailed {
    #[serde(with = "compact")]
    pub id: Uuid,
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SttResult {
    #[serde(with = "compact")]
    pub id: Uuid,
    pub result: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SttVerboseResult {
    #[serde(with = "compact")]
    pub id: Uuid,
    /// Number of transcripts in the result. If 0, no other fields are set.
    pub num_transcripts: u32,
    pub main_transcript: Option<String>,
    pub confidence: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SttError {
    #[serde(with = "compact")]
    pub id: Uuid,
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatusConnectionOpen {
    max_utilization: f64,
    can_overload: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatusConnectionData {
    pub utilization: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FatalIoError {
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FatalUnknownError {
    pub error: String,
}
