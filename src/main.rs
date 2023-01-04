use std::env::args;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::process::exit;

struct FileHeader {
    raw: Box<[u8]>,
}

impl FileHeader {
    pub fn new(array: &[u8]) -> Self {
        Self {
            raw: Box::from(array),
        }
    }

    pub fn machine(&self) -> u16 {
        u16::from_le_bytes([self.raw[0], self.raw[1]])
    }

    pub fn number_of_sections(&self) -> u16 {
        u16::from_le_bytes([self.raw[2], self.raw[3]])
    }

    pub fn time_date_stamp(&self) -> u32 {
        u32::from_le_bytes([self.raw[4], self.raw[5], self.raw[6], self.raw[7]])
    }

    pub fn pointer_to_symbol_table(&self) -> u32 {
        u32::from_le_bytes([self.raw[8], self.raw[9], self.raw[10], self.raw[11]])
    }

    pub fn number_of_symbols(&self) -> u32 {
        u32::from_le_bytes([self.raw[12], self.raw[13], self.raw[14], self.raw[15]])
    }

    pub fn size_of_optional_header(&self) -> u16 {
        u16::from_le_bytes([self.raw[16], self.raw[17]])
    }

    pub fn characteristics(&self) -> u16 {
        u16::from_le_bytes([self.raw[18], self.raw[19]])
    }
}

impl Display for FileHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("File Header\n")?;
        f.write_fmt(format_args!("  Machine: {:#X}\n", self.machine()))?;
        f.write_fmt(format_args!(
            "  Number of sections: {}\n",
            self.number_of_sections()
        ))?;
        f.write_fmt(format_args!(
            "  Time date stamp: {}\n",
            self.time_date_stamp()
        ))?;
        f.write_fmt(format_args!(
            "  Pointer to symbol table: {}\n",
            self.pointer_to_symbol_table()
        ))?;
        f.write_fmt(format_args!(
            "  Number of symbols: {}\n",
            self.number_of_symbols()
        ))?;
        f.write_fmt(format_args!(
            "  Size of optional header: {}\n",
            self.size_of_optional_header()
        ))?;
        f.write_fmt(format_args!(
            "  Characteristics: {:#X}\n",
            self.characteristics()
        ))
    }
}

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
    let pe_metadata = pe_file.metadata()?;
    if pe_metadata.len() < 60 {
        println!("file too small for PE");
        exit(0);
    }

    let mut magic_mz = [0u8; 2];
    pe_file.read(&mut magic_mz)?;
    if magic_mz != [b'M', b'Z'] {
        println!("Not PE file: first bytes must be 'MZ'");
        exit(0);
    }
    pe_file.seek(SeekFrom::Start(0x3C))?;

    let mut magic_pe_offset_buff = [0u8; 4];
    pe_file.read(&mut magic_pe_offset_buff)?;
    let magic_pe_offset = u32::from_le_bytes(magic_pe_offset_buff) as u64;
    pe_file.seek(SeekFrom::Start(magic_pe_offset))?;
    let mut magic_pe = [0u8; 4];
    pe_file.read(&mut magic_pe)?;
    if magic_pe != [b'P', b'E', 0, 0] {
        println!("Not a PE file: 'PE' bytes not found");
        exit(0);
    }
    let mut coff_header_buf = [0u8; 20];
    pe_file.read(&mut coff_header_buf)?;
    let coff_file_header = FileHeader::new(&coff_header_buf);
    print!("{}", coff_file_header);
    Ok(())
}
