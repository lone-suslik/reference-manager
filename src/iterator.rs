use crate::asset::ReferenceAsset;
use crate::hash::hash_big_file_async;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

pub struct FileHashIterator {
    dir: PathBuf,
    entries: fs::ReadDir,
}

impl Iterator for FileHashIterator {
    type Item = io::Result<ReferenceAsset>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(entry) = self.entries.next() {
            match entry {
                Err(e) => return Some(Err(e)),
                Ok(entry) => {
                    let path = entry.path();

                    if path.is_dir() {
                        let reference_info_path = path.join("reference-info.json");
                        if reference_info_path.is_file() {
                            let path_str = reference_info_path.to_string_lossy().into_owned();

                            match hash_big_file_async(&path_str) {
                                Err(e) => return Some(Err(e)),
                                Ok(hash) => {
                                    return Some(Ok(ReferenceAsset {
                                        origin: path,
                                        json_hash: hash,
                                    }))
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

pub fn collect_reference_objects(path: &Path) -> io::Result<FileHashIterator> {
    Ok(FileHashIterator {
        dir: path.to_path_buf(),
        entries: fs::read_dir(path)?,
    })
}

#[cfg(test)]
mod test_iterator {
    use super::*;

    #[test]
    fn test_collect_reference_objects() -> io::Result<()> {
        let path = Path::new("/Users/suslik/projects/rust/elf/test/fermen1/");
        let file_hash_iter = collect_reference_objects(&path)?;

        for a in file_hash_iter {
            let asset = a?;
            eprintln!("{:#?}", asset);
        }

        Ok(())
    }
}
