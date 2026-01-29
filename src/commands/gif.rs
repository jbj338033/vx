use crate::cli::GifArgs;
use crate::error::VxError;
use crate::ffmpeg::{get_video_duration, FfmpegRunner};
use crate::utils::{confirm_overwrite, default_output_path, parse_time};
use anyhow::Result;

pub fn execute(args: GifArgs) -> Result<()> {
    if !args.input.exists() {
        return Err(VxError::InputNotFound(args.input).into());
    }

    let output = args.output.unwrap_or_else(|| default_output_path(&args.input, None, "gif"));

    if !args.force && !confirm_overwrite(&output) {
        return Err(VxError::Cancelled.into());
    }

    let duration = get_video_duration(&args.input)?;
    let effective_duration = match (args.start.as_ref(), args.duration) {
        (Some(start), Some(dur)) => {
            let start_secs = parse_time(start)?;
            (duration - start_secs).min(dur)
        }
        (Some(start), None) => {
            let start_secs = parse_time(start)?;
            duration - start_secs
        }
        (None, Some(dur)) => dur.min(duration),
        (None, None) => duration,
    };

    let description = format!(
        "Converting {} → {}",
        args.input.file_name().unwrap_or_default().to_string_lossy(),
        output.file_name().unwrap_or_default().to_string_lossy()
    );

    let filter = format!(
        "fps={},scale={}:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse",
        args.fps, args.width
    );

    let mut runner = FfmpegRunner::new()?
        .with_progress(&description, Some(effective_duration))
        .overwrite();

    if let Some(ref start) = args.start {
        let start_secs = parse_time(start)?;
        runner = runner.args(["-ss", &start_secs.to_string()]);
    }

    runner = runner.input(&args.input);

    if let Some(dur) = args.duration {
        runner = runner.args(["-t", &dur.to_string()]);
    }

    runner = runner.args(["-vf", &filter]).output(&output);

    runner.run()?;

    println!("Created: {}", output.display());
    Ok(())
}
