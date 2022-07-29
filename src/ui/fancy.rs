use indicatif::{ProgressBar, ProgressStyle};

use super::UiHandler;

pub struct FancyUiHandler {
    progress_chars: String,

    current_file_size: u64,
    overall_progress_bar: Option<ProgressBar>,
    file_progress_bar: Option<ProgressBar>,
}

impl FancyUiHandler {
    pub fn new() -> Self {
        Self {
            progress_chars: "●●·".to_owned(),

            current_file_size: 0,
            overall_progress_bar: None,
            file_progress_bar: None,
        }
    }
}

impl UiHandler for FancyUiHandler {
    fn begin_checksums(&mut self, _file_count: u32, total_size: u64) {
        let bar = ProgressBar::new(total_size)
            .with_style(
                ProgressStyle::default_bar()
                    .template("[{bar:40.cyan/blue}] {bytes}/{total_bytes} {wide_msg}")
                    .progress_chars(&self.progress_chars),
            )
            .with_message("Calculating checksums...");

        // Draw initial bar.
        bar.tick();

        self.overall_progress_bar = Some(bar);
    }

    fn end_checksums(&mut self) {
        if let Some(bar) = self.overall_progress_bar.take() {
            bar.println("Checksum calculation finished.");
            bar.finish_and_clear();
        }
    }

    fn begin_file(&mut self, path: &str, size: u64) {
        let bar = ProgressBar::new(0)
            .with_style(ProgressStyle::default_spinner().template("{spinner:.blue} {wide_msg}"))
            .with_message(path.to_owned());

        // Draw initial bar.
        bar.tick();

        // Make spinner spin
        bar.enable_steady_tick(250);

        self.current_file_size = size;
        self.file_progress_bar = Some(bar);
    }

    fn end_file(&mut self) {
        if let Some(bar) = self.file_progress_bar.take() {
            bar.finish_and_clear();
        }

        if let Some(bar) = self.overall_progress_bar.as_ref() {
            bar.inc(self.current_file_size);
        }
    }
}
