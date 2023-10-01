use crate::header::{ImageType, RelativeVirtualAddress};
use crate::struct_parse::StructField;
use std::fmt::{self, Display};

pub struct OptionalHeaderBuffer {
    offset: u64,
    buffer: Vec<u8>,
}

impl OptionalHeaderBuffer {
    pub fn new(offset: u64, buffer: Vec<u8>) -> Self {
        Self { offset, buffer }
    }

    pub fn read_optional_header(&self) -> OptionalHeader {
        todo!()
    }

    fn read_image_type(&mut self) -> StructField<ImageType> {
        todo!()
    }

    fn read_major_linker_version(&mut self) -> StructField<u8> {
        todo!()
    }

    fn read_minor_linker_version(&mut self) -> StructField<u8> {
        todo!()
    }

    fn read_size_of_code(&mut self) -> StructField<u32> {
        todo!()
    }

    fn read_size_of_initialized_data(&mut self) -> StructField<u32> {
        todo!()
    }

    fn read_size_of_uninitialized_data(&mut self) -> StructField<u32> {
        todo!()
    }

    fn read_address_of_entry_point(&mut self) -> StructField<RelativeVirtualAddress> {
        todo!()
    }

    fn read_base_of_code(&mut self) -> StructField<RelativeVirtualAddress> {
        todo!()
    }

    fn read_base_of_data(&mut self) -> Option<StructField<RelativeVirtualAddress>> {
        todo!()
    }

    fn image_base(&mut self) -> StructField<u64> {
        todo!()
    }

    fn read_section_alignment(&mut self) -> StructField<u32> {
        todo!()
    }

    fn read_file_alignment(&mut self) -> StructField<u32> {
        todo!()
    }

    fn read_major_operating_system_version(&mut self) -> StructField<u16> {
        todo!()
    }

    fn read_minor_operating_system_version(&mut self) -> StructField<u16> {
        todo!()
    }

    fn read_major_image_version(&mut self) -> StructField<u16> {
        todo!()
    }

    fn read_minor_image_version(&mut self) -> StructField<u16> {
        todo!()
    }

    fn read_major_subsystem_version(&mut self) -> StructField<u16> {
        todo!()
    }

    fn read_minor_subsystem_version(&mut self) -> StructField<u16> {
        todo!()
    }

    fn read_win32_version_value(&mut self) -> StructField<u32> {
        todo!()
    }

    fn read_size_of_image(&mut self) -> StructField<u32> {
        todo!()
    }

    fn read_size_of_headers(&mut self) -> StructField<u32> {
        todo!()
    }

    fn read_check_sum(&mut self) -> StructField<u32> {
        todo!()
    }

    fn read_subsystem(&mut self) -> StructField<u16> {
        todo!()
    }

    fn read_dll_characteristics(&mut self) -> StructField<u16> {
        todo!()
    }

    fn read_size_of_stack_reserve(&mut self) -> StructField<u64> {
        todo!()
    }

    fn read_size_of_stack_commit(&mut self) -> StructField<u64> {
        todo!()
    }

    fn read_size_of_heap_reserve(&mut self) -> StructField<u64> {
        todo!()
    }

    fn read_size_of_heap_commit(&self) -> StructField<u64> {
        todo!()
    }

    fn read_loader_flags(&self) -> StructField<u32> {
        todo!()
    }

    fn read_number_of_rva_and_sizes(&self) -> StructField<u32> {
        todo!()
    }

    fn read_data_directories(&self) -> StructField<DataDirectories> {
        todo!()
    }
}

/// Optional Header structure
///
/// Every image file has an optional header that provides information to the loader.
/// This header is optional in the sense that some files (specifically, object files) do not have it.
/// For image files, this header is required.
/// An object file can have an optional header, but generally this header has no function in an object file except to increase its size.
/// Note that the size of the optional header is not fixed.
/// The [`size_of_optional_header`](crate::header::FileHeader#structfield.size_of_optional_header) field in the COFF header must be used
/// to validate that a probe into the file for a particular data directory does not go beyond [`size_of_optional_header`](crate::header::FileHeader#structfield.size_of_optional_header).
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
    pub image_type: StructField<ImageType>,

    /// The linker major version number.
    pub major_linker_version: StructField<u8>,

    /// The linker minor version number.
    pub minor_linker_version: StructField<u8>,

    /// The size of the code (`.text`) section, or the sum of all code sections if there are multiple sections.
    pub size_of_code: StructField<u32>,

    /// The size of the initialized data section, or the sum of all such sections if there are multiple data sections.
    pub size_of_initialized_data: StructField<u32>,

    /// The size of the uninitialized data section (`BSS`), or the sum of all such sections if there are multiple `BSS` sections.
    pub size_of_uninitialized_data: StructField<u32>,

    /// The address of the entry point relative to the image base when the executable file is loaded into memory.
    /// For program images, this is the starting address.
    /// For device drivers, this is the address of the initialization function.
    /// An entry point is optional for DLLs.
    pub address_of_entry_point: StructField<RelativeVirtualAddress>,

    /// The address that is relative to the image base of the beginning-of-code section when it is loaded into memory.
    pub base_of_code: StructField<RelativeVirtualAddress>,

    /// The address that is relative to the image base of the beginning-of-data section when it is loaded into memory.
    /// PE32 contains this additional field, which is absent in PE32+
    pub base_of_data: Option<StructField<RelativeVirtualAddress>>,

    /// The preferred address of the first byte of image when loaded into memory; must be a multiple of 64 K.
    /// The default for DLLs is `0x10000000`.
    /// The default for Windows CE EXEs is `0x00010000`.
    /// The default for Windows NT, Windows 2000, Windows XP, Windows 95, Windows 98, and Windows Me is `0x00400000`.
    pub image_base: StructField<u64>,

    pub section_alignment: StructField<u32>,

    pub file_alignment: StructField<u32>,

    pub major_operating_system_version: StructField<u16>,

    /// The minor version number of the required operating system.
    pub minor_operating_system_version: StructField<u16>,

    /// The major version number of the image.
    pub major_image_version: StructField<u16>,

    /// The minor version number of the image.
    pub minor_image_version: StructField<u16>,

    /// The major version number of the subsystem.
    pub major_subsystem_version: StructField<u16>,

    /// The minor version number of the subsystem.
    pub minor_subsystem_version: StructField<u16>,

    /// Reserved, must be zero.
    pub win32_version_value: StructField<u32>,

    /// The size (in bytes) of the image, including all headers, as the image is loaded in memory.
    /// It must be a multiple of `section_alignment`.
    pub size_of_image: StructField<u32>,

    /// The combined size of an MS-DOS stub, PE header, and section headers rounded up to a multiple of `file_alignment`.
    pub size_of_headers: StructField<u32>,

    /// The image file checksum.
    /// The algorithm for computing the checksum is incorporated into IMAGHELP.DLL.
    /// The following are checked for validation at load time: all drivers, any DLL loaded at boot time, and any DLL that is loaded into a critical Windows process.
    pub check_sum: StructField<u32>,

    /// The subsystem that is required to run this image. For more information, see [`win_subsystem`](win_subsystem) module.
    pub subsystem: StructField<u16>,

    /// See [`dll_characteristics`](dll_characteristics) module.
    pub dll_characteristics: StructField<u16>,

    /// The size of the stack to reserve. Only `size_of_stack_commit` is committed; the rest is made available one page at a time until the reserve size is reached.
    pub size_of_stack_reserve: StructField<u64>,

    /// The size of the stack to commit.
    pub size_of_stack_commit: StructField<u64>,

    /// The size of the local heap space to reserve. Only `size_of_heap_commit` is committed; the rest is made available one page at a time until the reserve size is reached.
    pub size_of_heap_reserve: StructField<u64>,

    /// The size of the local heap space to commit.
    pub size_of_heap_commit: StructField<u64>,

    /// Reserved, must be zero.
    pub loader_flags: StructField<u32>,

    /// The number of data-directory entries in the remainder of the optional header. Each describes a location and size.
    pub number_of_rva_and_sizes: StructField<u32>,

    /// Address/size pairs for special tables that are found in the image file and are used by the operating system (for example, the import table and the export table).
    /// Note that the number of directories is not fixed. Before looking for a specific directory,
    /// check the `number_of_rva_and_sizes` field.
    pub data_directories: StructField<DataDirectories>,
}

#[derive(Debug, Clone)]
pub enum DataDirectoryType {
    Export,
    Import,
    Resource,
    Exception,
    Certificate,
    BaseRelocation,
    Debug,
    Architecture,
}

/// Data Directory structure
///
/// Each data directory gives the address and size of a table or string that Windows uses.
/// These data directory entries are all loaded into memory so that the system can use them at run time.
#[derive(Debug, Clone)]
pub struct DataDirectoryBuffer {
    offset: u64,
    buffer: Vec<u8>,
}

impl DataDirectoryBuffer {
    /// The export table address and size.
    pub fn export(&self) -> Option<StructField<DataDirectory>> {
        todo!()
    }

    /// The import table address and size.
    pub fn import(&self) -> Option<StructField<DataDirectory>> {
        todo!()
    }

    /// The resource table address and size.
    pub fn resource(&self) -> Option<StructField<DataDirectory>> {
        todo!()
    }

    /// The exception table address and size.
    pub fn exception(&self) -> Option<StructField<DataDirectory>> {
        todo!()
    }

    // The attribute certificate table address and size.
    /// This entry points to a table of attribute certificates.
    /// These certificates are not loaded into memory as part of the image.
    /// As such, the first field of this entry, which is normally an [`RVA`](RelativeVirtualAddress), is a file pointer instead.
    pub fn certificate(&self) -> Option<StructField<DataDirectory>> {
        todo!()
    }

    /// The base relocation table address and size.
    pub fn base_relocation(&self) -> Option<StructField<DataDirectory>> {
        todo!()
    }

    /// The debug data starting address and size.
    pub fn debug(&self) -> Option<StructField<DataDirectory>> {
        todo!()
    }

    /// Reserved, must be 0
    pub fn architecture(&self) -> Option<StructField<DataDirectory>> {
        todo!()
    }

    /// The [`RVA`](RelativeVirtualAddress) of the value to be stored in the global pointer register. The size member of this structure must be set to zero.
    pub fn global_ptr(&self) -> Option<StructField<DataDirectory>> {
        todo!()
    }

    /// The thread local storage (TLS) table address and size.
    pub fn tls_table(&self) -> Option<StructField<DataDirectory>> {
        todo!()
    }

    /// The load configuration table address and size.
    pub fn load_config_table(&self) -> Option<StructField<DataDirectory>> {
        todo!()
    }

    /// The bound import table address and size.
    pub fn bound_import(&self) -> Option<StructField<DataDirectory>> {
        todo!()
    }

    /// The import address table address and size.
    pub fn import_address_table(&self) -> Option<StructField<DataDirectory>> {
        todo!()
    }

    /// The delay import descriptor address and size.
    pub fn delay_import_descriptor(&self) -> Option<StructField<DataDirectory>> {
        todo!()
    }

    /// The CLR runtime header address and size.
    pub fn clr_runtime_header(&self) -> Option<StructField<DataDirectory>> {
        todo!()
    }
}

#[derive(Debug)]
pub struct DataDirectory {
    virtual_address: RelativeVirtualAddress,
    size: u32,
    data_directory_type: DataDirectoryType,
}

impl DataDirectory {
    /// The [`RVA`](crate::header::RelativeVirtualAddress) of the table
    pub fn virtual_address(&self) -> [u8; 4] {
        todo!()
    }

    /// Size in bytes
    pub fn size(&self) -> [u8; 4] {
        todo!()
    }
}

impl Display for DataDirectory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Debug)]
pub struct DataDirectories {
    offset: u64,
    buffer: Vec<u8>,
}

impl DataDirectories {}

impl Display for DataDirectories {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
