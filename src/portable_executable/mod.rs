mod coff_header;
mod optional_headers;
mod section_table;

use coff_header::COFFFileHeader;
use optional_headers::OptionalHeader;
use section_table::SectionTable;
use std::io::{self, ErrorKind, Read, Seek, SeekFrom};

use coff_header::MachineType;

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
        Self {
            inner,
            pe_type: None,
        }
    }

    fn with_type(inner: R, pe_type: PEType) -> Self {
        Self {
            inner,
            pe_type: Some(pe_type),
        }
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
        let mut first_2_bytes = [0u8; 2];
        self.inner.read(&mut first_2_bytes)?;
        if first_2_bytes == [b'M', b'Z'] {
            self.inner.seek(SeekFrom::Start(0x3C))?;
            let mut pe_signature_offset = [0u8; 4];
            self.inner.read(&mut pe_signature_offset)?;
            self.inner.seek(SeekFrom::Start(
                u32::from_le_bytes(pe_signature_offset) as u64
            ))?;
            let mut pe_signature = [0u8; 4];
            self.inner.read(&mut pe_signature)?;
            if pe_signature == [b'P', b'E', 0, 0] {
                self.inner.seek(SeekFrom::Current(20))?;
                let mut pe_magic = [0u8; 2];
                self.inner.read(&mut pe_magic)?;
                match pe_magic {
                    [0x0B, 0x01] => self.pe_type = Some(PEType::Image32).or(self.pe_type),
                    [0x0B, 0x02] => self.pe_type = Some(PEType::Image64).or(self.pe_type),
                    [0x07, 0x01] => self.pe_type = Some(PEType::ImageRom).or(self.pe_type),
                    _ => return Err(io::Error::from(ErrorKind::InvalidData)),
                }
                return Ok(self);
            } else {
                return Err(io::Error::from(ErrorKind::InvalidData));
            }
        } else if MachineType::try_from(first_2_bytes).is_ok() {
            self.pe_type = Some(PEType::Object).or(self.pe_type);
            return Ok(self);
        } else {
            return Err(io::Error::from(ErrorKind::InvalidData));
        }
    }
}
