pub mod characteristics;
pub mod dll_characteristics;
pub mod header;
pub mod machine_types;
pub mod parser;
pub mod section_flags;
pub mod win_subsystem;

use parser::{ImageParser, ObjectParser, PortExeParse, PortExeType};
use std::env::args;
use std::fs::File;
use std::io::{self, Read, Seek};
use std::path::PathBuf;
use std::process::exit;

fn main() -> io::Result<()> {
    let mut cmdline_args = args();
    let path = match cmdline_args.nth(1) {
        Some(p) => PathBuf::from(p),
        None => {
            println!("Usage: pe_parser path");
            exit(0);
        }
    };
    let mut pe_file = File::open(&path)?;
    let pe_type = detect_pe_type(&mut pe_file).unwrap();
    let file_header = match pe_type {
        PortExeType::Image => ImageParser::new(&mut pe_file).file_header(),
        PortExeType::Object => ObjectParser::new(&mut pe_file).file_header(),
    };
    println!("{file_header}");
    Ok(())
}

fn detect_pe_type<R: Read + Seek>(reader: &mut R) -> io::Result<PortExeType> {
    let mut mz = [0u8; 2];
    reader.rewind()?;
    reader.read_exact(&mut mz)?;
    if mz == [b'M', b'Z'] {
        Ok(PortExeType::Image)
    } else {
        Ok(PortExeType::Object)
    }
}

trait BitFlags {}
