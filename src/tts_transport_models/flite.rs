use serde::{Deserialize, Serialize};

/// Client wants to use flite as the TTS engine
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct FliteData {
    /// The voice to use
    ///
    /// Should be one of supported voices returned by a prior fetch of the voices
    pub voice: String,

    /// The text to speak
    pub text: String,
    // flite does not support much more than this
}
