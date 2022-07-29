pub mod cli;

use std::path::Path;

pub trait UiHandler {
    fn begin_checksums(&mut self, file_count: u32, total_size: u64);
    fn end_checksums(&mut self);

    fn begin_file(&mut self, path: &Path, size: u64);
    fn end_file(&mut self);
}
