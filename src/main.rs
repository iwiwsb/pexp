use std::env::args;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::process::exit;

mod machine_types {
    const IMAGE_FILE_MACHINE_UNKNOWN: u16 = 0x0000; //The content of this field is assumed to be applicable to any machine type
    const IMAGE_FILE_MACHINE_AM33: u16 = 0x01D3; // Matsushita AM33
    const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664; // x64
    const IMAGE_FILE_MACHINE_ARM: u16 = 0x01C0; // ARM little endian
    const IMAGE_FILE_MACHINE_ARM64: u16 = 0xAA64; // ARM64 little endian
    const IMAGE_FILE_MACHINE_ARMNT: u16 = 0x01C4; //ARM Thumb-2 little endian
    const IMAGE_FILE_MACHINE_EBC: u16 = 0x0EBC; // EFI byte code
    const IMAGE_FILE_MACHINE_I386: u16 = 0x014C; // Intel 386 or later processors and compatible processors
    const IMAGE_FILE_MACHINE_IA64: u16 = 0x0200; // Intel Itanium processor family
    const IMAGE_FILE_MACHINE_LOONGARCH32: u16 = 0x6232; // LoongArch 32-bit processor family
    const IMAGE_FILE_MACHINE_LOONGARCH64: u16 = 0x6264; // LoongArch 64-bit processor family
    const IMAGE_FILE_MACHINE_M32R: u16 = 0x9041; // Mitsubishi M32R little endian
    const IMAGE_FILE_MACHINE_MIPS16: u16 = 0x266; // MIPS16
    const IMAGE_FILE_MACHINE_MIPSFPU: u16 = 0x0366; // MIPS with FPU
    const IMAGE_FILE_MACHINE_MIPSFPU16: u16 = 0x0466; // MIPS16 with FPU
    const IMAGE_FILE_MACHINE_POWERPC: u16 = 0x01F0; // Power PC little endian
    const IMAGE_FILE_MACHINE_POWERPCFP: u16 = 0x01F1; // Power PC with floating point support
    const IMAGE_FILE_MACHINE_R4000: u16 = 0x0166; // MIPS little endian
    const IMAGE_FILE_MACHINE_RISCV32: u16 = 0x5032; //RISC-V 32-bit address space
    const IMAGE_FILE_MACHINE_RISCV64: u16 = 0x5064; // RISC-V 64-bit address space
    const IMAGE_FILE_MACHINE_RISCV128: u16 = 0x5128; // RISC-V 128-bit address space
    const IMAGE_FILE_MACHINE_SH3: u16 = 0x01A2; // Hitachi SH3
    const IMAGE_FILE_MACHINE_SH3DSP: u16 = 0x01A3; // Hitachi SH3 DSP
    const IMAGE_FILE_MACHINE_SH4: u16 = 0x01A6; // Hitachi SH4
    const IMAGE_FILE_MACHINE_SH5: u16 = 0x01A8; // Hitachi SH5
    const IMAGE_FILE_MACHINE_THUMB: u16 = 0x01C2; // Thumb
    const IMAGE_FILE_MACHINE_WCEMIPSV2: u16 = 0x0169; // MIPS little-endian WCE v2
}

mod characteristics {
    const IMAGE_FILE_RELOCS_STRIPPED: u16 = 0x0001; // Image only, Windows CE, and Microsoft Windows NT and later. This indicates that the file does not contain base relocations and must therefore be loaded at its preferred base address. If the base address is not available, the loader reports an error. The default behavior of the linker is to strip base relocations from executable (EXE) files.
    const IMAGE_FILE_EXECUTABLE_IMAGE: u16 = 0x0002; // Image only. This indicates that the image file is valid and can be run. If this flag is not set, it indicates a linker error.
    const IMAGE_FILE_LINE_NUMS_STRIPPED: u16 = 0x0004; // COFF line numbers have been removed. This flag is deprecated and should be zero.
    const IMAGE_FILE_LOCAL_SYMS_STRIPPED: u16 = 0x0008; // COFF symbol table entries for local symbols have been removed. This flag is deprecated and should be zero.
    const IMAGE_FILE_AGGRESSIVE_WS_TRIM: u16 = 0x0010; // Obsolete. Aggressively trim working set. This flag is deprecated for Windows 2000 and later and must be zero.
    const IMAGE_FILE_LARGE_ADDRESS_AWARE: u16 = 0x0020; // Application can handle > 2-GB addresses.
    const IMAGE_FILE_RESERVED0: u16 = 0x0040; // This flag is reserved for future use.
    const IMAGE_FILE_BYTES_REVERSED_LO: u16 = 0x0080; // Little endian: the least significant bit (LSB) precedes the most significant bit (MSB) in memory. This flag is deprecated and should be zero.
    const IMAGE_FILE_32BIT_MACHINE: u16 = 0x0100; // Machine is based on a 32-bit-word architecture.
    const IMAGE_FILE_DEBUG_STRIPPED: u16 = 0x0200; // Debugging information is removed from the image file.
    const IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP: u16 = 0x0400; // If the image is on removable media, fully load it and copy it to the swap file.
    const IMAGE_FILE_NET_RUN_FROM_SWAP: u16 = 0x0800; // If the image is on network media, fully load it and copy it to the swap file.
    const IMAGE_FILE_SYSTEM: u16 = 0x1000; // The image file is a system file, not a user program.
    const IMAGE_FILE_DLL: u16 = 0x2000; // The image file is a dynamic-link library (DLL). Such files are considered executable files for almost all purposes, although they cannot be directly run.
    const IMAGE_FILE_UP_SYSTEM_ONLY: u16 = 0x4000; // The file should be run only on a uniprocessor machine.
    const IMAGE_FILE_BYTES_REVERSED_HI: u16 = 0x8000; // Big endian: the MSB precedes the LSB in memory. This flag is deprecated and should be zero.
}

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
