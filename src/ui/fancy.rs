use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use super::UiHandler;

const OVERALL_TEMPLATE: &str = " {prefix:>8} [{bar:40.cyan/blue}] {bytes}/{total_bytes} {wide_msg:.blue}";
const FILE_TEMPLATE: &str = " {prefix:>8} [{bar:40.cyan/blue}] {bytes}/{total_bytes} {wide_msg:.blue}";

pub struct FancyUiHandler {
    multi_progress: MultiProgress,
    progress_chars: String,

    overall_pb: Option<ProgressBar>,
    file_pb: Option<ProgressBar>,
}

impl FancyUiHandler {
    pub fn new() -> Self {
        Self {
            multi_progress: MultiProgress::new(),
            progress_chars: "●●·".to_owned(),

            overall_pb: None,
            file_pb: None,
        }
    }

    pub fn clear(self) -> Result<(), anyhow::Error> {
        self.multi_progress.clear()?;

        Ok(())
    }
}

impl UiHandler for FancyUiHandler {
    fn begin_generate(&mut self, _file_count: u32, total_size: u64) {
        let pb = ProgressBar::new(total_size)
            .with_style(
                ProgressStyle::default_bar()
                    .template(OVERALL_TEMPLATE)
                    .unwrap()
                    .progress_chars(&self.progress_chars),
            )
            .with_prefix("Overall")
            .with_message("Generating checksum set...");

        let pb = self.multi_progress.add(pb);

        self.overall_pb = Some(pb);
    }

    fn end_generate(&mut self) {
        if let Some(pb) = self.overall_pb.take() {
            pb.println("Generating checksum set finished.");
            pb.finish_and_clear();
        }
    }

    fn begin_verify(&mut self, _file_count: u32, total_size: u64) {
        let pb = ProgressBar::new(total_size)
            .with_style(
                ProgressStyle::default_bar()
                    .template(OVERALL_TEMPLATE)
                    .unwrap()
                    .progress_chars(&self.progress_chars),
            )
            .with_prefix("Overall")
            .with_message("Verifying...");

        let pb = self.multi_progress.add(pb);

        self.overall_pb = Some(pb);
    }

    fn end_verify(&mut self) {
        if let Some(pb) = self.overall_pb.take() {
            pb.println("Verification finished.");
            pb.finish_and_clear();
        }
    }

    fn begin_file(&mut self, filename: &str, size: u64) {
        let pb = ProgressBar::new(size)
            .with_style(
                ProgressStyle::default_bar()
                    .template(FILE_TEMPLATE)
                    .unwrap()
                    .progress_chars(&self.progress_chars),
            )
            .with_prefix("File")
            .with_message(filename.to_owned());

        let pb = self.multi_progress.add(pb);

        self.file_pb = Some(pb);
    }

    fn file_progress(&mut self, bytes: u64) {
        if let Some(pb) = self.file_pb.as_ref() {
            pb.inc(bytes);
        }

        if let Some(pb) = self.overall_pb.as_ref() {
            pb.inc(bytes);
        }
    }

    fn end_file(&mut self) {
        if let Some(pb) = self.file_pb.take() {
            pb.finish_and_clear();
        }
    }
}
