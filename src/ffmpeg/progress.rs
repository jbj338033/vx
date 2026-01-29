use indicatif::{ProgressBar, ProgressStyle};
use std::io::IsTerminal;
use std::time::Duration;

pub struct ProgressDisplay {
    bar: Option<ProgressBar>,
    total_duration: Option<f64>,
}

impl ProgressDisplay {
    pub fn new(description: &str, total_duration: Option<f64>) -> Self {
        if !std::io::stdout().is_terminal() {
            return Self { bar: None, total_duration };
        }

        let bar = match total_duration {
            Some(duration) => {
                let total = (duration * 1000.0) as u64;
                let pb = ProgressBar::new(total);
                pb.set_style(
                    ProgressStyle::default_bar()
                        .template("{msg}\n[{bar:40.cyan/blue}] {percent}% | {eta} remaining")
                        .expect("valid template")
                        .progress_chars("█▓░"),
                );
                pb.set_message(description.to_string());
                pb
            }
            None => {
                let pb = ProgressBar::new_spinner();
                pb.set_style(
                    ProgressStyle::default_spinner()
                        .template("{msg} {spinner}")
                        .expect("valid template"),
                );
                pb.set_message(description.to_string());
                pb.enable_steady_tick(Duration::from_millis(100));
                pb
            }
        };

        Self { bar: Some(bar), total_duration }
    }

    pub fn update(&self, current_time_ms: u64) {
        if let Some(ref bar) = self.bar {
            if self.total_duration.is_some() {
                bar.set_position(current_time_ms);
            }
        }
    }

    pub fn finish(&self) {
        if let Some(ref bar) = self.bar {
            bar.finish_and_clear();
        }
    }
}
