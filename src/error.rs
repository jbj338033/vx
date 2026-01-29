use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VxError {
    #[error("Input file not found: {0}")]
    InputNotFound(PathBuf),

    #[error("Output format not supported: {format}\n  Supported formats: {}", .supported.join(", "))]
    UnsupportedFormat { format: String, supported: Vec<&'static str> },

    #[error("Invalid time format: \"{0}\"\n  Expected format: seconds (30) or mm:ss (1:30)")]
    InvalidTime(String),

    #[error("ffmpeg not found\n\nInstall ffmpeg first:\n  macOS   : brew install ffmpeg\n  Ubuntu  : sudo apt install ffmpeg\n  Windows : winget install ffmpeg")]
    FfmpegNotFound,

    #[error("ffmpeg error: {0}")]
    FfmpegError(String),

    #[error("Operation cancelled")]
    Cancelled,
}

pub const SUPPORTED_FORMATS: &[&str] = &["mp4", "webm", "mov", "avi", "gif"];
