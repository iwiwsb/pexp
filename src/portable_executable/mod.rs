mod coff_header;
mod optional_headers;
mod section_table;

use coff_header::COFFFileHeader;
use optional_headers::OptionalHeader;
use section_table::SectionTable;
use std::io::{self, Read, Seek};

pub struct PEHeaders {
    ms_dos_stub: Vec<u8>,
    signature: [u8; 4],
    coff_file_header: COFFFileHeader,
    optional_header: Option<OptionalHeader>,
    section_table: SectionTable,
}

#[derive(Clone, Copy, Debug)]
enum PEType {
    Object,
    Image32,
    Image64,
    ImageRom,
}

struct Reader<R: Read> {
    inner: R,
    pe_type: Option<PEType>,
}

impl<R: Read> Reader<R> {
    fn new(inner: R) -> Self {
        Self { inner, pe_type: None }
    }

    fn with_type(inner: R, pe_type: PEType) -> Self {
        Self { inner, pe_type: Some(pe_type) }
    }

    fn pe_type(&self) -> Option<PEType> {
        self.pe_type
    }

    fn set_type(&mut self, pe_type: PEType) {
        self.pe_type = Some(pe_type);
    }
}

impl<R: Read + Seek> Reader<R> {
    fn with_guessed_type(mut self) -> io::Result<Self> {
        todo!()
    }
}
