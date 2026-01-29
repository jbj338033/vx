use crate::cli::CompressArgs;
use crate::error::VxError;
use crate::ffmpeg::{get_video_duration, FfmpegRunner};
use crate::utils::{confirm_overwrite, default_output_path};
use anyhow::Result;

pub fn execute(args: CompressArgs) -> Result<()> {
    if !args.input.exists() {
        return Err(VxError::InputNotFound(args.input).into());
    }

    let ext = args
        .input
        .extension()
        .map(|e| e.to_string_lossy().to_string())
        .unwrap_or_else(|| "mp4".to_string());

    let output = args
        .output
        .unwrap_or_else(|| default_output_path(&args.input, Some("_compressed"), &ext));

    if !args.force && !confirm_overwrite(&output) {
        return Err(VxError::Cancelled.into());
    }

    let duration = get_video_duration(&args.input)?;

    let description = format!(
        "Compressing {} → {}",
        args.input.file_name().unwrap_or_default().to_string_lossy(),
        output.file_name().unwrap_or_default().to_string_lossy()
    );

    let crf = args.quality.crf().to_string();

    FfmpegRunner::new()?
        .with_progress(&description, Some(duration))
        .input(&args.input)
        .args([
            "-c:v", "libx264",
            "-crf", &crf,
            "-preset", "medium",
            "-c:a", "aac",
            "-b:a", "128k",
            "-movflags", "+faststart",
        ])
        .output(&output)
        .overwrite()
        .run()?;

    println!("Created: {}", output.display());
    Ok(())
}
