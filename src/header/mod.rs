pub mod characteristics;
pub mod dll_characteristics;
pub mod machine_types;
pub mod section_flags;
pub mod win_subsystem;

use chrono::{DateTime, Utc};
use machine_types::*;
use std::{
    fmt::{self, Debug, Display},
    io::{self, Cursor, Read},
    ops::Add,
};

/// The file is an executable image of 32-bit application
pub const IMAGE_NT_OPTIONAL_HDR32_MAGIC: [u8; 2] = [0x0B, 0x01];
/// The file is an executable image of 64-bit application
pub const IMAGE_NT_OPTIONAL_HDR64_MAGIC: [u8; 2] = [0x0B, 0x02];
/// The file is a ROM image.
pub const IMAGE_ROM_OPTIONAL_HDR_MAGIC: [u8; 2] = [0x07, 0x01];
/// Size of COFF File Header
pub const FILE_HEADER_SIZE: u64 = 20;

#[derive(Debug)]
pub enum ImageType {
    /// Represents 32-bit PE image
    Image32 = 0x010B,
    /// Represents 64-bit PE image
    Image64 = 0x020B,
    /// Represents ROM PE Image
    ImageRom = 0x0107,
}

impl Display for ImageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// COFF File Header structure
#[derive(Debug)]
pub struct FileHeader {
    #[doc(hidden)]
    /// file header offset
    offset: u64,
    buffer: Cursor<Vec<u8>>,
    /// Identifies the type of target machine. For more information, see [`machine_types`](machine_types).
    pub machine: StructField<Machine>,
    /// Indicates the size of the section table, which immediately follows the headers.
    pub number_of_sections: StructField<u16>,
    /// The low 32 bits of the number of seconds since 00:00 January 1, 1970 (a C run-time time_t value), which indicates when the file was created.
    pub time_date_stamp: StructField<DateTime<Utc>>,
    /// The file offset of the COFF symbol table, or zero if no COFF symbol table is present.
    /// This value should be zero for an image because COFF debugging information is deprecated.
    pub pointer_to_symbol_table: StructField<u32>,
    /// The number of entries in the symbol table.
    /// This data can be used to locate the string table, which immediately follows the symbol table.
    /// This value should be zero for an image because COFF debugging information is deprecated.
    pub number_of_symbols: StructField<u32>,
    /// The size of the [`OptionalHeader`](crate::header::OptionalHeader), which is required for executable files but not for object files.
    /// This value should be zero for an object file.
    pub size_of_optional_header: StructField<u16>,
    /// The flags that indicate the attributes of the file. For specific flag values, see [`characteristics`](characteristics)
    pub characteristics: StructField<u16>,
}

#[allow(non_snake_case)]
impl FileHeader {
    fn read_machine(&mut self) -> StructField<Machine> {
        todo!()
    }

    fn read_number_of_sections(&mut self) -> StructField<u16> {
        todo!()
    }

    fn read_time_date_stamp(&mut self) -> StructField<DateTime<Utc>> {
        todo!()
    }

    fn read_pointer_to_symbol_table(&self) -> StructField<u32> {
        todo!()
    }

    fn read_number_of_symbols(&self) -> StructField<u32> {
        todo!()
    }

    fn read_size_of_optional_header(&self) -> StructField<u16> {
        todo!()
    }

    fn read_characteristics(&self) -> StructField<u16> {
        todo!()
    }
}

impl ParseStruct for FileHeader {
    fn parse_struct(offset: usize, buffer: Vec<u8>) -> Self {
        todo!()
    }
}

impl Display for FileHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    #[doc(hidden)]
    offset: u64,

    #[doc(hidden)]
    buffer: Cursor<Vec<u8>>,

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

impl OptionalHeader {
    #[doc(hidden)]
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

#[derive(Debug, Clone)]
enum DataDirType {
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
pub struct DataDir {
    offset: u64,
    buffer: Cursor<Vec<u8>>,
    data_dir_type: DataDirType,
}

impl DataDir {
    /// The [`RVA`](crate::header::RelativeVirtualAddress) of the table
    pub fn virtual_address(&self) -> [u8; 4] {
        todo!()
    }

    /// Size in bytes
    pub fn size(&self) -> [u8; 4] {
        todo!()
    }
}

impl Display for DataDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Debug)]
pub struct DataDirectories {
    offset: u64,
    buffer: Cursor<Vec<u8>>,
}

impl DataDirectories {
    /// The export table address and size.
    pub fn export(&self) -> Option<StructField<DataDir>> {
        todo!()
    }

    /// The import table address and size.
    pub fn import(&self) -> Option<StructField<DataDir>> {
        todo!()
    }

    /// The resource table address and size.
    pub fn resource(&self) -> Option<StructField<DataDir>> {
        todo!()
    }

    /// The exception table address and size.
    pub fn exception(&self) -> Option<StructField<DataDir>> {
        todo!()
    }

    // The attribute certificate table address and size.
    /// This entry points to a table of attribute certificates.
    /// These certificates are not loaded into memory as part of the image.
    /// As such, the first field of this entry, which is normally an [`RVA`](RelativeVirtualAddress), is a file pointer instead.
    pub fn certificate(&self) -> Option<StructField<DataDir>> {
        todo!()
    }

    /// The base relocation table address and size.
    pub fn base_relocation(&self) -> Option<StructField<DataDir>> {
        todo!()
    }

    /// The debug data starting address and size.
    pub fn debug(&self) -> Option<StructField<DataDir>> {
        todo!()
    }

    /// Reserved, must be 0
    pub fn architecture(&self) -> Option<StructField<DataDir>> {
        todo!()
    }

    /// The [`RVA`](RelativeVirtualAddress) of the value to be stored in the global pointer register. The size member of this structure must be set to zero.
    pub fn global_ptr(&self) -> Option<StructField<DataDir>> {
        todo!()
    }

    /// The thread local storage (TLS) table address and size.
    pub fn tls_table(&self) -> Option<StructField<DataDir>> {
        todo!()
    }

    /// The load configuration table address and size.
    pub fn load_config_table(&self) -> Option<StructField<DataDir>> {
        todo!()
    }

    /// The bound import table address and size.
    pub fn bound_import(&self) -> Option<StructField<DataDir>> {
        todo!()
    }

    /// The import address table address and size.
    pub fn import_address_table(&self) -> Option<StructField<DataDir>> {
        todo!()
    }

    /// The delay import descriptor address and size.
    pub fn delay_import_descriptor(&self) -> Option<StructField<DataDir>> {
        todo!()
    }

    /// The CLR runtime header address and size.
    pub fn clr_runtime_header(&self) -> Option<StructField<DataDir>> {
        todo!()
    }
}

impl Display for DataDirectories {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// Section header structure
///
/// The basic unit of code or data within a PE or COFF file.
/// For example, all code in an object file can be combined within a single section or (depending on compiler behavior) each function can occupy its own section.
/// With more sections, there is more file overhead, but the linker is able to link in code more selectively.
/// A section is similar to a segment in Intel 8086 architecture.
/// All the raw data in a section must be loaded contiguously.
/// In addition, an image file can contain a number of sections, such as `.tls` or `.reloc` , which have special purposes.
#[derive(Debug)]
pub struct Section {
    offset: u64,
    buffer: Cursor<Vec<u8>>,
}

impl Section {
    /// An 8-byte, null-padded UTF-8 encoded string.
    /// If the string is exactly 8 characters long, there is no terminating null.
    /// For longer names, this field contains a slash (/) that is followed by an ASCII representation of a decimal number that is an offset into the string table.
    /// Executable images do not use a string table and do not support section names longer than 8 characters.
    /// Long names in object files are truncated if they are emitted to an executable file.
    pub fn name(&self) -> StructField<String> {
        todo!()
    }

    /// The total size of the section when loaded into memory.
    /// If this value is greater than `size_of_raw_data`, the section is zero-padded.
    /// This field is valid only for executable images and should be set to zero for object files.
    pub fn virtual_size(&self) -> u32 {
        todo!()
    }

    /// For executable images, the address of the first byte of the section relative to the image base when the section is loaded into memory.
    /// For object files, this field is the address of the first byte before relocation is applied; for simplicity, compilers should set this to zero.
    /// Otherwise, it is an arbitrary value that is subtracted from offsets during relocation.
    pub fn virtual_address(&self, image_base: VirtualAddress) -> RelativeVirtualAddress {
        todo!()
    }

    /// The size of the section (for object files) or the size of the initialized data on disk (for image files).
    /// For executable images, this must be a multiple of [`file_alignment`](OptionalHeader#structfield.file_alignment) from the [`OptionalHeader`].
    /// If this is less than `virtual_size`, the remainder of the section is zero-filled.
    /// Because the `size_of_raw_data` field is rounded but the `virtual_size` field is not, it is possible for `size_of_raw_data` to be greater than `virtual_size` as well.
    /// When a section contains only uninitialized data, this field should be zero.
    pub fn size_of_raw_data(&self) -> u32 {
        todo!()
    }

    /// The file pointer to the first page of the section within the COFF file.
    /// For executable images, this must be a multiple of [`file_alignment`](OptionalHeader#structfield.file_alignment) from the [`OptionalHeader`].
    /// For object files, the value should be aligned on a 4-byte boundary for best performance.
    /// When a section contains only uninitialized data, this field should be zero.
    pub fn pointer_to_raw_data(&self) -> u64 {
        todo!()
    }

    /// The file pointer to the beginning of relocation entries for the section.
    /// This is set to zero for executable images or if there are no relocations.
    pub fn pointer_to_relocations(&self) -> u64 {
        todo!()
    }

    /// The file pointer to the beginning of line-number entries for the section.
    /// This is set to zero if there are no COFF line numbers.
    /// This value should be zero for an image because COFF debugging information is deprecated.
    pub fn pointer_to_linenumbers(&self) -> u64 {
        todo!()
    }

    /// The number of relocation entries for the section.
    /// This is set to zero for executable images.
    pub fn number_of_relocations(&self) -> u16 {
        todo!()
    }

    /// The number of line-number entries for the section.
    /// This value should be zero for an image because COFF debugging information is deprecated.
    pub fn number_of_linenumbers(&self) -> u16 {
        todo!()
    }

    /// The [flags](section_flags) that describe the characteristics of the section.
    pub fn characteristics(&self) -> [u8; 4] {
        todo!()
    }
}

/// Relavive virtual address (RVA)
///
/// In an image file, this is the address of an item after it is loaded into memory, with the base address of the image file subtracted from it.
/// The RVA of an item almost always differs from its position within the file on disk (file pointer).
/// In an object file, an RVA is less meaningful because memory locations are not assigned.
/// In this case, an RVA would be an address within a section (described later in this table), to which a relocation is later applied during linking.
/// For simplicity, a compiler should just set the first RVA in each section to zero.
#[derive(Debug)]
pub struct RelativeVirtualAddress {
    addr: VirtualAddress,
    image_base: VirtualAddress,
}

impl Display for RelativeVirtualAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
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

impl Add for VirtualAddress {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let addr = self.addr + rhs.addr;
        Self { addr }
    }
}

impl From<RelativeVirtualAddress> for VirtualAddress {
    fn from(value: RelativeVirtualAddress) -> Self {
        let addr: u64 = (value.addr + value.image_base).into();
        Self { addr }
    }
}

impl From<VirtualAddress> for u64 {
    fn from(value: VirtualAddress) -> Self {
        value.addr
    }
}

impl From<u64> for VirtualAddress {
    fn from(value: u64) -> Self {
        VirtualAddress { addr: value }
    }
}

impl From<u32> for VirtualAddress {
    fn from(value: u32) -> Self {
        Self { addr: value.into() }
    }
}

impl From<[u8; 4]> for VirtualAddress {
    fn from(value: [u8; 4]) -> Self {
        Self {
            addr: u32::from_le_bytes(value).into(),
        }
    }
}

impl From<[u8; 8]> for VirtualAddress {
    fn from(value: [u8; 8]) -> Self {
        Self {
            addr: u64::from_le_bytes(value),
        }
    }
}

#[derive(Debug)]
pub struct StructField<T: fmt::Debug + Display> {
    offset: u64,
    raw_bytes: Vec<u8>,
    data: T,
    meaning: String,
}

impl<T: Debug + Display> Display for StructField<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}\t{:#?}\t{:#?}\t{}",
            self.offset, self.raw_bytes, self.data, self.meaning
        ))
    }
}

trait ReadU16LE {
    fn read_u16_le(&mut self) -> u16;
}

trait ParseStruct {
    fn parse_struct(offset: usize, buffer: Vec<u8>) -> Self
    where
        Self: Sized,
    {
        todo!()
    }
}
