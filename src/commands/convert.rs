use crate::cli::ConvertArgs;
use crate::error::{VxError, SUPPORTED_FORMATS};
use crate::ffmpeg::{get_video_duration, FfmpegRunner};
use crate::utils::{confirm_overwrite, default_output_path};
use anyhow::Result;

pub fn execute(args: ConvertArgs) -> Result<()> {
    let format = args.format.to_lowercase();

    if !SUPPORTED_FORMATS.contains(&format.as_str()) {
        return Err(VxError::UnsupportedFormat {
            format,
            supported: SUPPORTED_FORMATS.to_vec(),
        }
        .into());
    }

    if !args.input.exists() {
        return Err(VxError::InputNotFound(args.input).into());
    }

    let output = args.output.unwrap_or_else(|| default_output_path(&args.input, None, &format));

    if !args.force && !confirm_overwrite(&output) {
        return Err(VxError::Cancelled.into());
    }

    let duration = get_video_duration(&args.input)?;

    let description = format!(
        "Converting {} → {}",
        args.input.file_name().unwrap_or_default().to_string_lossy(),
        output.file_name().unwrap_or_default().to_string_lossy()
    );

    let codec_args = get_codec_args(&format);

    FfmpegRunner::new()?
        .with_progress(&description, Some(duration))
        .input(&args.input)
        .args(codec_args)
        .output(&output)
        .overwrite()
        .run()?;

    println!("Created: {}", output.display());
    Ok(())
}

fn get_codec_args(format: &str) -> Vec<&'static str> {
    match format {
        "mp4" => vec!["-c:v", "libx264", "-c:a", "aac"],
        "webm" => vec!["-c:v", "libvpx-vp9", "-c:a", "libopus"],
        "mov" => vec!["-c:v", "libx264", "-c:a", "aac"],
        "avi" => vec!["-c:v", "mpeg4", "-c:a", "libmp3lame"],
        "gif" => vec![
            "-vf",
            "fps=10,scale=480:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse",
        ],
        _ => vec![],
    }
}
