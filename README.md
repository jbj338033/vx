# vx

Simple ffmpeg wrapper for humans.

```bash
# Before (ffmpeg)
ffmpeg -i video.mp4 -vf "fps=10,scale=480:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse" output.gif

# After (vx)
vx gif video.mp4
```

## Install

```bash
# Homebrew
brew install jbj338033/tap/vx

# Cargo
cargo install video-express
```

> Requires ffmpeg: `brew install ffmpeg`

## Usage

```bash
# Convert to GIF
vx gif video.mp4                     # → video.gif (480px, 10fps)
vx gif video.mp4 -w 720 -f 15        # Custom width and fps
vx gif video.mp4 -s 1:30 -d 5        # Start at 1:30, 5 seconds

# Compress video
vx compress video.mp4                # → video_compressed.mp4
vx compress video.mp4 -q low         # Smaller file, lower quality

# Convert format
vx to webm video.mp4                 # → video.webm
vx to mp4 video.mov                  # → video.mp4

# Show info
vx info video.mp4
```

## Commands

| Command | Description |
|---------|-------------|
| `vx gif <input>` | Convert to GIF with palette optimization |
| `vx compress <input>` | Compress video (H.264) |
| `vx to <format> <input>` | Convert format (mp4, webm, mov, avi, gif) |
| `vx info <input>` | Show video metadata |

## Options

```
vx gif
  -o, --output <file>    Output file
  -w, --width <px>       Width [default: 480]
  -f, --fps <n>          FPS [default: 10]
  -s, --start <time>     Start time (e.g., 1:30)
  -d, --duration <sec>   Duration
      --force            Overwrite without confirmation

vx compress
  -o, --output <file>    Output file
  -q, --quality <level>  low | medium | high [default: medium]
      --force            Overwrite without confirmation
```

## License

MIT
