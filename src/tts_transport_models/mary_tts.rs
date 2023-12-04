use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct MaryTtsData {
    /// Voice to use
    pub voice: String,

    /// Effects to use.
    pub effects: Vec<MaryTtsEffects>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum MaryTtsEffects {
    /// Scales the output volume by a fixed amount.
    Volume {
        /// Amount of scaling (the output is simply multiplied by this value)
        amount: f32,
    },

    /// Creates a shortened or lengthened vocal tract effect by shifting the formants.
    TractScaler {
        /// The amount of formant shifting.
        ///
        /// If < 1.0, the formants are shifted down, creating a longer vocal tract (ie deeper voice).
        /// If > 1.0, the formants are shifted up, creating a shorter vocal tract (ie higher voice).
        ///
        /// Range 0.25 to 4.0.
        amount: f32,
    },

    /// F0 scaling effect for HMM voices:
    /// All voiced f0 values are multiplied by <f0Scale> for HMM voices.
    /// This operation effectively scales the range of f0 values.
    /// Note that mean f0 is preserved during the operation.
    ///
    /// Operates on HMM-based voices only.
    F0Scale {
        /// Scale ratio for modifying the dynamic range of the f0 contour.
        ///
        /// If f0_scale > 1.0, the range is expanded (ie voice with more variable pitch).
        /// If f0_scale < 1.0, the range is compressed (ie voice with less variable pitch).
        ///
        /// Range 0.0 to 3.0
        f0_scale: f32,
    },

    /// F0 mean shifting effect for HMM voices:
    /// Shifts the mean F0 value by <f0Add> Hz for HMM voices.
    F0Add {
        /// Amount to shift the mean F0 value.
        ///
        /// Range -300.0 to 300.0
        f0_add: f32,
    },

    /// Duration scaling for HMM voices:
    /// Scales the HMM output speech duration by <duration_scale>.
    Rate {
        /// Duration scaling factor for synthesized speech output.
        ///
        /// Range 0.1 to 3.0
        duration_scale: f32,
    },

    /// Creates a robotic voice by setting all phases to zero.
    Robot {
        /// The amount of robotic voice at the output.
        ///
        /// Range 0.0 to 100.0
        amount: f32,
    },

    /// Creates a whispered voice by replacing the LPC residual with white noise.
    Whisper {
        /// The amount of whispering at the output.
        ///
        /// Range 0.0 to 100.0
        amount: f32,
    },

    /// Adds stadium effect by applying a specially designed multi-tap chorus.
    Stadium {
        /// The amount of stadium effect at the output.
        ///
        /// Range 0.0 to 200.0
        amount: f32,
    },

    /// Multi-Tap Chorus Effect:
    ///
    /// Adds chorus effect by summing up the original signal with delayed and amplitude scaled versions.
    /// The parameters should consist of delay and amplitude pairs for each tap.
    ///
    /// A variable number of taps (max 20) can be specified by simply defining more delay-amplitude pairs.
    ///
    /// Each tap outputs a delayed and gain-scaled version of the original signal.
    /// All tap outputs are summed up with the original signal with appropriate gain normalization.
    Chorus {
        /// The delay and amplitude pairs for each tap.
        ///
        /// Delay is specified in milliseconds, amplitude is relative to the input signal from the last tap.
        ///
        /// * Range for delay: 0.0 to 5000.0
        /// * Range for amplitude: -5.0 to 5.0
        /// * Maximum number of taps: 20
        delay_amp_pairs: Vec<(f32, f32)>,
    },

    /// Filters the input signal by an FIR filter.
    FirFilter {
        /// Type of filter.
        filter_type: MaryTtsFirFilter,

        /// Cutoff frequency in Hz for lowpass and highpass filters.
        ///
        /// Range: 0.0 to fs/2 where fs is the sampling frequency in Hz.
        cutoff_freq: f32,

        /// Lower frequency cutoff in Hz for bandpass and bandreject filters.
        ///
        /// Range: 0.0 to fs/2 where fs is the sampling frequency in Hz.
        lower_cutoff_freq: f32,

        /// Upper frequency cutoff in Hz for bandpass and bandreject filters.
        ///
        /// Range: 0.0 to fs/2 where fs is the sampling frequency in Hz.
        upper_cutoff_freq: f32,
    },

    /// Jet pilot effect:
    /// Filters the input signal using an FIR bandpass filter.
    JetPilot,
}

#[repr(u8)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum MaryTtsFirFilter {
    LowPass = 1,
    HighPass = 2,
    BandPass = 3,
    BandReject = 4,
}
