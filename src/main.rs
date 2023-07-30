pub mod header;
pub mod parser;

use header::machine_types::Machine;
use parser::PortExeType;
use std::env::args;
use std::fs::File;
use std::io::{self, ErrorKind, Read, Seek, SeekFrom};
use std::path::PathBuf;

use crate::header::read_file_header;

fn main() -> io::Result<()> {
    let mut cmdline_args = args();
    let path = match cmdline_args.nth(1) {
        Some(p) => PathBuf::from(p),
        None => {
            println!("Usage: pe_parser path");
            return Ok(());
        }
    };
    let mut pe_file = File::open(&path)?;
    let pe_type = detect_pe_type(&mut pe_file);
    match pe_type {
        Ok(PortExeType::Image) => {
            println!("File is an image");
            let file_header_offset =
                parser::get_file_header_offset(&mut pe_file, &PortExeType::Image)?;
            pe_file.seek(SeekFrom::Start(file_header_offset))?;
            let file_header = read_file_header(&mut pe_file, file_header_offset)?;

            println!("{}", file_header)
        }
        Ok(PortExeType::Object) => {
            println!("File is an object")
        }
        Err(e) => return Err(e),
    }
    Ok(())
}

fn detect_pe_type<R: Read + Seek>(reader: &mut R) -> io::Result<PortExeType> {
    let mut mz = [0u8; 2];
    reader.rewind()?;
    reader.read_exact(&mut mz)?;
    if mz == [b'M', b'Z'] {
        Ok(PortExeType::Image)
    } else if Machine::try_from(mz).is_ok() {
        Ok(PortExeType::Object)
    } else {
        Err(io::Error::from(ErrorKind::InvalidData))
    }
}

trait BitFlags {}
