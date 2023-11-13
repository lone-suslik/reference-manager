use crate::hash::hash_big_file_async;
use std::fs;
use std::io;
use std::path::Path;

pub fn collect_reference_objects(path: &Path) -> io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let reference_info_path = path.join("reference-info.json");

                if reference_info_path.is_file() {
                    let path_str = reference_info_path.to_string_lossy().into_owned();
                    let file_hash = hash_big_file_async(&path_str)?;
                }
            }
        }
    }
    Ok(())
}
