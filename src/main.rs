use std::env::args;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::process::exit;

use chrono::{DateTime, TimeZone, Utc};
use machine_types::Machine;

pub mod machine_types {
    use std::fmt::Display;

    pub const IMAGE_FILE_MACHINE_UNKNOWN: [u8; 2] = [0x00, 0x00]; //The content of this field is assumed to be applicable to any machine type
    pub const IMAGE_FILE_MACHINE_AM33: [u8; 2] = [0xD3, 0x01]; // Matsushita AM33
    pub const IMAGE_FILE_MACHINE_AMD64: [u8; 2] = [0x64, 0x86]; // x64
    pub const IMAGE_FILE_MACHINE_ARM: [u8; 2] = [0xC0, 0x01]; // ARM little endian
    pub const IMAGE_FILE_MACHINE_ARM64: [u8; 2] = [0x64, 0xAA]; // ARM64 little endian
    pub const IMAGE_FILE_MACHINE_ARMNT: [u8; 2] = [0xC4, 0x01]; // ARM Thumb-2 little endian
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
    pub const IMAGE_FILE_MACHINE_RISCV32: [u8; 2] = [0x32, 0x50]; // RISC-V 32-bit address space
    pub const IMAGE_FILE_MACHINE_RISCV64: [u8; 2] = [0x64, 0x50]; // RISC-V 64-bit address space
    pub const IMAGE_FILE_MACHINE_RISCV128: [u8; 2] = [0x28, 0x51]; // RISC-V 128-bit address space
    pub const IMAGE_FILE_MACHINE_SH3: [u8; 2] = [0xA2, 0x01]; // Hitachi SH3
    pub const IMAGE_FILE_MACHINE_SH3DSP: [u8; 2] = [0xA3, 0x01]; // Hitachi SH3 DSP
    pub const IMAGE_FILE_MACHINE_SH4: [u8; 2] = [0xA6, 0x01]; // Hitachi SH4
    pub const IMAGE_FILE_MACHINE_SH5: [u8; 2] = [0xA8, 0x01]; // Hitachi SH5
    pub const IMAGE_FILE_MACHINE_THUMB: [u8; 2] = [0xC2, 0x01]; // Thumb
    pub const IMAGE_FILE_MACHINE_WCEMIPSV2: [u8; 2] = [0x69, 0x01]; // MIPS little-endian WCE v2

    pub struct Machine {
        raw: [u8; 2],
    }

    impl From<[u8; 2]> for Machine {
        fn from(value: [u8; 2]) -> Self {
            Self { raw: value }
        }
    }

    impl Display for Machine {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let data = match self.raw {
                IMAGE_FILE_MACHINE_UNKNOWN => "Any machine type",
                IMAGE_FILE_MACHINE_AM33 => "Matsushita AM33",
                IMAGE_FILE_MACHINE_AMD64 => "x64",
                IMAGE_FILE_MACHINE_ARM => "ARM little endian",
                IMAGE_FILE_MACHINE_ARM64 => "ARM64 little endian",
                IMAGE_FILE_MACHINE_ARMNT => "ARM Thumb-2 little endian",
                IMAGE_FILE_MACHINE_EBC => "EFI byte code",
                IMAGE_FILE_MACHINE_I386 => {
                    "Intel 386 or later processors and compatible processors"
                }
                IMAGE_FILE_MACHINE_IA64 => "Intel Itanium processor family",
                IMAGE_FILE_MACHINE_LOONGARCH32 => "LoongArch 32-bit processor family",
                IMAGE_FILE_MACHINE_LOONGARCH64 => "LoongArch 64-bit processor family",
                IMAGE_FILE_MACHINE_M32R => "Mitsubishi M32R little endian",
                IMAGE_FILE_MACHINE_MIPS16 => "MIPS16",
                IMAGE_FILE_MACHINE_MIPSFPU => "MIPS with FPU",
                IMAGE_FILE_MACHINE_MIPSFPU16 => "MIPS16 with FPU",
                IMAGE_FILE_MACHINE_POWERPC => "Power PC little endian",
                IMAGE_FILE_MACHINE_POWERPCFP => "Power PC with floating point support",
                IMAGE_FILE_MACHINE_R4000 => "MIPS little endian",
                IMAGE_FILE_MACHINE_RISCV32 => "RISC-V 32-bit address space",
                IMAGE_FILE_MACHINE_RISCV64 => "RISC-V 64-bit address space",
                IMAGE_FILE_MACHINE_RISCV128 => "RISC-V 128-bit address space",
                IMAGE_FILE_MACHINE_SH3 => "Hitachi SH3",
                IMAGE_FILE_MACHINE_SH3DSP => "Hitachi SH3 DSP",
                IMAGE_FILE_MACHINE_SH4 => "Hitachi SH4",
                IMAGE_FILE_MACHINE_SH5 => "Hitachi SH5",
                IMAGE_FILE_MACHINE_THUMB => "Thumb",
                IMAGE_FILE_MACHINE_WCEMIPSV2 => "MIPS little-endian WCE v2",
                _ => return Err(std::fmt::Error),
            };
            f.write_str(data)
        }
    }
}

pub mod characteristics {
    pub const IMAGE_FILE_RELOCS_STRIPPED: [u8; 2] = [0x01, 0x00]; // Image only, Windows CE, and Microsoft Windows NT and later. This indicates that the file does not contain base relocations and must therefore be loaded at its preferred base address. If the base address is not available, the loader reports an error. The default behavior of the linker is to strip base relocations from executable (EXE) files.
    pub const IMAGE_FILE_EXECUTABLE_IMAGE: [u8; 2] = [0x02, 0x00]; // Image only. This indicates that the image file is valid and can be run. If this flag is not set, it indicates a linker error.
    pub const IMAGE_FILE_LINE_NUMS_STRIPPED: [u8; 2] = [0x04, 0x00]; // COFF line numbers have been removed. This flag is deprecated and should be zero.
    pub const IMAGE_FILE_LOCAL_SYMS_STRIPPED: [u8; 2] = [0x08, 0x00]; // COFF symbol table entries for local symbols have been removed. This flag is deprecated and should be zero.
    pub const IMAGE_FILE_AGGRESSIVE_WS_TRIM: [u8; 2] = [0x10, 0x00]; // Obsolete. Aggressively trim working set. This flag is deprecated for Windows 2000 and later and must be zero.
    pub const IMAGE_FILE_LARGE_ADDRESS_AWARE: [u8; 2] = [0x20, 0x00]; // Application can handle > 2-GB addresses.
    pub const IMAGE_FILE_RESERVED0: [u8; 2] = [0x40, 0x00]; // This flag is reserved for future use.
    pub const IMAGE_FILE_BYTES_REVERSED_LO: [u8; 2] = [0x80, 0x00]; // Little endian: the least significant bit (LSB) precedes the most significant bit (MSB) in memory. This flag is deprecated and should be zero.
    pub const IMAGE_FILE_32BIT_MACHINE: [u8; 2] = [0x00, 0x01]; // Machine is based on a 32-bit-word architecture.
    pub const IMAGE_FILE_DEBUG_STRIPPED: [u8; 2] = [0x00, 0x02]; // Debugging information is removed from the image file.
    pub const IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP: [u8; 2] = [0x00, 0x04]; // If the image is on removable media, fully load it and copy it to the swap file.
    pub const IMAGE_FILE_NET_RUN_FROM_SWAP: [u8; 2] = [0x00, 0x08]; // If the image is on network media, fully load it and copy it to the swap file.
    pub const IMAGE_FILE_SYSTEM: [u8; 2] = [0x00, 0x10]; // The image file is a system file, not a user program.
    pub const IMAGE_FILE_DLL: [u8; 2] = [0x00, 0x20]; // The image file is a dynamic-link library (DLL). Such files are considered executable files for almost all purposes, although they cannot be directly run.
    pub const IMAGE_FILE_UP_SYSTEM_ONLY: [u8; 2] = [0x00, 0x40]; // The file should be run only on a uniprocessor machine.
    pub const IMAGE_FILE_BYTES_REVERSED_HI: [u8; 2] = [0x00, 0x80]; // Big endian: the MSB precedes the LSB in memory. This flag is deprecated and should be zero.
}

pub mod section_flags {
    pub const IMAGE_SCN_TYPE_NO_PAD: [u8; 4] = [0x08, 0x00, 0x00, 0x00]; // The section should not be padded to the next boundary. This flag is obsolete and is replaced by IMAGE_SCN_ALIGN_1BYTES. This is valid only for object files.
    pub const IMAGE_SCN_CNT_CODE: [u8; 4] = [0x20, 0x00, 0x00, 0x00]; // The section contains executable code.
    pub const IMAGE_SCN_CNT_INITIALIZED_DATA: [u8; 4] = [0x40, 0x00, 0x00, 0x00]; // The section contains initialized data.
    pub const IMAGE_SCN_CNT_UNINITIALIZED_DATA: [u8; 4] = [0x80, 0x00, 0x00, 0x00]; // The section contains uninitialized data.
    pub const IMAGE_SCN_LNK_OTHER: [u8; 4] = [0x00, 0x01, 0x00, 0x00]; // Reserved for future use.
    pub const IMAGE_SCN_LNK_INFO: [u8; 4] = [0x00, 0x02, 0x00, 0x00]; // The section contains comments or other information. The .drectve section has this type. This is valid for object files only.
    pub const IMAGE_SCN_LNK_REMOVE: [u8; 4] = [0x00, 0x08, 0x00, 0x00]; // The section will not become part of the image. This is valid only for object files.
    pub const IMAGE_SCN_LNK_COMDAT: [u8; 4] = [0x00, 0x10, 0x00, 0x00]; // The section contains COMDAT data. For more information, see COMDAT Sections (Object Only). This is valid only for object files.
    pub const IMAGE_SCN_GPREL: [u8; 4] = [0x00, 0x80, 0x00, 0x00]; // The section contains data referenced through the global pointer (GP).
    pub const IMAGE_SCN_ALIGN_1BYTES: [u8; 4] = [0x00, 0x00, 0x10, 0x00]; // Align data on a 1-byte boundary. Valid only for object files.
    pub const IMAGE_SCN_ALIGN_2BYTES: [u8; 4] = [0x00, 0x00, 0x20, 0x00]; // Align data on a 2-byte boundary. Valid only for object files.
    pub const IMAGE_SCN_ALIGN_4BYTES: [u8; 4] = [0x00, 0x00, 0x30, 0x00]; // Align data on a 4-byte boundary. Valid only for object files.
    pub const IMAGE_SCN_ALIGN_8BYTES: [u8; 4] = [0x00, 0x00, 0x40, 0x00]; // Align data on an 8-byte boundary. Valid only for object files.
    pub const IMAGE_SCN_ALIGN_16BYTES: [u8; 4] = [0x00, 0x00, 0x50, 0x00]; // Align data on a 16-byte boundary. Valid only for object files.
    pub const IMAGE_SCN_ALIGN_32BYTES: [u8; 4] = [0x00, 0x00, 0x60, 0x00]; // Align data on a 32-byte boundary. Valid only for object files.
    pub const IMAGE_SCN_ALIGN_64BYTES: [u8; 4] = [0x00, 0x00, 0x70, 0x00]; // Align data on a 64-byte boundary. Valid only for object files.
    pub const IMAGE_SCN_ALIGN_128BYTES: [u8; 4] = [0x00, 0x00, 0x80, 0x00]; // Align data on a 128-byte boundary. Valid only for object files.
    pub const IMAGE_SCN_ALIGN_256BYTES: [u8; 4] = [0x00, 0x00, 0x90, 0x00]; // Align data on a 256-byte boundary. Valid only for object files.
    pub const IMAGE_SCN_ALIGN_512BYTES: [u8; 4] = [0x00, 0x00, 0xA0, 0x00]; // Align data on a 512-byte boundary. Valid only for object files.
    pub const IMAGE_SCN_ALIGN_1024BYTES: [u8; 4] = [0x00, 0x00, 0xB0, 0x00]; // Align data on a 1024-byte boundary. Valid only for object files.
    pub const IMAGE_SCN_ALIGN_2048BYTES: [u8; 4] = [0x00, 0x00, 0xC0, 0x00]; // Align data on a 2048-byte boundary. Valid only for object files.
    pub const IMAGE_SCN_ALIGN_4096BYTES: [u8; 4] = [0x00, 0x00, 0xD0, 0x00]; // Align data on a 4096-byte boundary. Valid only for object files.
    pub const IMAGE_SCN_ALIGN_8192BYTES: [u8; 4] = [0x00, 0x00, 0xE0, 0x00]; // Align data on an 8192-byte boundary. Valid only for object files.
    pub const IMAGE_SCN_LNK_NRELOC_OVFL: [u8; 4] = [0x00, 0x00, 0x00, 0x01]; // The section contains extended relocations.
    pub const IMAGE_SCN_MEM_DISCARDABLE: [u8; 4] = [0x00, 0x00, 0x00, 0x02]; // The section can be discarded as needed.
    pub const IMAGE_SCN_MEM_NOT_CACHED: [u8; 4] = [0x00, 0x00, 0x00, 0x04]; // The section cannot be cached.
    pub const IMAGE_SCN_MEM_NOT_PAGED: [u8; 4] = [0x00, 0x00, 0x00, 0x08]; // The section is not pageable.
    pub const IMAGE_SCN_MEM_SHARED: [u8; 4] = [0x00, 0x00, 0x00, 0x10]; // The section can be shared in memory.
    pub const IMAGE_SCN_MEM_EXECUTE: [u8; 4] = [0x00, 0x00, 0x00, 0x20]; // The section can be executed as code.
    pub const IMAGE_SCN_MEM_READ: [u8; 4] = [0x00, 0x00, 0x00, 0x40]; // The section can be read.
    pub const IMAGE_SCN_MEM_WRITE: [u8; 4] = [0x00, 0x00, 0x00, 0x80]; // The section can be written to.
}

pub const IMAGE_NT_OPTIONAL_HDR32_MAGIC: [u8; 2] = [0x0B, 0x01]; // The file is an executable image of 32-bit application
pub const IMAGE_NT_OPTIONAL_HDR64_MAGIC: [u8; 2] = [0x0B, 0x02]; // The file is an executable image of 64-bit application
pub const IMAGE_ROM_OPTIONAL_HDR_MAGIC: [u8; 2] = [0x07, 0x01]; // The file is a ROM image.

enum PortExeImageType {
    PortExeImage32,
    PortExeImage64,
    PortExeImageRom,
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
                (u32::from_le_bytes(bytes) as u64) + 4
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
        read_file_header(&mut self.inner, self.file_header_offset).unwrap()
    }
}

fn read_file_header<R: Read + Seek>(
    reader: &mut R,
    file_header_offset: u64,
) -> io::Result<FileHeader> {
    let mut machine = [0u8; 2];
    let mut number_of_sections = [0u8; 2];
    let mut time_date_stamp = [0u8; 4];
    let mut pointer_to_symbol_table = [0u8; 4];
    let mut number_of_symbols = [0u8; 4];
    let mut size_of_optional_header = [0u8; 2];
    let mut characteristics = [0u8; 2];

    reader.seek(SeekFrom::Start(file_header_offset))?;
    reader.read_exact(&mut machine)?;
    reader.read_exact(&mut number_of_sections)?;
    reader.read_exact(&mut time_date_stamp)?;
    reader.read_exact(&mut pointer_to_symbol_table)?;
    reader.read_exact(&mut number_of_symbols)?;
    reader.read_exact(&mut size_of_optional_header)?;
    reader.read_exact(&mut characteristics)?;

    Ok(FileHeader {
        machine,
        number_of_sections,
        time_date_stamp,
        pointer_to_symbol_table,
        number_of_symbols,
        size_of_optional_header,
        characteristics,
    })
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

pub struct OptionalHeader {
    magic: [u8; 2],
    major_linker_version: [u8; 1],
    minor_linker_version: [u8; 1],
    size_of_code: [u8; 4],
    size_of_initialized_data: [u8; 4],
    size_of_uninitialized_data: [u8; 4],
    address_of_entry_point: [u8; 4],
    base_of_code: [u8; 4],
    base_of_data: Option<[u8; 4]>,
    image_base: [u8; 8],
    section_alignment: [u8; 4],
    file_alignment: [u8; 4],
    major_operating_system_version: [u8; 2],
    minor_operating_system_version: [u8; 2],
    major_image_version: [u8; 2],
    minor_image_version: [u8; 2],
    major_subsystem_version: [u8; 2],
    minor_subsystem_version: [u8; 2],
    win32_version_value: [u8; 4],
    size_of_image: [u8; 4],
    size_of_headers: [u8; 4],
    check_sum: [u8; 4],
    subsystem: [u8; 2],
    dll_characteristics: [u8; 2],
    size_of_stack_reserve: [u8; 8],
    size_of_stack_commit: [u8; 8],
    size_of_heap_reserve: [u8; 8],
    size_of_heap_commit: [u8; 8],
    loader_flags: [u8; 4],
    number_of_rva_and_sizes: [u8; 4],
}

impl OptionalHeader {
    fn image_type(&self) -> PortExeImageType {
        match self.magic {
            IMAGE_NT_OPTIONAL_HDR32_MAGIC => PortExeImageType::PortExeImage32,
            IMAGE_NT_OPTIONAL_HDR64_MAGIC => PortExeImageType::PortExeImage64,
            IMAGE_ROM_OPTIONAL_HDR_MAGIC => PortExeImageType::PortExeImageRom,
            _ => panic!()
        }
    }

    fn linker_version(&self) -> String {
        todo!()
    }

    fn os_version(&self) -> String {
        todo!()
    }

    fn image_version(&self) -> String {
        todo!()
    }
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

trait BitFlags {}
