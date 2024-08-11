use std::fmt;

// Custom Errors for the Wav module
#[derive(Debug, Clone)]
pub enum WavError {
    FileNotEncodedProperly,
    // UnknownError,
}

impl std::error::Error for WavError {}

impl fmt::Display for WavError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WavError::FileNotEncodedProperly => write!(f, "invalid input"),
            // WavError::UnknownError => write!(f, "unknown error"),
        }
    }
}
