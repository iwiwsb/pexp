pub mod characteristics;
pub mod machine_types;
pub mod section_flags;
pub mod win_subsystem;

use chrono::{DateTime, TimeZone, Utc};
use machine_types::Machine;
use std::{
    fmt::Display,
    io::{self, Read, Seek, SeekFrom},
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

pub enum ImageType {
    /// Represents 32-bit PE image
    Image32 = 0x010B,
    /// Represents 64-bit PE image
    Image64 = 0x020B,
    /// Represents ROM PE Image
    ImageRom = 0x0107,
}

/// COFF File Header structure
#[derive(Debug)]
pub struct FileHeader {
    /// Identifies the type of target machine. For more information, see [`machine_types`](crate::machine_types).
    machine: [u8; 2],
    /// The number of sections. This indicates the size of the section table, which immediately follows the headers.
    number_of_sections: [u8; 2],
    /// The low 32 bits of the number of seconds since 00:00 January 1, 1970 (a C run-time time_t value), which indicates when the file was created.
    time_date_stamp: [u8; 4],
    /// The file offset of the COFF symbol table, or zero if no COFF symbol table is present.
    /// This value should be zero for an image because COFF debugging information is deprecated.
    pointer_to_symbol_table: [u8; 4],
    /// The number of entries in the symbol table.
    /// This data can be used to locate the string table, which immediately follows the symbol table.
    /// This value should be zero for an image because COFF debugging information is deprecated.
    number_of_symbols: [u8; 4],
    /// The size of the [`OptionalHeader`](crate::header::OptionalHeader), which is required for executable files but not for object files.
    /// This value should be zero for an object file.
    size_of_optional_header: [u8; 2],
    /// The flags that indicate the attributes of the file. For specific flag values, see [`characteristics`](crate::characteristics)
    characteristics: [u8; 2],
}

impl FileHeader {
    fn machine(&self) -> Machine {
        Machine::try_from(self.machine).unwrap()
    }

    fn number_of_sections(&self) -> u16 {
        u16::from_le_bytes(self.number_of_sections)
    }

    fn time_date_stamp(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(u32::from_le_bytes(self.time_date_stamp) as i64, 0)
            .unwrap()
    }

    fn pointer_to_symbol_table(&self) -> u32 {
        u32::from_le_bytes(self.pointer_to_symbol_table)
    }

    fn number_of_symbols(&self) -> u32 {
        u32::from_le_bytes(self.number_of_symbols)
    }

    fn size_of_optional_header(&self) -> u16 {
        u16::from_le_bytes(self.size_of_optional_header)
    }

    fn characteristics(&self) -> u16 {
        u16::from_le_bytes(self.characteristics)
    }
}

impl Display for FileHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Machine: {0}\n", self.machine()))
            .unwrap();
        f.write_fmt(format_args!(
            "Number of sections: {0}\n",
            self.number_of_sections()
        ))
        .unwrap();
        f.write_fmt(format_args!("Timestamp: {0}\n", self.time_date_stamp()))
            .unwrap();
        f.write_fmt(format_args!(
            "Pointer to symbol table: 0x{0:08X}\n",
            self.pointer_to_symbol_table()
        ))
        .unwrap();
        f.write_fmt(format_args!(
            "Number of symbols: {0}\n",
            self.number_of_symbols()
        ))
        .unwrap();
        f.write_fmt(format_args!(
            "Size of optional header: {0} bytes\n",
            self.size_of_optional_header()
        ))
        .unwrap();
        f.write_fmt(format_args!(
            "Characteristics: 0x{0:04X}\n",
            self.characteristics()
        ))
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
#[allow(unused)]
pub struct OptionalHeader {
    /// Identifies the state of the image file.
    /// The most common number is `0x10B`, which identifies it as a 32-bit (PE32) executable file.
    /// `0x107` identifies it as a ROM image, and `0x20B` identifies it as a 64-bit (PE32+) executable file.
    magic: [u8; 2],
    /// The linker major version number.
    major_linker_version: [u8; 1],
    /// The linker minor version number.
    minor_linker_version: [u8; 1],
    /// The size of the code (`.text`) section, or the sum of all code sections if there are multiple sections.
    size_of_code: [u8; 4],
    /// The size of the initialized data section, or the sum of all such sections if there are multiple data sections.
    size_of_initialized_data: [u8; 4],
    /// The size of the uninitialized data section (`BSS`), or the sum of all such sections if there are multiple `BSS` sections.
    size_of_uninitialized_data: [u8; 4],
    /// The address of the entry point relative to the image base when the executable file is loaded into memory.
    /// For program images, this is the starting address.
    /// For device drivers, this is the address of the initialization function.
    /// An entry point is optional for DLLs.
    /// When no entry point is present, this field must be zero.
    address_of_entry_point: [u8; 4],
    /// The address that is relative to the image base of the beginning-of-code section when it is loaded into memory.
    base_of_code: [u8; 4],
    /// The address that is relative to the image base of the beginning-of-data section when it is loaded into memory.
    /// PE32 contains this additional field, which is absent in PE32+
    base_of_data: Option<[u8; 4]>,
    /// The preferred address of the first byte of image when loaded into memory; must be a multiple of 64 K.
    /// The default for DLLs is `0x10000000`.
    /// The default for Windows CE EXEs is `0x00010000`.
    /// The default for Windows NT, Windows 2000, Windows XP, Windows 95, Windows 98, and Windows Me is `0x00400000`.
    image_base: [u8; 8],
    /// The alignment (in bytes) of sections when they are loaded into memory.
    /// It must be greater than or equal to `file_alignment`.
    /// The default is the page size for the architecture.
    section_alignment: [u8; 4],
    /// The alignment factor (in bytes) that is used to align the raw data of sections in the image file.
    /// The value should be a power of 2 between 512 and 64 K, inclusive.
    /// The default is 512. If the `section_alignment` is less than the architecture's page size, then FileAlignment must match `section_alignment`.
    file_alignment: [u8; 4],
    /// The major version number of the required operating system.
    major_operating_system_version: [u8; 2],
    /// The minor version number of the required operating system.
    minor_operating_system_version: [u8; 2],
    /// The major version number of the image.
    major_image_version: [u8; 2],
    /// The minor version number of the image.
    minor_image_version: [u8; 2],
    /// The major version number of the subsystem.
    major_subsystem_version: [u8; 2],
    /// The minor version number of the subsystem.
    minor_subsystem_version: [u8; 2],
    /// Reserved, must be zero.
    win32_version_value: [u8; 4],
    /// The size (in bytes) of the image, including all headers, as the image is loaded in memory.
    /// It must be a multiple of `section_alignment`.
    size_of_image: [u8; 4],
    /// The combined size of an MS-DOS stub, PE header, and section headers rounded up to a multiple of `file_alignment`.
    size_of_headers: [u8; 4],
    /// The image file checksum.
    /// The algorithm for computing the checksum is incorporated into IMAGHELP.DLL.
    /// The following are checked for validation at load time: all drivers, any DLL loaded at boot time, and any DLL that is loaded into a critical Windows process.
    check_sum: [u8; 4],
    /// The subsystem that is required to run this image. For more information, see [`win_subsystem`](crate::win_subsystem) module.
    subsystem: [u8; 2],
    /// See [`dll_characteristics`](crate::dll_characteristics) module.
    dll_characteristics: [u8; 2],
    /// The size of the stack to reserve. Only `size_of_stack_commit` is committed; the rest is made available one page at a time until the reserve size is reached.
    size_of_stack_reserve: [u8; 8],
    /// The size of the stack to commit.
    size_of_stack_commit: [u8; 8],
    /// The size of the local heap space to reserve. Only `size_of_heap_commit` is committed; the rest is made available one page at a time until the reserve size is reached.
    size_of_heap_reserve: [u8; 8],
    /// The size of the local heap space to commit.
    size_of_heap_commit: [u8; 8],
    /// Reserved, must be zero.
    loader_flags: [u8; 4],
    /// The number of data-directory entries in the remainder of the optional header. Each describes a location and size.
    number_of_rva_and_sizes: [u8; 4],
    /// Address/size pairs for special tables that are found in the image file and are used by the operating system (for example, the import table and the export table).
    /// Note that the number of directories is not fixed. Before looking for a specific directory,
    /// check the `number_of_rva_and_sizes` field.
    data_directories: Vec<DataDir>,
}

impl OptionalHeader {
    pub fn image_type(&self) -> ImageType {
        match self.magic {
            IMAGE_NT_OPTIONAL_HDR32_MAGIC => ImageType::Image32,
            IMAGE_NT_OPTIONAL_HDR64_MAGIC => ImageType::Image64,
            IMAGE_ROM_OPTIONAL_HDR_MAGIC => ImageType::ImageRom,
            _ => panic!(),
        }
    }

    pub fn size_of_uninitialized_data(&self) -> u32 {
        todo!()
    }

    pub fn number_of_rva_and_sizes(&self) -> u32 {
        u32::from_le_bytes(self.number_of_rva_and_sizes)
    }

    pub fn major_linker_version(&self) -> u8 {
        self.major_linker_version[0]
    }

    pub fn minor_linker_version(&self) -> u8 {
        self.minor_linker_version[0]
    }

    pub fn size_of_code(&self) -> u32 {
        let bytes = self.size_of_code;
        u32::from_le_bytes(bytes)
    }

    pub fn size_of_initialized_data(&self) -> u32 {
        let bytes = self.size_of_initialized_data;
        u32::from_le_bytes(bytes)
    }

    pub fn size_of_uninitialized_data_mut(&self) -> u32 {
        let bytes = self.size_of_uninitialized_data;
        u32::from_le_bytes(bytes)
    }

    pub fn address_of_entry_point(&self) -> RelativeVirtualAddress {
        let addr: VirtualAddress = self.address_of_entry_point.into();
        let image_base: VirtualAddress = self.image_base.into();
        RelativeVirtualAddress { addr, image_base }
    }

    pub fn base_of_code(&self) -> RelativeVirtualAddress {
        let addr: VirtualAddress = self.base_of_code.into();
        let image_base: VirtualAddress = self.image_base.into();
        RelativeVirtualAddress { addr, image_base }
    }

    pub fn base_of_data(&self) -> Option<RelativeVirtualAddress> {
        match self.base_of_data {
            Some(base) => {
                let addr: VirtualAddress = base.into();
                let image_base = self.image_base.into();
                Some(RelativeVirtualAddress { addr, image_base })
            }
            None => None,
        }
    }

    pub fn data_directories(&self) -> DataDirectories {
        DataDirectories::from(self.data_directories.clone())
    }
}

/// Data Directory structure
///
/// Each data directory gives the address and size of a table or string that Windows uses.
/// These data directory entries are all loaded into memory so that the system can use them at run time.
#[derive(Debug, Clone)]
pub struct DataDir {
    /// The [`RVA`](crate::header::RelativeVirtualAddress) of the table
    virtual_address: [u8; 4],
    /// Size in bytes
    size: [u8; 4],
}

impl DataDir {
    pub fn virtual_address(&self) -> [u8; 4] {
        self.virtual_address
    }

    pub fn size(&self) -> [u8; 4] {
        self.size
    }
}

#[derive(Debug)]
#[allow(unused)]
pub struct DataDirectories {
    /// The export table address and size.
    export_table: Option<DataDir>,
    /// The import table address and size.
    import_table: Option<DataDir>,
    /// The resource table address and size.
    resource_table: Option<DataDir>,
    /// The exception table address and size.
    exception_table: Option<DataDir>,
    /// The attribute certificate table address and size.
    /// This entry points to a table of attribute certificates.
    /// These certificates are not loaded into memory as part of the image.
    /// As such, the first field of this entry, which is normally an [`RVA`](RelativeVirtualAddress), is a file pointer instead.
    certificate_table: Option<DataDir>,
    /// The base relocation table address and size.
    base_relocation_table: Option<DataDir>,
    /// The debug data starting address and size.
    debug: Option<DataDir>,
    /// Reserved, must be 0
    architecture: Option<DataDir>,
    /// The [`RVA`](RelativeVirtualAddress) of the value to be stored in the global pointer register. The size member of this structure must be set to zero.
    global_ptr: Option<DataDir>,
    /// The thread local storage (TLS) table address and size.
    tls_table: Option<DataDir>,
    /// The load configuration table address and size.
    load_config_table: Option<DataDir>,
    /// The bound import table address and size.
    bound_import: Option<DataDir>,
    /// The import address table address and size.
    import_address_table: Option<DataDir>,
    /// The delay import descriptor address and size.
    delay_import_descriptor: Option<DataDir>,
    /// The CLR runtime header address and size.
    clr_runtime_header: Option<DataDir>,
}

impl DataDirectories {
    pub fn export_table(&self) -> Option<&DataDir> {
        self.export_table.as_ref()
    }

    pub fn import_table(&self) -> Option<&DataDir> {
        self.import_table.as_ref()
    }

    pub fn resource_table(&self) -> Option<&DataDir> {
        self.resource_table.as_ref()
    }

    pub fn exception_table(&self) -> Option<&DataDir> {
        self.exception_table.as_ref()
    }

    pub fn certificate_table(&self) -> Option<&DataDir> {
        self.certificate_table.as_ref()
    }

    pub fn base_relocation_table(&self) -> Option<&DataDir> {
        self.base_relocation_table.as_ref()
    }

    pub fn architecture(&self) -> Option<&DataDir> {
        self.architecture.as_ref()
    }

    pub fn global_ptr(&self) -> Option<&DataDir> {
        self.global_ptr.as_ref()
    }

    pub fn tls_table(&self) -> Option<&DataDir> {
        self.tls_table.as_ref()
    }

    pub fn load_config_table(&self) -> Option<&DataDir> {
        self.load_config_table.as_ref()
    }

    pub fn bound_import(&self) -> Option<&DataDir> {
        self.bound_import.as_ref()
    }

    pub fn import_address_table(&self) -> Option<&DataDir> {
        self.import_address_table.as_ref()
    }

    pub fn delay_import_descriptor(&self) -> Option<&DataDir> {
        self.delay_import_descriptor.as_ref()
    }

    pub fn clr_runtime_header(&self) -> Option<&DataDir> {
        self.clr_runtime_header.as_ref()
    }
}

impl From<Vec<DataDir>> for DataDirectories {
    fn from(value: Vec<DataDir>) -> Self {
        let export_table = value.get(0).cloned();
        let import_table = value.get(1).cloned();
        let resource_table = value.get(2).cloned();
        let exception_table = value.get(3).cloned();
        let certificate_table = value.get(4).cloned();
        let base_relocation_table = value.get(5).cloned();
        let debug = value.get(6).cloned();
        let architecture = value.get(7).cloned();
        let global_ptr = value.get(8).cloned();
        let tls_table = value.get(9).cloned();
        let load_config_table = value.get(10).cloned();
        let bound_import = value.get(11).cloned();
        let import_address_table = value.get(12).cloned();
        let delay_import_descriptor = value.get(13).cloned();
        let clr_runtime_header = value.get(14).cloned();

        Self {
            export_table,
            import_table,
            resource_table,
            exception_table,
            certificate_table,
            base_relocation_table,
            debug,
            architecture,
            global_ptr,
            tls_table,
            load_config_table,
            bound_import,
            import_address_table,
            delay_import_descriptor,
            clr_runtime_header,
        }
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
    /// An 8-byte, null-padded UTF-8 encoded string.
    /// If the string is exactly 8 characters long, there is no terminating null.
    /// For longer names, this field contains a slash (/) that is followed by an ASCII representation of a decimal number that is an offset into the string table.
    /// Executable images do not use a string table and do not support section names longer than 8 characters.
    /// Long names in object files are truncated if they are emitted to an executable file.
    name: [u8; 8],
    /// The total size of the section when loaded into memory.
    /// If this value is greater than `size_of_raw_data`, the section is zero-padded.
    /// This field is valid only for executable images and should be set to zero for object files.
    virtual_size: [u8; 4],
    /// For executable images, the address of the first byte of the section relative to the image base when the section is loaded into memory.
    /// For object files, this field is the address of the first byte before relocation is applied; for simplicity, compilers should set this to zero.
    /// Otherwise, it is an arbitrary value that is subtracted from offsets during relocation.
    virtual_address: [u8; 4],
    /// The size of the section (for object files) or the size of the initialized data on disk (for image files).
    /// For executable images, this must be a multiple of [`file_alignment`](OptionalHeader#structfield.file_alignment) from the [`OptionalHeader`].
    /// If this is less than `virtual_size`, the remainder of the section is zero-filled.
    /// Because the `size_of_raw_data` field is rounded but the `virtual_size` field is not, it is possible for `size_of_raw_data` to be greater than `virtual_size` as well.
    /// When a section contains only uninitialized data, this field should be zero.
    size_of_raw_data: [u8; 4],
    /// The file pointer to the first page of the section within the COFF file.
    /// For executable images, this must be a multiple of [`file_alignment`](OptionalHeader#structfield.file_alignment) from the [`OptionalHeader`].
    /// For object files, the value should be aligned on a 4-byte boundary for best performance.
    /// When a section contains only uninitialized data, this field should be zero.
    pointer_to_raw_data: [u8; 4],
    /// The file pointer to the beginning of relocation entries for the section.
    /// This is set to zero for executable images or if there are no relocations.
    pointer_to_relocations: [u8; 4],
    /// The file pointer to the beginning of line-number entries for the section.
    /// This is set to zero if there are no COFF line numbers.
    /// This value should be zero for an image because COFF debugging information is deprecated.
    pointer_to_linenumbers: [u8; 4],
    /// The number of relocation entries for the section.
    /// This is set to zero for executable images.
    number_of_relocations: [u8; 2],
    /// The number of line-number entries for the section.
    /// This value should be zero for an image because COFF debugging information is deprecated.
    number_of_linenumbers: [u8; 2],
    /// The [flags](crate::section_flags) that describe the characteristics of the section.
    characteristics: [u8; 4],
}

impl Section {
    pub fn name(&self) -> String {
        self.name
            .iter()
            .take_while(|&c| *c != 0)
            .map(|c| *c as char)
            .collect()
    }

    pub fn virtual_size(&self) -> u32 {
        u32::from_le_bytes(self.virtual_size)
    }

    pub fn virtual_address(&self, image_base: VirtualAddress) -> RelativeVirtualAddress {
        let addr: VirtualAddress = u32::from_le_bytes(self.virtual_address).into();
        RelativeVirtualAddress { addr, image_base }
    }

    pub fn size_of_raw_data(&self) -> u32 {
        u32::from_le_bytes(self.size_of_raw_data)
    }

    pub fn pointer_to_raw_data(&self) -> u64 {
        let pos: u64 = u32::from_le_bytes(self.pointer_to_raw_data).into();
        pos
    }

    pub fn pointer_to_relocations(&self) -> u64 {
        u32::from_le_bytes(self.pointer_to_relocations).into()
    }

    pub fn pointer_to_linenumbers(&self) -> u64 {
        u32::from_le_bytes(self.pointer_to_linenumbers).into()
    }

    pub fn number_of_relocations(&self) -> u16 {
        u16::from_le_bytes(self.number_of_relocations)
    }

    pub fn number_of_linenumbers(&self) -> u16 {
        u16::from_le_bytes(self.number_of_linenumbers)
    }

    pub fn characteristics(&self) -> [u8; 4] {
        self.characteristics
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

pub fn read_file_header<R: Read + Seek>(
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

pub fn read_optional_header<R: Read + Seek>(
    reader: &mut R,
    opt_header_offset: u64,
) -> io::Result<OptionalHeader> {
    let mut magic = [0u8; 2];
    let mut major_linker_version = [0u8; 1];
    let mut minor_linker_version = [0u8; 1];
    let mut size_of_code = [0u8; 4];
    let mut size_of_initialized_data = [0u8; 4];
    let mut size_of_uninitialized_data = [0u8; 4];
    let mut address_of_entry_point = [0u8; 4];
    let mut base_of_code = [0u8; 4];
    let mut base_of_data: Option<[u8; 4]> = Some([0u8; 4]);
    let mut image_base = [0u8; 8];
    let mut section_alignment = [0u8; 4];
    let mut file_alignment = [0u8; 4];
    let mut major_operating_system_version = [0u8; 2];
    let mut minor_operating_system_version = [0u8; 2];
    let mut major_image_version = [0u8; 2];
    let mut minor_image_version = [0u8; 2];
    let mut major_subsystem_version = [0u8; 2];
    let mut minor_subsystem_version = [0u8; 2];
    let mut win32_version_value = [0u8; 4];
    let mut size_of_image = [0u8; 4];
    let mut size_of_headers = [0u8; 4];
    let mut check_sum = [0u8; 4];
    let mut subsystem = [0u8; 2];
    let mut dll_characteristics = [0u8; 2];
    let mut size_of_stack_reserve = [0u8; 8];
    let mut size_of_stack_commit = [0u8; 8];
    let mut size_of_heap_reserve = [0u8; 8];
    let mut size_of_heap_commit = [0u8; 8];
    let mut loader_flags = [0u8; 4];
    let mut number_of_rva_and_sizes = [0u8; 4];
    let mut data_directories: Vec<DataDir> = Vec::new();

    reader.seek(SeekFrom::Start(opt_header_offset))?;
    reader.read_exact(&mut magic)?;
    reader.read_exact(&mut major_linker_version)?;
    reader.read_exact(&mut minor_linker_version)?;
    reader.read_exact(&mut size_of_code)?;
    reader.read_exact(&mut size_of_initialized_data)?;
    reader.read_exact(&mut size_of_uninitialized_data)?;
    reader.read_exact(&mut address_of_entry_point)?;
    reader.read_exact(&mut base_of_code)?;
    base_of_data = match magic {
        IMAGE_NT_OPTIONAL_HDR32_MAGIC | IMAGE_ROM_OPTIONAL_HDR_MAGIC => {
            let mut buf = [0u8; 4];
            reader.read_exact(&mut buf)?;
            Some(buf)
        }
        IMAGE_NT_OPTIONAL_HDR64_MAGIC => None,
        _ => panic!(),
    };
    reader.read_exact(&mut image_base)?;
    reader.read_exact(&mut section_alignment)?;
    reader.read_exact(&mut file_alignment)?;
    reader.read_exact(&mut major_operating_system_version)?;
    reader.read_exact(&mut minor_operating_system_version)?;
    reader.read_exact(&mut major_image_version)?;
    reader.read_exact(&mut minor_image_version)?;
    reader.read_exact(&mut major_subsystem_version)?;
    reader.read_exact(&mut minor_subsystem_version)?;
    reader.read_exact(&mut win32_version_value)?;
    reader.read_exact(&mut size_of_image)?;
    reader.read_exact(&mut size_of_headers)?;
    reader.read_exact(&mut check_sum)?;
    reader.read_exact(&mut subsystem)?;
    reader.read_exact(&mut dll_characteristics)?;
    reader.read_exact(&mut size_of_stack_reserve)?;
    reader.read_exact(&mut size_of_stack_commit)?;
    reader.read_exact(&mut size_of_heap_reserve)?;
    reader.read_exact(&mut size_of_heap_commit)?;
    reader.read_exact(&mut loader_flags)?;
    reader.read_exact(&mut number_of_rva_and_sizes)?;

    let data_dir_offset = reader.seek(SeekFrom::Current(0))?;
    for _ in 0..u32::from_le_bytes(number_of_rva_and_sizes) {
        let value = read_data_dir(reader, data_dir_offset)?;
        data_directories.push(value);
    }

    Ok(OptionalHeader {
        magic,
        major_linker_version,
        minor_linker_version,
        size_of_code,
        size_of_initialized_data,
        size_of_uninitialized_data,
        address_of_entry_point,
        base_of_code,
        base_of_data,
        image_base,
        section_alignment,
        file_alignment,
        major_operating_system_version,
        minor_operating_system_version,
        major_image_version,
        minor_image_version,
        major_subsystem_version,
        minor_subsystem_version,
        win32_version_value,
        size_of_image,
        size_of_headers,
        check_sum,
        subsystem,
        dll_characteristics,
        size_of_stack_reserve,
        size_of_stack_commit,
        size_of_heap_reserve,
        size_of_heap_commit,
        loader_flags,
        number_of_rva_and_sizes,
        data_directories,
    })
}

fn read_data_dir<R: Read + Seek>(reader: &mut R, data_dir_offset: u64) -> io::Result<DataDir> {
    let mut virtual_address: [u8; 4] = [0u8; 4];
    let mut size: [u8; 4] = [0u8; 4];
    reader.seek(SeekFrom::Start(data_dir_offset))?;
    reader.read_exact(&mut virtual_address)?;
    reader.read_exact(&mut size)?;
    Ok(DataDir {
        virtual_address,
        size,
    })
}
