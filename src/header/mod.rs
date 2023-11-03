pub mod characteristics;
pub mod dll_characteristics;
pub mod machine_types;
pub mod section_flags;
pub mod section_header;
pub mod win_subsystem;

use chrono::NaiveDateTime;

use std::fmt::{self, Debug, Display};
use std::io::{Read, Seek};

use self::machine_types::Machine;

/// The file is an executable image of 32-bit application
pub const IMAGE_NT_OPTIONAL_HDR32_MAGIC: [u8; 2] = [0x0B, 0x01];
/// The file is an executable image of 64-bit application
pub const IMAGE_NT_OPTIONAL_HDR64_MAGIC: [u8; 2] = [0x0B, 0x02];
/// The file is a ROM image.
pub const IMAGE_ROM_OPTIONAL_HDR_MAGIC: [u8; 2] = [0x07, 0x01];
/// Size of COFF File Header
pub const FILE_HEADER_SIZE: u64 = 28;

#[derive(Debug)]
pub enum ImageType {
    /// Represents 32-bit PE image
    Image32,
    /// Represents 64-bit PE image
    Image64,
    /// Represents ROM PE Image
    ImageRom,
    ImageUnknown,
}

impl From<u16> for ImageType {
    fn from(value: u16) -> Self {
        match value {
            0x010B => ImageType::Image32,
            0x020B => ImageType::Image64,
            0x0107 => ImageType::ImageRom,
            _ => ImageType::ImageUnknown,
        }
    }
}

impl From<[u8; 2]> for ImageType {
    fn from(value: [u8; 2]) -> Self {
        let x = u16::from_le_bytes(value);
        ImageType::from(x)
    }
}

impl Display for ImageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageType::Image32 => write!(f, "32-bit PE image"),
            ImageType::Image64 => write!(f, "64-bit PE image"),
            ImageType::ImageRom => write!(f, "ROM PE image"),
            ImageType::ImageUnknown => write!(f, "Unknown image"),
        }
    }
}

/// Relative virtual address (RVA)
///
/// In an image file, this is the address of an item after it is loaded into memory, with the base address of the image file subtracted from it.
/// The RVA of an item almost always differs from its position within the file on disk (file pointer).
/// In an object file, an RVA is less meaningful because memory locations are not assigned.
/// In this case, an RVA would be an address within a section (described later in this table), to which a relocation is later applied during linking.
/// For simplicity, a compiler should just set the first RVA in each section to zero.
#[derive(Debug)]
pub struct RelativeVirtualAddress {
    addr: u64,
}

/// Virtual address (VA)
///
/// Same as [RVA](RelativeVirtualAddress), except that the base address of the image file is not subtracted.
/// The address is called a VA because Windows creates a distinct VA space for each process, independent of physical memory.
/// For almost all purposes, a VA should be considered just an address.
/// A VA is not as predictable as an [RVA](RelativeVirtualAddress) because the loader might not load the image at its preferred location.
#[derive(Debug)]
pub struct VirtualAddress {
    addr: u64,
}

/// COFF File Header structure
#[derive(Debug, PartialEq)]
pub struct FileHeader {
    /// Identifies the type of target machine. For more information, see [`machine_types`](crate::header::machine_types).
    pub machine: Machine,
    /// Indicates the size of the section table, which immediately follows the headers.
    pub number_of_sections: u16,
    /// The low 32 bits of the number of seconds since 00:00 January 1, 1970 (a C run-time time_t value), which indicates when the file was created.
    pub time_date_stamp: NaiveDateTime,
    /// The file offset of the COFF symbol table, or zero if no COFF symbol table is present.
    /// This value should be zero for an image because COFF debugging information is deprecated.
    pub pointer_to_symbol_table: u32,
    /// The number of entries in the symbol table.
    /// This data can be used to locate the string table, which immediately follows the symbol table.
    /// This value should be zero for an image because COFF debugging information is deprecated.
    pub number_of_symbols: u32,
    /// The size of the [`OptionalHeader`](crate::header::optional_header::OptionalHeader), which is required for executable files but not for object files.
    /// This value should be zero for an object file.
    pub size_of_optional_header: u16,
    /// The flags that indicate the attributes of the file. For specific flag values, see [`characteristics`](crate::header::characteristics)
    pub characteristics: u16,
}

impl FileHeader {
    pub fn read_from<R: Read + Seek>(reader: &mut R) -> Self {
        let machine = Machine::from(u16::from_le_bytes(Self::read_array(reader)));
        let number_of_sections = u16::from_le_bytes(Self::read_array(reader));
        let time_date_stamp =
            NaiveDateTime::from_timestamp(u32::from_le_bytes(Self::read_array(reader)) as i64, 0);
        let pointer_to_symbol_table = u32::from_le_bytes(Self::read_array(reader));
        let number_of_symbols = u32::from_le_bytes(Self::read_array(reader));
        let size_of_optional_header = u16::from_le_bytes(Self::read_array(reader));
        let characteristics = u16::from_le_bytes(Self::read_array(reader));
        Self {
            machine,
            number_of_sections,
            time_date_stamp,
            pointer_to_symbol_table,
            number_of_symbols,
            size_of_optional_header,
            characteristics,
        }
    }

    fn read_array<R: Read + Seek, const N: usize>(reader: &mut R) -> [u8; N] {
        let mut buf = [0u8; N];
        reader
            .read_exact(&mut buf)
            .expect("Data stream should be readable");
        buf
    }
}

/// Optional Header structure
///
/// Every image file has an optional header that provides information to the loader.
/// This header is optional in the sense that some files (specifically, object files) do not have it.
/// For image files, this header is required.
/// An object file can have an optional header, but generally this header has no function in an object file except to increase its size.
/// Note that the size of the optional header is not fixed.
/// The [`size_of_optional_header`](crate::header::file_header::FileHeader#structfield.size_of_optional_header) field in the COFF header must be used
/// to validate that a probe into the file for a particular data directory does not go beyond [`size_of_optional_header`](crate::header::file_header::FileHeader#structfield.size_of_optional_header).
///
/// The first 8 fields of the optional header are standard fields that are defined for every implementation of COFF.
/// PE32 contains additional field `base_of_data`, which is absent in PE32+, following `base_of_code`.
/// These fields contain general information that is useful for loading and running an executable file. They are unchanged for the PE32+ format.
/// The next 21 fields are an extension to the COFF optional header format. They contain additional information that is required by the linker and loader in Windows.
#[derive(Debug)]
pub struct OptionalHeader {
    /// Identifies the state of the image file.
    /// The most common number is `0x10B`, which identifies it as a 32-bit (PE32) executable file.
    /// `0x107` identifies it as a ROM image, and `0x20B` identifies it as a 64-bit (PE32+) executable file.
    pub image_type: ImageType,

    /// The linker major version number.
    pub major_linker_version: u8,

    /// The linker minor version number.
    pub minor_linker_version: u8,

    /// The size of the code (`.text`) section, or the sum of all code sections if there are multiple sections.
    pub size_of_code: u32,

    /// The size of the initialized data section, or the sum of all such sections if there are multiple data sections.
    pub size_of_initialized_data: u32,

    /// The size of the uninitialized data section (`BSS`), or the sum of all such sections if there are multiple `BSS` sections.
    pub size_of_uninitialized_data: u32,

    /// The address of the entry point relative to the image base when the executable file is loaded into memory.
    /// For program images, this is the starting address.
    /// For device drivers, this is the address of the initialization function.
    /// An entry point is optional for DLLs.
    pub address_of_entry_point: u32,

    /// The address that is relative to the image base of the beginning-of-code section when it is loaded into memory.
    pub base_of_code: u32,

    /// The address that is relative to the image base of the beginning-of-data section when it is loaded into memory.
    /// PE32 contains this additional field, which is absent in PE32+
    pub base_of_data: Option<u32>,

    /// The preferred address of the first byte of image when loaded into memory; must be a multiple of 64 K.
    /// The default for DLLs is `0x10000000`.
    /// The default for Windows CE EXEs is `0x00010000`.
    /// The default for Windows NT, Windows 2000, Windows XP, Windows 95, Windows 98, and Windows Me is `0x00400000`.
    pub image_base: u64,

    pub section_alignment: u32,

    pub file_alignment: u32,

    pub major_operating_system_version: u16,

    /// The minor version number of the required operating system.
    pub minor_operating_system_version: u16,

    /// The major version number of the image.
    pub major_image_version: u16,

    /// The minor version number of the image.
    pub minor_image_version: u16,

    /// The major version number of the subsystem.
    pub major_subsystem_version: u16,

    /// The minor version number of the subsystem.
    pub minor_subsystem_version: u16,

    /// Reserved, must be zero.
    pub win32_version_value: u32,

    /// The size (in bytes) of the image, including all headers, as the image is loaded in memory.
    /// It must be a multiple of `section_alignment`.
    pub size_of_image: u32,

    /// The combined size of an MS-DOS stub, PE header, and section headers rounded up to a multiple of `file_alignment`.
    pub size_of_headers: u32,

    /// The image file checksum.
    /// The algorithm for computing the checksum is incorporated into IMAGHELP.DLL.
    /// The following are checked for validation at load time: all drivers, any DLL loaded at boot time, and any DLL that is loaded into a critical Windows process.
    pub check_sum: u32,

    /// The subsystem that is required to run this image. For more information, see [`win_subsystem`](crate::header::win_subsystem) module.
    pub subsystem: u16,

    /// See [`dll_characteristics`](crate::header::dll_characteristics) module.
    pub dll_characteristics: u16,

    /// The size of the stack to reserve. Only `size_of_stack_commit` is committed; the rest is made available one page at a time until the reserve size is reached.
    pub size_of_stack_reserve: u64,

    /// The size of the stack to commit.
    pub size_of_stack_commit: u64,

    /// The size of the local heap space to reserve. Only `size_of_heap_commit` is committed; the rest is made available one page at a time until the reserve size is reached.
    pub size_of_heap_reserve: u64,

    /// The size of the local heap space to commit.
    pub size_of_heap_commit: u64,

    /// Reserved, must be zero.
    pub loader_flags: u32,

    /// The number of data-directory entries in the remainder of the optional header. Each describes a location and size.
    pub number_of_rva_and_sizes: u32,

    /// Address/size pairs for special tables that are found in the image file and are used by the operating system (for example, the import table and the export table).
    /// Note that the number of directories is not fixed. Before looking for a specific directory,
    /// check the `number_of_rva_and_sizes` field.
    pub data_directories: Vec<DataDirectory>,
}

#[derive(Debug, Clone)]
pub enum DataDirectoryType {
    ExportTable,
    ImportTable,
    ResourceTable,
    ExceptionTable,
    CertificateTable,
    BaseRelocationTable,
    Debug,
    Architecture,
    GlobalPtr,
    TLSTable,
    LoadConfig,
    BoundImport,
    ImportAdressTable,
    DelayImportDescriptor,
    CLRHeader,
    Reserved,
}

/// Data Directory structure
///
/// Each data directory gives the address and size of a table or string that Windows uses.
/// These data directory entries are all loaded into memory so that the system can use them at run time.
#[derive(Debug)]
pub struct DataDirectory {
    /// The [`RVA`](crate::header::RelativeVirtualAddress) of the table
    pub virtual_address: RelativeVirtualAddress,
    /// Size in bytes
    pub size: u32,
    pub data_directory_type: DataDirectoryType,
}
