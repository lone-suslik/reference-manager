use std::path::PathBuf;

#[derive(Debug)]
pub struct ReferenceAsset {
    /// This directory must contain file reference-info.json
    pub origin: PathBuf,
    /// Full path to the original directory of the reference.
    pub json_hash: u64,
}
