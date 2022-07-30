pub mod cli;
pub mod fancy;

pub trait UiHandler {
    fn begin_checksums(&mut self, file_count: u32, total_size: u64);
    fn end_checksums(&mut self);

    fn begin_file(&mut self, filename: &str, size: u64);
    fn file_progress(&mut self, bytes: u64);
    fn end_file(&mut self);
}
