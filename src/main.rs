#![allow(dead_code)]

pub mod header;
pub mod struct_parse;

use std::{
    fs::OpenOptions,
    io::{self, ErrorKind, Read, Seek, SeekFrom},
};
use crate::header::file_header::FileHeaderReader;

use crate::header::machine_types::MACHINE_TYPES;

fn main() -> io::Result<()> {
    let mut pe_file = OpenOptions::new()
        .read(true)
        .write(false)
        .open(r"C:\Windows\System32\calc.exe")?;
    pe_file.seek(SeekFrom::Start(0x3C))?;
    let mut buf = [0u8; 4];
    pe_file.read_exact(&mut buf)?;
    let file_header_offset = u32::from_le_bytes(buf) as u64;
    let file_header = FileHeaderReader::new(file_header_offset, pe_file).read_file_header();
    println!("{}", file_header);
    Ok(())
}

pub enum PortExeType {
    Image,
    Object,
}

fn detect_pe_type<R: Read + Seek>(reader: &mut R) -> io::Result<PortExeType> {
    use PortExeType::*;

    const MZ_SIGNATURE: [u8; 2] = [b'M', b'Z'];
    let mut mz = [0u8; 2];
    reader.rewind()?;
    reader.read_exact(&mut mz)?;
    if mz == MZ_SIGNATURE {
        Ok(Image)
    } else if MACHINE_TYPES.contains(&u16::from_le_bytes(mz)) {
        Ok(Object)
    } else {
        Err(io::Error::from(ErrorKind::InvalidData))
    }
}
