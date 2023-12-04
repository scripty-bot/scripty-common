use serde::{Deserialize, Serialize};

/// Client wants to use espeak-ng as the TTS engine
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct EspeakNgData {
    /// The voice to use
    pub voice: String,
}
