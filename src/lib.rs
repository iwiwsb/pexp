mod file_header;
use std::io::Read;
use std::io::Seek;

use file_header::FileHeaderWrapper;

#[derive(Debug)]
struct StructField<T, const N: usize> {
    offset: u64,
    name: String,
    raw_bytes: [u8; N],
    value: T,
}

struct PEParser<R: Read + Seek> {
    inner: R,
}

impl<R: Read + Seek> PEParser<R> {
    pub fn read_file_header() -> FileHeaderWrapper {
        todo!()
    }
}
