mod coff_header;
mod optional_headers;
mod section_table;

use coff_header::COFFFileHeader;
use optional_headers::OptionalHeader;
use section_table::SectionTable;
use std::io::{self, Read, Seek, SeekFrom};

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
        let mut first_2_bytes = [0u8; 2];
        let mut reader = self.inner;
        reader.read(&mut first_2_bytes)?;
        if first_2_bytes == [b'M', b'Z'] {
            reader.seek(SeekFrom::Start(0x3C))?;
            let mut pe_signature_offset = [0u8; 4];
            reader.read(&mut pe_signature_offset)?;
            reader.seek(SeekFrom::Start(u32::from_le_bytes(pe_signature_offset) as u64))?;
            let mut pe_signature = [0u8; 4];
            reader.read(&mut pe_signature)?;
            if pe_signature == [b'P', b'E', 0, 0] {
                reader.seek(SeekFrom::Current(20))?;
                let mut pe_magic = [0u8; 2];
                reader.read(&mut pe_magic)?;
                match pe_magic {
                    [0x0B, 0x01] => Ok(Self { inner: reader, pe_type: Some(PEType::Image32)}),
                    [0x0B, 0x02] => Ok(Self { inner: reader, pe_type: Some(PEType::Image64)}),
                    [0x07, 0x01] => Ok(Self { inner: reader, pe_type: Some(PEType::ImageRom)}),
                    _ => todo!()
                }
            } else {
                todo!()
            }
        } else if MachineType::try_from(first_2_bytes).is_ok() {
             todo!()
        } else {
            todo!()
        }
    }
}