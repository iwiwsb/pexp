use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
};

fn main() {
    let mut pe_reader =
        File::open(".\\target\\debug\\pexp.exe").expect("The file must exists and could be opened");
    let mut first_two_bytes = [0u8; 2];
    let _ = pe_reader.read_exact(&mut first_two_bytes);
    println!(
        "First two bytes: {:X} {:X}\n",
        first_two_bytes[0], first_two_bytes[1]
    );
    if first_two_bytes == [b'M', b'Z'] {
        let _ = pe_reader.seek(SeekFrom::Start(0x3C));
        let mut pe_header_addr = [0u8; 4];
        let _ = pe_reader.read_exact(&mut pe_header_addr);
        let _ = pe_reader.seek(SeekFrom::Start(u32::from_le_bytes(pe_header_addr) as u64));
        let mut image_signature = [0u8; 4];
        let _ = pe_reader.read_exact(&mut image_signature);
        println!(
            "Image signature: {:X} {:X} {:X} {:X}\n",
            image_signature[0], image_signature[1], image_signature[2], image_signature[3]
        );
        let _offset = pe_reader.stream_position().unwrap();
    }
}
