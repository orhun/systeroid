use std::fs::File;
use std::io::{BufRead, BufReader as IoBufReader, Result as IoResult};
use std::path::Path;
use std::rc::Rc;

const DEFAULT_BUFFER_SIZE: usize = 1024;

pub struct BufReader {
    reader: IoBufReader<File>,
    buffer: Rc<String>,
}

impl BufReader {
    pub fn open(path: impl AsRef<Path>, buffer_size: Option<usize>) -> IoResult<Self> {
        let file = File::open(path)?;
        let reader = IoBufReader::new(file);
        let buffer = Self::new_buffer(buffer_size);

        Ok(Self { reader, buffer })
    }

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

pub fn read_to_string(path: &Path) -> IoResult<String> {
    let mut lines = Vec::<String>::new();
    for line in BufReader::open(path, None)? {
        lines.push(line?.to_string());
    }
    Ok(lines.join(""))
}
