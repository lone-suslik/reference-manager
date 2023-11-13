use std::io::BufReader;
use std::{hash::Hasher, io::Read};
use twox_hash::xxh3::Hash128;

pub fn hash_big_file_async(f: &str) -> Result<u64, std::io::Error> {
    let mut hash = Hash128::with_seed(0);
    let f: std::fs::File = std::fs::File::open(f)?;
    let mut f: BufReader<std::fs::File> = BufReader::new(f);
    let chunk_size = 4;

    loop {
        let mut chunk = Vec::with_capacity(chunk_size);
        let n = f.by_ref().take(chunk_size as u64).read_to_end(&mut chunk)?;

        if n == 0 {
            break; // this way the hash.finish() will just return 0
        }

        let a = chunk.as_slice();
        hash.write(a);
    }
    let res = hash.finish();
    Ok(res)
}
