use super::UiHandler;

#[derive(Default)]
pub struct CliUiHandler;

impl UiHandler for CliUiHandler {
    fn begin_checksums(&mut self, _file_count: u32, _total_size: u64) {
        eprintln!("Calculating checksums...");
    }

    fn end_checksums(&mut self) {
        eprintln!("Calculating checksums done.");
    }

    fn begin_file(&mut self, path: &std::path::Path, _size: u64) {
        eprintln!(" {} ...", path.display());
    }

    fn end_file(&mut self) {}
}
