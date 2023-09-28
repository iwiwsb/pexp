#![allow(unused)]

pub mod header;

use header::machine_types::Machine;
use std::io::{self, ErrorKind, Read, Seek};

fn main() -> io::Result<()> {
    Ok(())
}

enum PortExeType {
    Image,
    Object,
}

fn detect_pe_type<R: Read + Seek>(reader: &mut R) -> io::Result<PortExeType> {
    const MZ_SIGNATURE: [u8; 2] = [b'M', b'Z'];
    let mut mz = [0u8; 2];
    reader.rewind()?;
    reader.read_exact(&mut mz)?;
    if mz == MZ_SIGNATURE {
        Ok(PortExeType::Image)
    } else if Machine::try_from(mz).is_ok() {
        Ok(PortExeType::Object)
    } else {
        Err(io::Error::from(ErrorKind::InvalidData))
    }
}

trait BitFlags {}
