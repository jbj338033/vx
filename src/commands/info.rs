use crate::cli::InfoArgs;
use crate::error::VxError;
use crate::ffmpeg::get_video_info;
use crate::utils::{format_bitrate, format_duration, format_size};
use anyhow::Result;

pub fn execute(args: InfoArgs) -> Result<()> {
    if !args.input.exists() {
        return Err(VxError::InputNotFound(args.input).into());
    }

    let info = get_video_info(&args.input)?;
    let filename = args.input.file_name().unwrap_or_default().to_string_lossy();

    let codec_display = match &info.audio_codec {
        Some(audio) => format!("{} / {}", info.video_codec.to_uppercase(), audio.to_uppercase()),
        None => info.video_codec.to_uppercase(),
    };

    println!("{filename}");
    println!("─────────────────────────");
    println!("Duration    : {}", format_duration(info.duration));
    println!("Resolution  : {}x{}", info.width, info.height);
    println!("FPS         : {:.0}", info.fps);
    println!("Codec       : {codec_display}");
    println!("File size   : {}", format_size(info.file_size));
    println!("Bitrate     : {}", format_bitrate(info.bitrate));

    Ok(())
}
