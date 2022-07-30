use super::UiHandler;

#[derive(Default)]
pub struct CliUiHandler;

impl UiHandler for CliUiHandler {
    fn begin_generate(&mut self, _file_count: u32, _total_size: u64) {
        eprintln!("Generating checksum set...");
    }

    fn end_generate(&mut self) {
        eprintln!("Generating checksum set finished.");
    }

    fn begin_verify(&mut self, _file_count: u32, _total_size: u64) {
        eprintln!("Verifying...");
    }

    fn end_verify(&mut self) {
        eprintln!("Verification finished.");
    }

    fn begin_file(&mut self, filename: &str, _size: u64) {
        eprintln!(" {} ...", filename);
    }

    fn file_progress(&mut self, _bytes: u64) {}

    fn end_file(&mut self) {}
}
