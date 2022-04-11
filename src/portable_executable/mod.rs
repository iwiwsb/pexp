mod coff_header;
use coff_header::COFFFileHeader;

use std::{
    fs::{File, OpenOptions},
    io,
    path::Path,
};

#[allow(non_camel_case_types)]
enum Characteristics {
    RELOCS_STRIPPED,
    EXECUTABLE_IMAGE,
    LINE_NUMS_STRIPPED,
    SYMS_STRIPPED,
    AGGRESSIVE_WS_TRIM,
    LARGE_ADDRESS_AWARE,
    BYTES_REVERSED_LO,
    MACHINE_32BIT,
    DEBUG_STRIPPED,
    REMOVABLE_RUN_FROM_SWAP,
    RUN_FROM_SWAP,
    SYSTEM,
    DLL,
    SYSTEM_ONLY,
    BYTES_REVERSED_HI,
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
            [0xB, 0x1] => Ok(Self::PE32),
            [0xB, 0x2] => Ok(Self::PE64),
            _ => Err("invalid image type"),
        }
    }
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

impl TryFrom<[u8; 2]> for Subsystem {
    type Error = &'static str;

    fn try_from(value: [u8; 2]) -> Result<Self, Self::Error> {
        match value {
            [0x00, 0x00] => Ok(Self::UNKNOWN),
            [0x01, 0x00] => Ok(Self::NATIVE),
            [0x02, 0x00] => Ok(Self::WINDOWS_GUI),
            [0x03, 0x00] => Ok(Self::WINDOWS_CUI),
            [0x05, 0x00] => Ok(Self::OS2_CUI),
            [0x07, 0x00] => Ok(Self::POSIX_CUI),
            [0x08, 0x00] => Ok(Self::NATIVE_WINDOWS),
            [0x09, 0x00] => Ok(Self::WINDOWS_CE_GUI),
            [0x0A, 0x00] => Ok(Self::EFI_APPLICATION),
            [0x0B, 0x00] => Ok(Self::EFI_BOOT_SERVICE_DRIVER),
            [0x0C, 0x00] => Ok(Self::EFI_RUNTIME_DRIVER),
            [0x0D, 0x00] => Ok(Self::EFI_ROM),
            [0x0E, 0x00] => Ok(Self::XBOX),
            [0x10, 0x00] => Ok(Self::WINDOWS_BOOT_APPLICATION),
            _ => Err("invalid subsystem type"),
        }
    }
}

struct SectionTable {
    name: String,
    virtual_size: u32,
    virtual_address: u32,
    size_of_raw_data: u32,
    pointer_to_raw_data: u32,
    pointer_to_relocations: u32,
    pointer_to_line_numbers: u32,
    number_of_relocations: u32,
    number_of_line_numbers: u32,
    characteristics: u32,
}

struct DataDir {
    virtual_address: u32,
    size: u32,
}

struct DataDirectories {
    export: DataDir,
    import: DataDir,
    resource: DataDir,
    exception: DataDir,
    certificate: DataDir,
    base_relocation: DataDir,
    debug: DataDir,
    architecture: DataDir,
    global_ptr: DataDir,
    tls: DataDir,
    load_config: DataDir,
    bound_import: DataDir,
    iat: DataDir,
    delay_import_descriptor: DataDir,
    clr_runtime_header: DataDir,
    reserved: DataDir,
}

pub struct PEHeaders {
    ms_dos_stub: Option<Vec<u8>>,
    signature: [u8; 4],
    coff_file_header: COFFFileHeader,
    optional_header: Option<OptionalHeader>,
    section_table: SectionTable,
}

struct PEParser {
    pe_file: File,
}

impl PEParser {
    fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let pe_file = OpenOptions::new().read(true).open(path)?;
        Ok(Self { pe_file })
    }

    fn parse(&self) -> io::Result<PEHeaders> {
        todo!()
    }
}
