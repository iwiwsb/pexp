pub mod header;
pub mod section;

use std::{
    fs::File,
    io::{Read, Seek},
};

use header::FileHeader;

fn main() {
    let mut pe_file = File::open("target\\release\\pexp.exe").expect("File should be openable");
    let mut buf = [0u8; 4];
    pe_file.seek(std::io::SeekFrom::Start(0x3C));
    pe_file.read_exact(&mut buf);
    pe_file.seek(std::io::SeekFrom::Start(u32::from_le_bytes(buf) as u64 + 4));
    let file_header = FileHeader::read_from(&mut pe_file);
    print!("{:?}", file_header);
}
