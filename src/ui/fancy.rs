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
                    .template(
                        " {prefix:>8} [{bar:40.cyan/blue}] {bytes}/{total_bytes} {wide_msg:.blue}",
                    )
                    .progress_chars(&self.progress_chars),
            )
            .with_prefix("Overall")
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

    fn begin_file(&mut self, filename: &str, size: u64) {
        let bar = ProgressBar::new(size)
            .with_style(
                ProgressStyle::default_bar()
                    .template(
                        " {prefix:>8} [{bar:40.cyan/blue}] {bytes}/{total_bytes} {wide_msg:.blue}",
                    )
                    .progress_chars(&self.progress_chars),
            )
            .with_prefix("File")
            .with_message(filename.to_owned());

        // Draw initial bar.
        bar.tick();

        self.current_file_size = size;
        self.file_progress_bar = Some(bar);
    }

    fn file_progress(&mut self, bytes: u64) {
        if let Some(bar) = self.file_progress_bar.as_ref() {
            bar.inc(bytes);
        }
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
