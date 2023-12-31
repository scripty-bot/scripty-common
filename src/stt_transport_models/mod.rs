use serde::{Deserialize, Serialize};
use uuid::{serde::compact, Uuid};

#[repr(u8)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ClientToServerMessage {
    InitializeStreaming(InitializeStreaming) = 0x00,
    AudioData(AudioData) = 0x01,
    FinalizeStreaming(FinalizeStreaming) = 0x02,
    CloseConnection = 0x03,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InitializeStreaming {
    #[serde(with = "compact")]
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioData {
    pub data: Vec<i16>,
    #[serde(with = "compact")]
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FinalizeStreaming {
    pub verbose: bool,
    pub language: String,
    pub translate: bool,
    #[serde(with = "compact")]
    pub id: Uuid,
}

#[repr(u8)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServerToClientMessage {
    InitializationComplete(InitializationComplete) = 0x00,
    InitializationFailed(InitializationFailed) = 0x01,
    SttResult(SttSuccess) = 0x02,
    SttError(SttError) = 0x03,
    ShuttingDown = 0x04,
    StatusConnectionOpen(StatusConnectionOpen) = 0x05,
    StatusConnectionData(StatusConnectionData) = 0x06,
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
pub struct SttSuccess {
    #[serde(with = "compact")]
    pub id: Uuid,
    pub result: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SttError {
    #[serde(with = "compact")]
    pub id: Uuid,
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatusConnectionOpen {
    pub max_utilization: f64,
    pub can_overload: bool,
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
