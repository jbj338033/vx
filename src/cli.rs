use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "vx")]
#[command(version, about = "Simple ffmpeg wrapper for humans")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Convert video to GIF
    Gif(GifArgs),

    /// Compress video file
    Compress(CompressArgs),

    /// Convert video format
    To(ConvertArgs),

    /// Show video information
    Info(InfoArgs),
}

#[derive(Parser)]
pub struct GifArgs {
    /// Input video file
    pub input: PathBuf,

    /// Output file [default: {input_name}.gif]
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Width in pixels
    #[arg(short, long, default_value = "480")]
    pub width: u32,

    /// Frames per second
    #[arg(short, long, default_value = "10")]
    pub fps: u32,

    /// Start time (e.g., 0:30, 30)
    #[arg(short, long)]
    pub start: Option<String>,

    /// Duration in seconds
    #[arg(short, long)]
    pub duration: Option<f64>,

    /// Overwrite without confirmation
    #[arg(long)]
    pub force: bool,
}

#[derive(Parser)]
pub struct CompressArgs {
    /// Input video file
    pub input: PathBuf,

    /// Output file [default: {input_name}_compressed.{ext}]
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Quality level
    #[arg(short, long, value_enum, default_value = "medium")]
    pub quality: Quality,

    /// Overwrite without confirmation
    #[arg(long)]
    pub force: bool,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum Quality {
    Low,
    Medium,
    High,
}

impl Quality {
    pub fn crf(self) -> u8 {
        match self {
            Quality::Low => 28,
            Quality::Medium => 23,
            Quality::High => 18,
        }
    }
}

#[derive(Parser)]
pub struct ConvertArgs {
    /// Target format (mp4, webm, mov, avi, gif)
    pub format: String,

    /// Input video file
    pub input: PathBuf,

    /// Output file [default: {input_name}.{format}]
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Overwrite without confirmation
    #[arg(long)]
    pub force: bool,
}

#[derive(Parser)]
pub struct InfoArgs {
    /// Input video file
    pub input: PathBuf,
}
