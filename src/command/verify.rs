use std::path::PathBuf;

use crate::{checksum_set::ChecksumSet, error::*, ui::UiHandler, util};

pub fn verify(
    path: PathBuf,
    root_path: Option<PathBuf>,
    ui: &mut dyn UiHandler,
) -> Result<(), CliError> {
    let root_path = root_path.unwrap_or_else(|| path.parent().unwrap().into());

    let checksums: ChecksumSet = {
        let file = util::open_file(&path).unwrap();
        serde_json::from_reader(file).unwrap()
    };

    let diff = checksums.verify(&root_path, ui);

    if diff.is_different() {
        diff.print();
    } else {
        println!("Verified OK.");
    }

    Ok(())
}
