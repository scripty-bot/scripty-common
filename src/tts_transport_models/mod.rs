use serde::{Deserialize, Serialize};
use uuid::{serde::compact, Uuid};

/// Message sent from the client to the server
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ClientMessage {
    /// Request a TTS session
    RequestSession(RequestSession),
}

/// Message sent from the server to the client
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ServerMessage {
    /// Approve a TTS session
    ApproveSession(ApproveSession),
}

/// Request a TTS session
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RequestSession {
    /// An internal identifier for the session
    #[serde(with = "compact")]
    pub id: Uuid,

    /// The text to speak
    pub text: String,

    /// Language to use
    pub language: String,

    /// Per-service configuration (ie voice, pitch, etc)
    pub config: TtsService,
}

/// Server response to a session request
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ApproveSession {
    /// The session identifier
    #[serde(with = "compact")]
    pub id: Uuid,
}

/// Server response upon successful audio completion
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SessionComplete {
    /// The session identifier
    #[serde(with = "compact")]
    pub id: Uuid,

    /// The audio data
    pub audio: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum TtsService {
    EspeakNg(espeak_ng::EspeakNgData),
    Flite(flite::FliteData),
    MaryTts(mary_tts::MaryTtsData),
}

mod espeak_ng;
mod flite;
mod mary_tts;
