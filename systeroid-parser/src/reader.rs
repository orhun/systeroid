use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{
    BufRead, BufReader as IoBufReader, Error as IoError, ErrorKind as IoErrorKind, Read,
    Result as IoResult,
};
use std::path::Path;
use std::rc::Rc;
use std::str;

/// Default buffer size of the reader.
const DEFAULT_BUFFER_SIZE: usize = 1024;

/// Buffered reader.
pub struct BufReader {
    /// Inner type.
    reader: IoBufReader<File>,
    /// Buffer.
    buffer: Rc<Vec<u8>>,
}

impl BufReader {
    /// Opens the given file and initializes the buffered reader with given buffer size.
    pub fn open<P: AsRef<Path>>(path: P, buffer_size: Option<usize>) -> IoResult<Self> {
        let file = File::open(path)?;
        let reader = IoBufReader::new(file);
        let buffer = Self::new_buffer(buffer_size);
        Ok(Self { reader, buffer })
    }

    /// Creates a new buffer with the given size.
    fn new_buffer(buffer_size: Option<usize>) -> Rc<Vec<u8>> {
        Rc::new(Vec::with_capacity(
            buffer_size.unwrap_or(DEFAULT_BUFFER_SIZE),
        ))
    }
}

impl Iterator for BufReader {
    type Item = IoResult<Rc<Vec<u8>>>;
    fn next(&mut self) -> Option<Self::Item> {
        let buffer = match Rc::get_mut(&mut self.buffer) {
            Some(rc_buffer) => {
                rc_buffer.clear();
                rc_buffer
            }
            None => {
                self.buffer = Self::new_buffer(None);
                Rc::make_mut(&mut self.buffer)
            }
        };
        self.reader
            .read_until(b'\n', buffer)
            .map(|u| {
                if u == 0 {
                    None
                } else {
                    Some(Rc::clone(&self.buffer))
                }
            })
            .transpose()
    }
}

/// Reads the contents of the file into a string.
///
/// Uses [`BufReader`] under the hood.
pub fn read_to_string<P: AsRef<Path>>(path: P) -> IoResult<String> {
    let mut lines = Vec::<String>::new();
    for line in BufReader::open(path, None)? {
        lines.push(
            str::from_utf8(&line?)
                .map_err(|e| IoError::new(IoErrorKind::Other, e))?
                .to_string(),
        );
    }
    Ok(lines.join(""))
}

/// Reads (decodes) the given gzip file into a string.
///
/// Uses [`BufReader`] under the hood.
pub fn read_gzip<P: AsRef<Path>>(path: P) -> IoResult<String> {
    let mut bytes = Vec::<u8>::new();
    for read_bytes in BufReader::open(path, None)? {
        bytes.extend(read_bytes?.to_vec());
    }
    let mut gz = GzDecoder::new(&bytes[..]);
    let mut data = String::new();
    gz.read_to_string(&mut data)?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use std::path::PathBuf;

    #[test]
    fn test_file_reader() -> Result<(), Error> {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        assert!(read_to_string(path)?.contains(&format!("name = \"{}\"", env!("CARGO_PKG_NAME"))));
        Ok(())
    }
}
