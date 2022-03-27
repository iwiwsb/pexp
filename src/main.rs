#![allow(dead_code)]

use std::env::args;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::process::exit;

mod pe;

fn main() {
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
        println!("error: {path_str} is not a file");
        exit(0);
    }
    let mut pe_file = File::open(&path).expect("error opening file");
    let pe_metadata = pe_file.metadata().expect("error reading metadata");
    if pe_metadata.len() < 60 {
        println!("error: file too small");
        exit(0)
    }

    let mut magic_mz = [0u8; 2];
    pe_file.read(&mut magic_mz).expect("error reading file");
    if magic_mz != [b'M', b'Z'] {
        println!("Not PE file: first bytes must be 'MZ'");
        exit(0);
    }
    pe_file
        .seek(SeekFrom::Start(0x3C))
        .expect("error reading file at 0x3C offset");

    let mut magic_pe_offset_buff = [0u8; 4];
    pe_file
        .read(&mut magic_pe_offset_buff)
        .expect("error reading pe bytes offset");
    let magic_pe_offset = u32::from_le_bytes(magic_pe_offset_buff) as u64;
    pe_file
        .seek(SeekFrom::Start(magic_pe_offset))
        .expect("error seeking pe bytes");
    let mut magic_pe = [0u8; 4];
    pe_file
        .read(&mut magic_pe)
        .expect(format!("error reading bytes at {magic_pe_offset:?} offset").as_str());
    if magic_pe != [b'P', b'E', 0, 0] {
        println!("Not a PE file: 'PE' bytes not found");
    }
    let mut machine_buf = [0u8; 2];
    pe_file
        .read_exact(&mut machine_buf)
        .expect("error reading COFF File Header");
}
