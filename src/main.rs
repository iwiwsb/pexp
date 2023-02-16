use std::env::args;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::process::exit;

pub mod machine_types {
    pub const IMAGE_FILE_MACHINE_UNKNOWN: [u8; 2] = [0x00, 0x00]; //The content of this field is assumed to be applicable to any machine type
    pub const IMAGE_FILE_MACHINE_AM33: [u8; 2] = [0xD3, 0x01]; // Matsushita AM33
    pub const IMAGE_FILE_MACHINE_AMD64: [u8; 2] = [0x64, 0x86]; // x64
    pub const IMAGE_FILE_MACHINE_ARM: [u8; 2] = [0xC0, 0x01]; // ARM little endian
    pub const IMAGE_FILE_MACHINE_ARM64: [u8; 2] = [0x64, 0xAA]; // ARM64 little endian
    pub const IMAGE_FILE_MACHINE_ARMNT: [u8; 2] = [0xC4, 0x01]; //ARM Thumb-2 little endian
    pub const IMAGE_FILE_MACHINE_EBC: [u8; 2] = [0xBC, 0x0E]; // EFI byte code
    pub const IMAGE_FILE_MACHINE_I386: [u8; 2] = [0x4C, 0x01]; // Intel 386 or later processors and compatible processors
    pub const IMAGE_FILE_MACHINE_IA64: [u8; 2] = [0x00, 0x02]; // Intel Itanium processor family
    pub const IMAGE_FILE_MACHINE_LOONGARCH32: [u8; 2] = [0x32, 0x62]; // LoongArch 32-bit processor family
    pub const IMAGE_FILE_MACHINE_LOONGARCH64: [u8; 2] = [0x64, 0x62]; // LoongArch 64-bit processor family
    pub const IMAGE_FILE_MACHINE_M32R: [u8; 2] = [0x41, 0x90]; // Mitsubishi M32R little endian
    pub const IMAGE_FILE_MACHINE_MIPS16: [u8; 2] = [0x66, 0x02]; // MIPS16
    pub const IMAGE_FILE_MACHINE_MIPSFPU: [u8; 2] = [0x66, 0x03]; // MIPS with FPU
    pub const IMAGE_FILE_MACHINE_MIPSFPU16: [u8; 2] = [0x66, 0x04]; // MIPS16 with FPU
    pub const IMAGE_FILE_MACHINE_POWERPC: [u8; 2] = [0xF0, 0x01]; // Power PC little endian
    pub const IMAGE_FILE_MACHINE_POWERPCFP: [u8; 2] = [0xF1, 0x01]; // Power PC with floating point support
    pub const IMAGE_FILE_MACHINE_R4000: [u8; 2] = [0x66, 0x01]; // MIPS little endian
    pub const IMAGE_FILE_MACHINE_RISCV32: [u8; 2] = [0x32, 0x50]; //RISC-V 32-bit address space
    pub const IMAGE_FILE_MACHINE_RISCV64: [u8; 2] = [0x64, 0x50]; // RISC-V 64-bit address space
    pub const IMAGE_FILE_MACHINE_RISCV128: [u8; 2] = [0x28, 0x51]; // RISC-V 128-bit address space
    pub const IMAGE_FILE_MACHINE_SH3: [u8; 2] = [0xA2, 0x01]; // Hitachi SH3
    pub const IMAGE_FILE_MACHINE_SH3DSP: [u8; 2] = [0xA3, 0x01]; // Hitachi SH3 DSP
    pub const IMAGE_FILE_MACHINE_SH4: [u8; 2] = [0xA6, 0x01]; // Hitachi SH4
    pub const IMAGE_FILE_MACHINE_SH5: [u8; 2] = [0xA8, 0x01]; // Hitachi SH5
    pub const IMAGE_FILE_MACHINE_THUMB: [u8; 2] = [0xC2, 0x01]; // Thumb
    pub const IMAGE_FILE_MACHINE_WCEMIPSV2: [u8; 2] = [0x69, 0x01]; // MIPS little-endian WCE v2
}

pub mod characteristics {
    pub const IMAGE_FILE_RELOCS_STRIPPED: u16 = 0x0001; // Image only, Windows CE, and Microsoft Windows NT and later. This indicates that the file does not contain base relocations and must therefore be loaded at its preferred base address. If the base address is not available, the loader reports an error. The default behavior of the linker is to strip base relocations from executable (EXE) files.
    pub const IMAGE_FILE_EXECUTABLE_IMAGE: u16 = 0x0002; // Image only. This indicates that the image file is valid and can be run. If this flag is not set, it indicates a linker error.
    pub const IMAGE_FILE_LINE_NUMS_STRIPPED: u16 = 0x0004; // COFF line numbers have been removed. This flag is deprecated and should be zero.
    pub const IMAGE_FILE_LOCAL_SYMS_STRIPPED: u16 = 0x0008; // COFF symbol table entries for local symbols have been removed. This flag is deprecated and should be zero.
    pub const IMAGE_FILE_AGGRESSIVE_WS_TRIM: u16 = 0x0010; // Obsolete. Aggressively trim working set. This flag is deprecated for Windows 2000 and later and must be zero.
    pub const IMAGE_FILE_LARGE_ADDRESS_AWARE: u16 = 0x0020; // Application can handle > 2-GB addresses.
    pub const IMAGE_FILE_RESERVED0: u16 = 0x0040; // This flag is reserved for future use.
    pub const IMAGE_FILE_BYTES_REVERSED_LO: u16 = 0x0080; // Little endian: the least significant bit (LSB) precedes the most significant bit (MSB) in memory. This flag is deprecated and should be zero.
    pub const IMAGE_FILE_32BIT_MACHINE: u16 = 0x0100; // Machine is based on a 32-bit-word architecture.
    pub const IMAGE_FILE_DEBUG_STRIPPED: u16 = 0x0200; // Debugging information is removed from the image file.
    pub const IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP: u16 = 0x0400; // If the image is on removable media, fully load it and copy it to the swap file.
    pub const IMAGE_FILE_NET_RUN_FROM_SWAP: u16 = 0x0800; // If the image is on network media, fully load it and copy it to the swap file.
    pub const IMAGE_FILE_SYSTEM: u16 = 0x1000; // The image file is a system file, not a user program.
    pub const IMAGE_FILE_DLL: u16 = 0x2000; // The image file is a dynamic-link library (DLL). Such files are considered executable files for almost all purposes, although they cannot be directly run.
    pub const IMAGE_FILE_UP_SYSTEM_ONLY: u16 = 0x4000; // The file should be run only on a uniprocessor machine.
    pub const IMAGE_FILE_BYTES_REVERSED_HI: u16 = 0x8000; // Big endian: the MSB precedes the LSB in memory. This flag is deprecated and should be zero.
}

pub const IMAGE_NT_OPTIONAL_HDR32_MAGIC: u16 = 0x010B; // The file is an executable image of 32-bit application
pub const IMAGE_NT_OPTIONAL_HDR64_MAGIC: u16 = 0x020B; // The file is an executable image of 64-bit application
pub const IMAGE_ROM_OPTIONAL_HDR_MAGIC: u16 = 0x0107; // The file is a ROM image.

fn main() -> io::Result<()> {
    let mut cmdline_args = args();
    let path = match cmdline_args.nth(1) {
        Some(p) => PathBuf::from(p),
        None => {
            println!("Usage: pe_parser path");
            exit(0);
        }
    };
    let pe_file = File::open(&path)?;
    let mut parser = PortExeParser::new(pe_file);
    let file_header = parser.file_header();
    println!("{file_header:X?}");
    Ok(())
}

#[derive(Debug)]
pub struct PortExeParser<R> {
    inner: R,
    pe_type: PortExeType,
    file_header_offset: u64,
}

impl<R: Read + Seek> PortExeParser<R> {
    fn new(mut inner: R) -> Self {
        let mut mz = [0u8; 2];
        inner.seek(SeekFrom::Start(0)).expect("Should be seekable");
        inner.read_exact(&mut mz).expect("Should be readable");
        let pe_type = if mz == [b'M', b'Z'] {
            PortExeType::PortExeImage
        } else {
            PortExeType::PortExeObject
        };
        let file_header_offset = match pe_type {
            PortExeType::PortExeImage => {
                let mut bytes = [0u8; 4];
                inner
                    .seek(SeekFrom::Start(0x3C))
                    .expect("Should be seekable");
                inner.read_exact(&mut bytes).expect("Should be readable");
                u32::from_le_bytes(bytes) as u64
            }
            PortExeType::PortExeObject => 0,
        };
        Self {
            inner,
            pe_type,
            file_header_offset,
        }
    }
}

impl<R: Read + Seek> PortExeParse for PortExeParser<R> {
    fn file_header(&mut self) -> FileHeader {
        let mut machine = [0u8; 2];
        let mut number_of_sections = [0u8; 2];
        let mut time_date_stamp = [0u8; 4];
        let mut pointer_to_symbol_table = [0u8; 4];
        let mut number_of_symbols = [0u8; 4];
        let mut size_of_optional_header = [0u8; 2];
        let mut characteristics = [0u8; 2];

        self.inner
            .seek(SeekFrom::Start(self.file_header_offset + 4));
        self.inner.read_exact(&mut machine);
        self.inner.read_exact(&mut number_of_sections);
        self.inner.read_exact(&mut time_date_stamp);
        self.inner.read_exact(&mut pointer_to_symbol_table);
        self.inner.read_exact(&mut number_of_symbols);
        self.inner.read_exact(&mut size_of_optional_header);
        self.inner.read_exact(&mut characteristics);

        FileHeader {
            machine,
            number_of_sections,
            time_date_stamp,
            pointer_to_symbol_table,
            number_of_symbols,
            size_of_optional_header,
            characteristics,
        }
    }
}

trait PortExeParse {
    fn file_header(&mut self) -> FileHeader;
}

trait PortExeImageParse: PortExeParse {
    fn optional_header(&self);
    fn data_directories(&self);
}

trait PortExeObjectParse: PortExeParse {}

#[derive(Debug)]
pub enum PortExeType {
    PortExeObject,
    PortExeImage,
}

#[derive(Debug)]
pub struct FileHeader {
    machine: [u8; 2],
    number_of_sections: [u8; 2],
    time_date_stamp: [u8; 4],
    pointer_to_symbol_table: [u8; 4],
    number_of_symbols: [u8; 4],
    size_of_optional_header: [u8; 2],
    characteristics: [u8; 2],
}

impl Display for FileHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub struct OptHeader32 {
    magic: u16,
    major_linker_version: u8,
    minor_linker_version: u8,
    size_of_code: u32,
    size_of_initialized_data: u32,
    size_of_uninitialized_data: u32,
    address_of_entry_point: u32,
    base_of_code: u32,
    base_of_data: u32,
    image_base: u32,
    section_alignment: u32,
    file_alignment: u32,
    major_operating_system_version: u16,
    minor_operating_system_version: u16,
    major_image_version: u16,
    minor_image_version: u16,
    major_subsystem_version: u16,
    minor_subsystem_version: u16,
    win32_version_value: u32,
    size_of_image: u32,
    size_of_headers: u32,
    check_sum: u32,
    subsystem: u16,
    dll_characteristics: u16,
    size_of_stack_reserve: u32,
    size_of_stack_commit: u32,
    size_of_heap_reserve: u32,
    size_of_heap_commit: u32,
    loader_flags: u32,
    number_of_rva_and_sizes: u32,
}

pub struct DataDir {
    virtual_address: [u8; 4],
    size: [u8; 4],
}

pub struct Section {
    name: [u8; 8],
    virtual_size: [u8; 4],
    virtual_address: [u8; 4],
    size_of_raw_data: [u8; 4],
    pointer_to_raw_data: [u8; 4],
    pointer_to_relocations: [u8; 4],
    pointer_to_linenumbers: [u8; 4],
    number_of_relocations: [u8; 2],
    number_of_linenumbers: [u8; 2],
    characteristics: [u8; 4],
}

impl Section {
    fn name(&self) -> String {
        self.name.iter().map(|&b| b as char).collect()
    }
}
