mod coff_header;
mod optional_headers;
mod section_table;

use coff_header::COFFFileHeader;
use optional_headers::OptionalHeader;
use section_table::SectionTable;

use std::{
    fs::{File, OpenOptions},
    io,
    path::Path,
};

pub struct PEHeaders {
    ms_dos_stub: Vec<u8>,
    signature: [u8; 4],
    coff_file_header: COFFFileHeader,
    optional_header: Option<OptionalHeader>,
    section_table: SectionTable,
}

struct PEParser {
    pe_file: File,
}

impl PEParser {
    fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let pe_file = OpenOptions::new().read(true).open(path)?;
        Ok(Self { pe_file })
    }

    fn parse(&self) -> io::Result<PEHeaders> {
        todo!()
    }
}
