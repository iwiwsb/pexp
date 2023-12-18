use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
};

fn main() {
    let mut pe_reader =
        File::open(".\\target\\debug\\pexp.exe").expect("The file must exists and could be opened");
    let mut magic_num = [0u8; 2];
    let _ = pe_reader.read_exact(&mut magic_num);
    println!(
        "Signature: {}{}\n",
        char::from(magic_num[0]),
        char::from(magic_num[1])
    );
    let _ = pe_reader.seek(SeekFrom::Start(0x3C));
    let mut pe_header_addr = [0u8; 4];
    let _ = pe_reader.read_exact(&mut pe_header_addr);
    let _ = pe_reader.seek(SeekFrom::Start(u32::from_le_bytes(pe_header_addr) as u64));
    
}
