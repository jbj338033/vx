use crate::error::VxError;
use crate::ffmpeg::progress::ProgressDisplay;
use anyhow::Result;
use ffmpeg_sidecar::command::FfmpegCommand;
use ffmpeg_sidecar::event::{FfmpegEvent, LogLevel};
use std::path::Path;

pub struct FfmpegRunner {
    cmd: FfmpegCommand,
    progress: Option<ProgressDisplay>,
}

impl FfmpegRunner {
    pub fn new() -> Result<Self> {
        if !ffmpeg_sidecar::command::ffmpeg_is_installed() {
            return Err(VxError::FfmpegNotFound.into());
        }

        let cmd = FfmpegCommand::new();
        Ok(Self { cmd, progress: None })
    }

    pub fn input(mut self, path: &Path) -> Self {
        self.cmd.input(path.to_string_lossy());
        self
    }

    pub fn output(mut self, path: &Path) -> Self {
        self.cmd.output(path.to_string_lossy());
        self
    }

    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for arg in args {
            self.cmd.arg(arg.as_ref());
        }
        self
    }

    pub fn overwrite(mut self) -> Self {
        self.cmd.overwrite();
        self
    }

    pub fn with_progress(mut self, description: &str, duration: Option<f64>) -> Self {
        self.progress = Some(ProgressDisplay::new(description, duration));
        self
    }

    pub fn run(mut self) -> Result<()> {
        let mut child = self.cmd.spawn()?;

        let iter = child.iter()?;
        let mut last_error: Option<String> = None;

        for event in iter {
            match event {
                FfmpegEvent::Progress(p) => {
                    if let Some(ref progress) = self.progress {
                        if let Some(ms) = parse_time_to_ms(&p.time) {
                            progress.update(ms);
                        }
                    }
                }
                FfmpegEvent::Log(level, msg) => {
                    if matches!(level, LogLevel::Error | LogLevel::Fatal) {
                        last_error = Some(msg);
                    }
                }
                FfmpegEvent::Done => {
                    if let Some(ref progress) = self.progress {
                        progress.finish();
                    }
                }
                _ => {}
            }
        }

        if let Some(err) = last_error {
            return Err(VxError::FfmpegError(err).into());
        }

        Ok(())
    }
}

fn parse_time_to_ms(time_str: &str) -> Option<u64> {
    let parts: Vec<&str> = time_str.split(':').collect();
    match parts.as_slice() {
        [hours, mins, secs] => {
            let hours: f64 = hours.parse().ok()?;
            let mins: f64 = mins.parse().ok()?;
            let secs: f64 = secs.parse().ok()?;
            Some(((hours * 3600.0 + mins * 60.0 + secs) * 1000.0) as u64)
        }
        [mins, secs] => {
            let mins: f64 = mins.parse().ok()?;
            let secs: f64 = secs.parse().ok()?;
            Some(((mins * 60.0 + secs) * 1000.0) as u64)
        }
        _ => None,
    }
}

pub fn get_video_duration(path: &Path) -> Result<f64> {
    let mut cmd = FfmpegCommand::new();
    cmd.input(path.to_string_lossy());
    cmd.args(["-f", "null", "-"]);

    let mut child = cmd.spawn()?;
    let iter = child.iter()?;

    let mut duration: Option<f64> = None;

    for event in iter {
        match event {
            FfmpegEvent::ParsedDuration(d) => {
                duration = Some(d.duration);
            }
            FfmpegEvent::ParsedInput(input) => {
                if duration.is_none() {
                    duration = input.duration;
                }
            }
            _ => {}
        }
    }

    Ok(duration.unwrap_or(0.0))
}

pub struct VideoInfo {
    pub duration: f64,
    pub width: u32,
    pub height: u32,
    pub fps: f32,
    pub video_codec: String,
    pub audio_codec: Option<String>,
    pub file_size: u64,
    pub bitrate: u64,
}

pub fn get_video_info(path: &Path) -> Result<VideoInfo> {
    let metadata = std::fs::metadata(path)?;

    let mut cmd = FfmpegCommand::new();
    cmd.input(path.to_string_lossy());
    cmd.args(["-f", "null", "-"]);

    let mut child = cmd.spawn()?;
    let iter = child.iter()?;

    let mut duration = 0.0f64;
    let mut width = 0u32;
    let mut height = 0u32;
    let mut fps = 0.0f32;
    let mut video_codec = String::new();
    let mut audio_codec: Option<String> = None;

    for event in iter {
        match event {
            FfmpegEvent::ParsedDuration(d) => {
                duration = d.duration;
            }
            FfmpegEvent::ParsedInput(input) => {
                if duration == 0.0 {
                    duration = input.duration.unwrap_or(0.0);
                }
            }
            FfmpegEvent::ParsedInputStream(stream) => {
                if let Some(video) = stream.video_data() {
                    width = video.width;
                    height = video.height;
                    fps = video.fps;
                    video_codec = stream.format.clone();
                } else if stream.is_audio() && audio_codec.is_none() {
                    audio_codec = Some(stream.format.clone());
                }
            }
            _ => {}
        }
    }

    let file_size = metadata.len();
    let bitrate = if duration > 0.0 {
        ((file_size as f64 * 8.0) / duration) as u64
    } else {
        0
    };

    Ok(VideoInfo {
        duration,
        width,
        height,
        fps,
        video_codec,
        audio_codec,
        file_size,
        bitrate,
    })
}
