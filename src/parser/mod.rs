use crate::header::{
    read_file_header, read_optional_header, FileHeader, OptionalHeader, FILE_HEADER_SIZE,
};
use std::io::{self, Read, Seek, SeekFrom};

#[derive(Debug, PartialEq)]
pub enum PortExeType {
    Object,
    Image,
}

pub struct ImageParser<R> {
    reader: R,
    file_header_offset: u64,
}

impl<R: Read + Seek> ImageParser<R> {
    pub fn new(mut reader: R) -> Self {
        let file_header_offset = get_file_header_offset(&mut reader, &PortExeType::Image).unwrap();
        Self {
            reader,
            file_header_offset,
        }
    }
}

impl<R: Read + Seek> PortExeParse for ImageParser<R> {
    fn file_header(&mut self) -> io::Result<FileHeader> {
        read_file_header(&mut self.reader, self.file_header_offset)
    }
}

impl<R: Read + Seek> PortExeImageParse for ImageParser<R> {
    fn optional_header(&mut self) -> OptionalHeader {
        let opt_header_offset = self.file_header_offset + FILE_HEADER_SIZE;
        read_optional_header(&mut self.reader, opt_header_offset).unwrap()
    }
}

pub struct ObjectParser<R> {
    reader: R,
}

impl<R: Read + Seek> ObjectParser<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R: Read + Seek> PortExeParse for ObjectParser<R> {
    fn file_header(&mut self) -> io::Result<FileHeader> {
        read_file_header(&mut self.reader, 0)
    }
}

pub trait PortExeParse {
    /// Returns file header
    fn file_header(&mut self) -> io::Result<FileHeader>;
}

trait PortExeImageParse: PortExeParse {
    /// Returns optional header
    fn optional_header(&mut self) -> OptionalHeader;
}

trait PortExeObjectParse: PortExeParse {}

pub fn get_file_header_offset<R: Read + Seek>(
    reader: &mut R,
    pe_type: &PortExeType,
) -> io::Result<u64> {
    match pe_type {
        PortExeType::Image => {
            let mut bytes = [0u8; 4];
            reader.seek(SeekFrom::Start(0x3C))?;
            reader.read_exact(&mut bytes)?;
            Ok((u32::from_le_bytes(bytes) as u64) + 4)
        }
        PortExeType::Object => Ok(0),
    }
}
