use crate::header::{ImageType, RelativeVirtualAddress};
use crate::struct_parse::StructField;
use std::io::{Read, Seek, SeekFrom};

pub struct OptionalHeaderReader<R: Read + Seek> {
    offset: u64,
    reader: R,
}

impl<R: Read + Seek> OptionalHeaderReader<R> {
    pub fn new(offset: u64, reader: R) -> Self {
        Self { offset, reader }
    }

    pub fn read_optional_header_32(&mut self) -> OptionalHeader {
        let image_type = self.read_image_type();
        let major_linker_version = self.read_major_linker_version();
        let minor_linker_version = self.read_minor_linker_version();
        let size_of_code = self.read_size_of_code();
        let size_of_initialized_data = self.read_size_of_initialized_data();
        let size_of_uninitialized_data = self.read_size_of_uninitialized_data();
        let address_of_entry_point = self.read_address_of_entry_point();
        let base_of_code = self.read_base_of_code();
        let base_of_data = Some(self.read_base_of_data());
        let image_base = self.read_image_base_64();
        let section_alignment = self.read_section_alignment();
        let file_alignment = self.read_file_alignment();
        let major_operating_system_version = self.read_major_operating_system_version();
        let minor_operating_system_version = self.read_minor_operating_system_version();
        let major_image_version = self.read_major_image_version();
        let minor_image_version = self.read_minor_image_version();
        let major_subsystem_version = self.read_major_subsystem_version();
        let minor_subsystem_version = self.read_minor_subsystem_version();
        let win32_version_value = self.read_win32_version_value();
        let size_of_image = self.read_size_of_image();
        let size_of_headers = self.read_size_of_headers();
        let check_sum = self.read_check_sum();
        let subsystem = self.read_subsystem();
        let dll_characteristics = self.read_dll_characteristics();
        let size_of_stack_reserve = self.read_size_of_stack_reserve_32();
        let size_of_stack_commit = self.read_size_of_stack_commit_64();
        let size_of_heap_reserve = self.read_size_of_heap_reserve_64();
        let size_of_heap_commit = self.read_size_of_heap_commit_64();
        let loader_flags = self.read_loader_flags();
        let number_of_rva_and_sizes = self.read_number_of_rva_and_sizes();
        let data_directories = self.read_data_directories();

        OptionalHeader {
            image_type,
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
        }
    }

    pub fn read_optional_header_64(&mut self) -> OptionalHeader {
        let image_type = self.read_image_type();
        let major_linker_version = self.read_major_linker_version();
        let minor_linker_version = self.read_minor_linker_version();
        let size_of_code = self.read_size_of_code();
        let size_of_initialized_data = self.read_size_of_initialized_data();
        let size_of_uninitialized_data = self.read_size_of_uninitialized_data();
        let address_of_entry_point = self.read_address_of_entry_point();
        let base_of_code = self.read_base_of_code();
        let base_of_data = None;
        let image_base = self.read_image_base_64();
        let section_alignment = self.read_section_alignment();
        let file_alignment = self.read_file_alignment();
        let major_operating_system_version = self.read_major_operating_system_version();
        let minor_operating_system_version = self.read_minor_operating_system_version();
        let major_image_version = self.read_major_image_version();
        let minor_image_version = self.read_minor_image_version();
        let major_subsystem_version = self.read_major_subsystem_version();
        let minor_subsystem_version = self.read_minor_subsystem_version();
        let win32_version_value = self.read_win32_version_value();
        let size_of_image = self.read_size_of_image();
        let size_of_headers = self.read_size_of_headers();
        let check_sum = self.read_check_sum();
        let subsystem = self.read_subsystem();
        let dll_characteristics = self.read_dll_characteristics();
        let size_of_stack_reserve = self.read_size_of_stack_reserve_64();
        let size_of_stack_commit = self.read_size_of_stack_commit_64();
        let size_of_heap_reserve = self.read_size_of_heap_reserve_64();
        let size_of_heap_commit = self.read_size_of_heap_commit_64();
        let loader_flags = self.read_loader_flags();
        let number_of_rva_and_sizes = self.read_number_of_rva_and_sizes();
        let data_directories = self.read_data_directories();

        OptionalHeader {
            image_type,
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
        }
    }

    pub fn read_image_type(&mut self) -> ImageType {
        const RELATIVE_OFFSET: u64 = 0;
        let offset = self.offset + RELATIVE_OFFSET;
        let magic = self.read_array(offset);
        let data = ImageType::try_from(magic).unwrap();
        data
    }

    pub fn read_major_linker_version(&mut self) -> u8 {
        const RELATIVE_OFFSET: u64 = 2;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u8(offset)
    }

    pub fn read_minor_linker_version(&mut self) -> u8 {
        const RELATIVE_OFFSET: u64 = 3;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u8(offset)
    }

    pub fn read_size_of_code(&mut self) -> u32 {
        const RELATIVE_OFFSET: u64 = 4;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u32_le(offset)
    }

    pub fn read_size_of_initialized_data(&mut self) -> u32 {
        const RELATIVE_OFFSET: u64 = 8;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u32_le(offset)
    }

    pub fn read_size_of_uninitialized_data(&mut self) -> u32 {
        const RELATIVE_OFFSET: u64 = 12;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u32_le(offset)
    }

    pub fn read_address_of_entry_point(&mut self) -> u32 {
        const RELATIVE_OFFSET: u64 = 16;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u32_le(offset)
    }

    pub fn read_base_of_code(&mut self) -> u32 {
        const RELATIVE_OFFSET: u64 = 20;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u32_le(offset)
    }

    pub fn read_base_of_data(&mut self) -> u32 {
        const RELATIVE_OFFSET: u64 = 24;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u32_le(offset)
    }

    pub fn read_image_base_64(&mut self) -> u64 {
        const RELATIVE_OFFSET: u64 = 28;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u64_le(offset)
    }

    pub fn read_section_alignment(&mut self) -> u32 {
        const RELATIVE_OFFSET: u64 = 32;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u32_le(offset)
    }

    fn read_file_alignment(&mut self) -> u32 {
        const RELATIVE_OFFSET: u64 = 36;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u32_le(offset)
    }

    fn read_major_operating_system_version(&mut self) -> u16 {
        const RELATIVE_OFFSET: u64 = 40;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u16_le(offset)
    }

    fn read_minor_operating_system_version(&mut self) -> u16 {
        const RELATIVE_OFFSET: u64 = 42;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u16_le(offset)
    }

    fn read_major_image_version(&mut self) -> u16 {
        const RELATIVE_OFFSET: u64 = 44;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u16_le(offset)
    }

    fn read_minor_image_version(&mut self) -> u16 {
        const RELATIVE_OFFSET: u64 = 46;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u16_le(offset)
    }

    fn read_major_subsystem_version(&mut self) -> u16 {
        const RELATIVE_OFFSET: u64 = 48;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u16_le(offset)
    }

    fn read_minor_subsystem_version(&mut self) -> u16 {
        const RELATIVE_OFFSET: u64 = 50;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u16_le(offset)
    }

    fn read_win32_version_value(&mut self) -> u32 {
        const RELATIVE_OFFSET: u64 = 52;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u32_le(offset)
    }

    fn read_size_of_image(&mut self) -> u32 {
        const RELATIVE_OFFSET: u64 = 56;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u32_le(offset)
    }

    fn read_size_of_headers(&mut self) -> u32 {
        const RELATIVE_OFFSET: u64 = 60;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u32_le(offset)
    }

    fn read_check_sum(&mut self) -> u32 {
        const RELATIVE_OFFSET: u64 = 64;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u32_le(offset)
    }

    fn read_subsystem(&mut self) -> u16 {
        const RELATIVE_OFFSET: u64 = 68;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u16_le(offset)
    }

    fn read_dll_characteristics(&mut self) -> u16 {
        const RELATIVE_OFFSET: u64 = 70;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u16_le(offset)
    }

    fn read_size_of_stack_reserve_32(&mut self) -> u64 {
        const RELATIVE_OFFSET: u64 = 72;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u64_le(offset)
    }

    fn read_size_of_stack_reserve_64(&mut self) -> u64 {
        const RELATIVE_OFFSET: u64 = 72;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u64_le(offset)
    }

    fn read_size_of_stack_commit_32(&mut self) -> u64 {
        const RELATIVE_OFFSET: u64 = 76;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u64_le(offset)
    }

    fn read_size_of_stack_commit_64(&mut self) -> u64 {
        const RELATIVE_OFFSET: u64 = 80;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u64_le(offset)
    }

    fn read_size_of_heap_reserve_32(&mut self) -> u64 {
        const RELATIVE_OFFSET: u64 = 80;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u64_le(offset)
    }

    fn read_size_of_heap_reserve_64(&mut self) -> u64 {
        const RELATIVE_OFFSET: u64 = 88;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u64_le(offset)
    }

    fn read_size_of_heap_commit_64(&mut self) -> u64 {
        const RELATIVE_OFFSET: u64 = 96;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u64_le(offset)
    }

    fn read_loader_flags(&mut self) -> u32 {
        const RELATIVE_OFFSET: u64 = 94;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u32_le(offset)
    }

    fn read_number_of_rva_and_sizes(&mut self) -> u32 {
        const RELATIVE_OFFSET: u64 = 98;
        let offset = self.offset + RELATIVE_OFFSET;
        self.read_u32_le(offset)
    }

    fn read_data_directories(&mut self) -> Vec<DataDirectory> {
        const RELATIVE_OFFSET: u64 = 102;
        let _offset = self.offset + RELATIVE_OFFSET;
        todo!()
    }

    fn read_u8(&mut self, offset: u64) -> u8 {
        let buf: [u8; 1] = self.read_array(offset);
        buf[0]
    }

    fn read_u16_le(&mut self, offset: u64) -> u16 {
        let buf = self.read_array(offset);
        u16::from_le_bytes(buf)
    }

    fn read_u32_le(&mut self, offset: u64) -> u32 {
        let buf = self.read_array(offset);
        u32::from_le_bytes(buf)
    }

    fn read_u64_le(&mut self, offset: u64) -> u64 {
        let buf = self.read_array(offset);
        u64::from_le_bytes(buf)
    }

    fn read_array<const N: usize>(&mut self, offset: u64) -> [u8; N] {
        let pos = SeekFrom::Start(self.offset + offset);
        let _ = self.reader.seek(pos);
        let mut buf = [0u8; N];
        let _ = self.reader.read_exact(&mut buf);
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
#[derive(Debug, Clone)]
pub struct DataDirectoryReader<R: Read + Seek> {
    offset: u64,
    reader: R,
}

impl<R: Read + Seek> DataDirectoryReader<R> {
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
    /// The [`RVA`](crate::header::RelativeVirtualAddress) of the table
    pub virtual_address: RelativeVirtualAddress,
    /// Size in bytes
    pub size: u32,
    pub data_directory_type: DataDirectoryType,
}

#[derive(Debug)]
pub struct DataDirectoriesReader {
    offset: u64,
    buffer: Vec<u8>,
}

