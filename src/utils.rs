use crate::error::VxError;
use std::io::{self, Write};
use std::path::Path;

pub fn parse_time(s: &str) -> Result<f64, VxError> {
    if let Ok(secs) = s.parse::<f64>() {
        return Ok(secs);
    }

    let parts: Vec<&str> = s.split(':').collect();
    match parts.as_slice() {
        [mins, secs] => {
            let mins: f64 = mins.parse().map_err(|_| VxError::InvalidTime(s.to_string()))?;
            let secs: f64 = secs.parse().map_err(|_| VxError::InvalidTime(s.to_string()))?;
            Ok(mins * 60.0 + secs)
        }
        [hours, mins, secs] => {
            let hours: f64 = hours.parse().map_err(|_| VxError::InvalidTime(s.to_string()))?;
            let mins: f64 = mins.parse().map_err(|_| VxError::InvalidTime(s.to_string()))?;
            let secs: f64 = secs.parse().map_err(|_| VxError::InvalidTime(s.to_string()))?;
            Ok(hours * 3600.0 + mins * 60.0 + secs)
        }
        _ => Err(VxError::InvalidTime(s.to_string())),
    }
}

pub fn format_duration(secs: f64) -> String {
    let total_secs = secs as u64;
    let hours = total_secs / 3600;
    let mins = (total_secs % 3600) / 60;
    let s = total_secs % 60;
    let millis = ((secs - secs.floor()) * 100.0) as u64;

    if hours > 0 {
        format!("{hours}:{mins:02}:{s:02}.{millis:02}")
    } else {
        format!("{mins}:{s:02}.{millis:02}")
    }
}

pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} B")
    }
}

pub fn format_bitrate(bps: u64) -> String {
    const KBPS: u64 = 1000;
    const MBPS: u64 = KBPS * 1000;

    if bps >= MBPS {
        format!("{:.1} Mbps", bps as f64 / MBPS as f64)
    } else if bps >= KBPS {
        format!("{:.1} kbps", bps as f64 / KBPS as f64)
    } else {
        format!("{bps} bps")
    }
}

pub fn confirm_overwrite(path: &Path) -> bool {
    if !path.exists() {
        return true;
    }

    print!("{} already exists. Overwrite? [y/N] ", path.display());
    io::stdout().flush().ok();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return false;
    }

    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}

pub fn default_output_path(input: &Path, suffix: Option<&str>, new_ext: &str) -> std::path::PathBuf {
    let stem = input.file_stem().unwrap_or_default().to_string_lossy();
    let parent = input.parent().unwrap_or(Path::new("."));

    let filename = match suffix {
        Some(s) => format!("{stem}{s}.{new_ext}"),
        None => format!("{stem}.{new_ext}"),
    };

    parent.join(filename)
}
