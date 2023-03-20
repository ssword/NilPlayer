use crate::file;
use crate::{errors::WaveError, pattern};
use byteorder::ByteOrder;
use byteorder::LittleEndian;
use std::path::Path;
use std::str;

pub struct Reader {
    pub data: Vec<u8>,
    cursor: usize,
}

impl Reader {
    pub fn new(path: &Path) -> Self {
        Reader {
            data: file::read_file(path).unwrap(),
            cursor: 0,
        }
    }

    pub fn seek(&mut self, index: uszie) {
        self.cursor = index;
    }
}
