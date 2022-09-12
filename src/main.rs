#![allow(unused)]

use std::env::args;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
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
    if !path.is_file() {
        let path_str = path.to_str().expect("error converting path to string");
        panic!("error: {path_str} is not a file");
    }
    let mut pe_file = File::open(&path)?;
    let pe_metadata = pe_file.metadata()?;
    if pe_metadata.len() < 60 {
        panic!("error: file too small");
    }

    let mut magic_mz = [0u8; 2];
    pe_file.read(&mut magic_mz)?;
    if magic_mz != [b'M', b'Z'] {
        panic!("Not PE file: first bytes must be 'MZ'");
    }
    pe_file.seek(SeekFrom::Start(0x3C))?;

    let mut magic_pe_offset_buff = [0u8; 4];
    pe_file.read(&mut magic_pe_offset_buff)?;
    let magic_pe_offset = u32::from_le_bytes(magic_pe_offset_buff) as u64;
    pe_file.seek(SeekFrom::Start(magic_pe_offset))?;
    let mut magic_pe = [0u8; 4];
    pe_file.read(&mut magic_pe)?;
    if magic_pe != [b'P', b'E', 0, 0] {
        panic!("Not a PE file: 'PE' bytes not found");
    }
    let mut machine_buf = [0u8; 2];
    pe_file.read_exact(&mut machine_buf)?;
    Ok(())
}
