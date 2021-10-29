use std::fs::File;
use std::io::{BufRead, BufReader as IoBufReader, Result as IoResult};
use std::path::Path;
use std::rc::Rc;

/// Default buffer size of the reader.
const DEFAULT_BUFFER_SIZE: usize = 1024;

/// Buffered reader.
pub struct BufReader {
    /// Inner type.
    reader: IoBufReader<File>,
    /// Buffer.
    buffer: Rc<String>,
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
    fn new_buffer(buffer_size: Option<usize>) -> Rc<String> {
        Rc::new(String::with_capacity(
            buffer_size.unwrap_or(DEFAULT_BUFFER_SIZE),
        ))
    }
}

impl Iterator for BufReader {
    type Item = IoResult<Rc<String>>;
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
            .read_line(buffer)
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
        lines.push(line?.to_string());
    }
    Ok(lines.join(""))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_file_reader() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        println!("{:?}", path);
        assert!(read_to_string(path)
            .expect("cannot read Cargo.toml")
            .lines()
            .collect::<String>()
            .contains(&format!("name = \"{}\"", env!("CARGO_PKG_NAME"))));
    }
}