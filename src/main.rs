#![allow(dead_code)]

use chrono::{DateTime, Utc};
use std::env::args;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::process::exit;

#[allow(non_camel_case_types)]
enum MachineType {
    UNKNOWN = 0x0000, // The content of this field is assumed to be applicable to any machine type
    AM33 = 0x01D3,    // Matsushita AM33
    AMD64 = 0x8664,   // x64
    ARM = 0x01C0,     // ARM little endian
    ARM64 = 0xAA64,   // ARM64 little endian
    ARMNT = 0x01C4,   // ARM Thumb-2 little endian
    EBC = 0x0EBC,     // EFI byte code
    I386 = 0x014C,    // Intel 386 or later processors and compatible processors
    IA64 = 0x0200,    // Intel Itanium processor family
    LOONGARCH32 = 0x6232, // LoongArch 32-bit processor family
    LOONGARCH64 = 0x6264, // LoongArch 64-bit processor family
    M32R = 0x9041,    // Mitsubishi M32R little endian
    MIPS16 = 0x0266,  // MIPS16
    MIPSFPU = 0x0366, // MIPS with FPU
    MIPSFPU16 = 0x0466, //MIPS16 with FPU
    POWERPC = 0x01F0, // Power PC little endian
    POWERPCFP = 0x01F1, // Power PC with floating point support
    R4000 = 0x0166,   //MIPS little endian
    RISCV32 = 0x5032, //RISC-V 32-bit address space
    RISCV64 = 0x5064, //RISC-V 64-bit address space
    RISCV128 = 0x5128, //RISC-V 128-bit address space
    SH3 = 0x01A2,     // Hitachi SH3
    SH3DSP = 0x01A3,  // Hitachi SH3 DSP
    SH4 = 0x01A6,     // Hitachi SH4
    SH5 = 0x01A8,     //Hitachi SH5
    THUMB = 0x01C2,   //Thumb
    WCEMIPSV2 = 0x0169, //MIPS little-endian WCE v2
}

#[allow(non_camel_case_types)]
enum Characteristics {
    RELOCS_STRIPPED,
    EXECUTABLE_IMAGE,
    LINE_NUMS_STRIPPED,
    SYMS_STRIPPED,
    AGGRESSIVE_WS_TRIM,
    LARGE_ADDRESS_AWARE,
    BYTES_REVERSED_LO,
    X32BIT_MACHINE,
    DEBUG_STRIPPED,
    REMOVABLE_RUN_FROM_SWAP,
    RUN_FROM_SWAP,
    SYSTEM,
    DLL,
    SYSTEM_ONLY,
    BYTES_REVERSED_HI,
}

impl TryFrom<u16> for MachineType {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0000 => Ok(Self::UNKNOWN),
            0x01D3 => Ok(Self::AM33),
            0x8664 => Ok(Self::AMD64),
            0x01C0 => Ok(Self::ARM),
            0xAA64 => Ok(Self::ARM64),
            0x01C4 => Ok(Self::ARMNT),
            0x0EBC => Ok(Self::EBC),
            0x014C => Ok(Self::I386),
            0x0200 => Ok(Self::IA64),
            0x6232 => Ok(Self::LOONGARCH32),
            0x6264 => Ok(Self::LOONGARCH64),
            0x9041 => Ok(Self::M32R),
            0x0266 => Ok(Self::MIPS16),
            0x0366 => Ok(Self::MIPSFPU),
            0x0466 => Ok(Self::MIPSFPU16),
            0x01F0 => Ok(Self::POWERPC),
            0x01F1 => Ok(Self::POWERPCFP),
            0x0166 => Ok(Self::R4000),
            0x5032 => Ok(Self::RISCV32),
            0x5064 => Ok(Self::RISCV64),
            0x5128 => Ok(Self::RISCV128),
            0x01A2 => Ok(Self::SH3),
            0x01A3 => Ok(Self::SH3DSP),
            0x01A6 => Ok(Self::SH4),
            0x01A8 => Ok(Self::SH5),
            0x01C2 => Ok(Self::THUMB),
            0x0169 => Ok(Self::WCEMIPSV2),
            _ => Err("invalid machine type"),
        }
    }
}

#[allow(non_camel_case_types)]
enum PEImageType {
    PE32,
    PE64,
}

impl TryFrom<u16> for PEImageType {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x010B => Ok(Self::PE32),
            0x020B => Ok(Self::PE64),
            _ => Err("invalid image type"),
        }
    }
}

impl TryFrom<[u8; 2]> for PEImageType {
    type Error = &'static str;

    fn try_from(value: [u8; 2]) -> Result<Self, Self::Error> {
        match value {
            [0x1, 0xB] => Ok(Self::PE32),
            [0x2, 0xB] => Ok(Self::PE64),
            _ => Err("invalid image type"),
        }
    }
}

#[allow(non_camel_case_types)]
struct COFFFileHeader {
    machine: MachineType,
    number_of_sections: u16,
    time_date_stamp: DateTime<Utc>,
    pointer_to_symbol_table: Option<u32>,
    number_of_symbols: u32,
    size_of_optional_header: u16,
    characteristics: u32,
}

struct OptionalHeader {
    standard_fields: StandardFields,
    windows_specific_fields: WindowsSpecificFields,
    data_directories: DataDirectories,
}

struct StandardFields {
    magic: PEImageType,
    major_linker_version: u8,
    minor_linker_version: u8,
    size_of_code: u32,
    size_of_initialized_data: u32,
    size_of_uninitialized_data: u32,
    address_of_entry_point: u32,
    base_of_code: u32,
    base_of_data: Option<u32>,
}

#[allow(non_snake_case)]
struct WindowsSpecificFields {
    image_base: u64,
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
    subsystem: Subsystem,
    dll_characteristics: u16,
    size_of_stack_reserve: u64,
    size_of_stack_commit: u64,
    size_of_heap_reserve: u64,
    size_of_heap_commit: u64,
    loader_flags: u32,
    number_of_RVA_and_sizes: u32,
}

#[allow(non_camel_case_types)]
enum Subsystem {
    UNKNOWN,
    NATIVE,
    WINDOWS_GUI,
    WINDOWS_CUI,
    OS2_CUI,
    POSIX_CUI,
    NATIVE_WINDOWS,
    WINDOWS_CE_GUI,
    EFI_APPLICATION,
    EFI_BOOT_SERVICE_DRIVER,
    EFI_RUNTIME_DRIVER,
    EFI_ROM,
    XBOX,
    WINDOWS_BOOT_APPLICATION,
}

impl TryFrom<u16> for Subsystem {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::UNKNOWN),
            1 => Ok(Self::NATIVE),
            2 => Ok(Self::WINDOWS_GUI),
            3 => Ok(Self::WINDOWS_CUI),
            5 => Ok(Self::OS2_CUI),
            7 => Ok(Self::POSIX_CUI),
            8 => Ok(Self::NATIVE_WINDOWS),
            9 => Ok(Self::WINDOWS_CE_GUI),
            10 => Ok(Self::EFI_APPLICATION),
            11 => Ok(Self::EFI_BOOT_SERVICE_DRIVER),
            12 => Ok(Self::EFI_RUNTIME_DRIVER),
            13 => Ok(Self::EFI_ROM),
            14 => Ok(Self::XBOX),
            16 => Ok(Self::WINDOWS_BOOT_APPLICATION),
            _ => Err("invalid subsystem type"),
        }
    }
}

struct SectionTable {}

struct DataDirectories {}

struct PortableExecutable {
    ms_dos_stub: Option<Vec<u8>>,
    signature: [u8; 4],
    coff_file_header: COFFFileHeader,
    optional_header: Option<OptionalHeader>,

}

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
