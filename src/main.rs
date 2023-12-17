pub mod file_header;
pub mod optional_header;
pub mod section;

use std::{
    fs::File,
    io::{Read, Seek},
};

fn main() {
    let mut pe_file = File::open("target\\release\\pexp.exe").expect("File should be openable");
    let mut buf = [0u8; 4];
    let _ = pe_file.seek(std::io::SeekFrom::Start(0x3C));
    let _ = pe_file.read_exact(&mut buf);
    let _ = pe_file.seek(std::io::SeekFrom::Start(u32::from_le_bytes(buf) as u64 + 4));
}
