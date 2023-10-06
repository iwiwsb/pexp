#![allow(dead_code)]

pub mod header;
pub mod struct_parse;

use header::machine_types::Machine;
use std::{
    fs::OpenOptions,
    io::{self, ErrorKind, Read, Seek, SeekFrom},
};

fn main() -> io::Result<()> {
    let mut pe_file = OpenOptions::new()
        .read(true)
        .write(false)
        .open(r#"C:\Windows\System32\calc.exe"#)?;
    pe_file.seek(SeekFrom::Start(0x3C))?;
    let mut buf = [0u8; 4];
    pe_file.read(&mut buf)?;
    let file_header_offset = u64::from_le_bytes([buf[0], buf[1], buf[2], buf[3], 0, 0, 0, 0]);
    pe_file.seek(SeekFrom::Start(file_header_offset))?;
    let mut file_header_buffer = [0u8; 24];
    pe_file.read(&mut file_header_buffer)?;

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
    } else if Machine::try_from(mz).is_ok() {
        Ok(Object)
    } else {
        Err(io::Error::from(ErrorKind::InvalidData))
    }
}
