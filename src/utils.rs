use std::path::Path;
use std::fs::File;
use std::io::{Read, Result as IoResult};

pub fn read_file<T: AsRef<Path>>(path: T) -> IoResult<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut vec = Vec::with_capacity(2048);
    file.read_to_end(&mut vec)?;
    Ok(vec)
}
