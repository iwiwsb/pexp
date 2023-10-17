use crate::struct_parse::StructField;
use std::io::{Read, Seek, SeekFrom};

pub struct OptionalHeaderReader<R: Read + Seek> {
    /// OptionalHeader offset from the beginning of the file
    offset: u64,
    reader: R,
}

impl<R: Read + Seek> OptionalHeaderReader<R> {
    pub fn new(offset: u64, reader: R) -> Self {
        Self { offset, reader }
    }

    pub fn read_optional_header_32(&mut self) -> OptionalHeader {
        let image_type = self.read_array_field(0);
        let major_linker_version = self.read_array_field(2);
        let minor_linker_version = self.read_array_field(3);
        let size_of_code = self.read_array_field(4);
        let size_of_initialized_data = self.read_array_field(8);
        let size_of_uninitialized_data = self.read_array_field(12);
        let address_of_entry_point = self.read_array_field(16);
        let base_of_code = self.read_array_field(20);
        let base_of_data = Some(self.read_array_field(24));
        let image_base = self.read_array_field(28);
        let section_alignment = self.read_array_field(32);
        let file_alignment = self.read_array_field(36);
        let major_operating_system_version = self.read_array_field(40);
        let minor_operating_system_version = self.read_array_field(42);
        let major_image_version = self.read_array_field(44);
        let minor_image_version = self.read_array_field(46);
        let major_subsystem_version = self.read_array_field(48);
        let minor_subsystem_version = self.read_array_field(50);
        let win32_version_value = self.read_array_field(52);
        let size_of_image = self.read_array_field(56);
        let size_of_headers = self.read_array_field(60);
        let check_sum = self.read_array_field(64);
        let subsystem = self.read_array_field(68);
        let dll_characteristics = self.read_array_field(70);
        let size_of_stack_reserve = self.read_array_field(72);
        let size_of_stack_commit = self.read_array_field(76);
        let size_of_heap_reserve = self.read_array_field(80);
        let size_of_heap_commit = self.read_array_field(84);
        let loader_flags = self.read_array_field(88);
        let number_of_rva_and_sizes = self.read_array_field(92);
        let data_directories = self.read_data_directories(96);

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
        let image_type = self.read_array_field(0);
        let major_linker_version = self.read_array_field(2);
        let minor_linker_version = self.read_array_field(3);
        let size_of_code = self.read_array_field(4);
        let size_of_initialized_data = self.read_array_field(8);
        let size_of_uninitialized_data = self.read_array_field(12);
        let address_of_entry_point = self.read_array_field(16);
        let base_of_code = self.read_array_field(20);
        let base_of_data = None;
        let image_base = self.read_array_field(24);
        let section_alignment = self.read_array_field(32);
        let file_alignment = self.read_array_field(36);
        let major_operating_system_version = self.read_array_field(40);
        let minor_operating_system_version = self.read_array_field(42);
        let major_image_version = self.read_array_field(44);
        let minor_image_version = self.read_array_field(46);
        let major_subsystem_version = self.read_array_field(48);
        let minor_subsystem_version = self.read_array_field(50);
        let win32_version_value = self.read_array_field(52);
        let size_of_image = self.read_array_field(56);
        let size_of_headers = self.read_array_field(60);
        let check_sum = self.read_array_field(64);
        let subsystem = self.read_array_field(68);
        let dll_characteristics = self.read_array_field(70);
        let size_of_stack_reserve = self.read_array_field(72);
        let size_of_stack_commit = self.read_array_field(80);
        let size_of_heap_reserve = self.read_array_field(88);
        let size_of_heap_commit = self.read_array_field(96);
        let loader_flags = self.read_array_field(104);
        let number_of_rva_and_sizes = self.read_array_field(108);
        let data_directories = self.read_data_directories(112);

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

    fn read_data_directories(&mut self, relative_offset: u64) -> DataDirectories {
        todo!()
    }

    fn read_array_field<const N: usize>(&mut self, relative_offset: u64) -> StructField<[u8; N]> {
        let pos = SeekFrom::Start(self.offset + relative_offset);
        let _ = self.reader.seek(pos);
        let mut data = [0u8; N];
        let _ = self.reader.read_exact(&mut data);
        StructField {
            abs_offset: self.offset + relative_offset,
            data,
        }
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
    pub image_type: StructField<[u8; 2]>,

    /// The linker major version number.
    pub major_linker_version: StructField<[u8; 1]>,

    /// The linker minor version number.
    pub minor_linker_version: StructField<[u8; 1]>,

    /// The size of the code (`.text`) section, or the sum of all code sections if there are multiple sections.
    pub size_of_code: StructField<[u8; 4]>,

    /// The size of the initialized data section, or the sum of all such sections if there are multiple data sections.
    pub size_of_initialized_data: StructField<[u8; 4]>,

    /// The size of the uninitialized data section (`BSS`), or the sum of all such sections if there are multiple `BSS` sections.
    pub size_of_uninitialized_data: StructField<[u8; 4]>,

    /// The address of the entry point relative to the image base when the executable file is loaded into memory.
    /// For program images, this is the starting address.
    /// For device drivers, this is the address of the initialization function.
    /// An entry point is optional for DLLs.
    pub address_of_entry_point: StructField<[u8; 4]>,

    /// The address that is relative to the image base of the beginning-of-code section when it is loaded into memory.
    pub base_of_code: StructField<[u8; 4]>,

    /// The address that is relative to the image base of the beginning-of-data section when it is loaded into memory.
    /// PE32 contains this additional field, which is absent in PE32+
    pub base_of_data: Option<StructField<[u8; 4]>>,

    /// The preferred address of the first byte of image when loaded into memory; must be a multiple of 64 K.
    /// The default for DLLs is `0x10000000`.
    /// The default for Windows CE EXEs is `0x00010000`.
    /// The default for Windows NT, Windows 2000, Windows XP, Windows 95, Windows 98, and Windows Me is `0x00400000`.
    pub image_base: StructField<[u8; 8]>,

    /// The alignment (in bytes) of sections when they are loaded into memory. It must be greater than or equal to FileAlignment. The default is the page size for the architecture.
    pub section_alignment: StructField<[u8; 4]>,

    /// The alignment factor (in bytes) that is used to align the raw data of sections in the image file. The value should be a power of 2 between 512 and 64 K, inclusive. The default is 512. If the SectionAlignment is less than the architecture's page size, then FileAlignment must match SectionAlignment.
    pub file_alignment: StructField<[u8; 4]>,

    /// The major version number of the required operating system.
    pub major_operating_system_version: StructField<[u8; 2]>,

    /// The minor version number of the required operating system.
    pub minor_operating_system_version: StructField<[u8; 2]>,

    /// The major version number of the image.
    pub major_image_version: StructField<[u8; 2]>,

    /// The minor version number of the image.
    pub minor_image_version: StructField<[u8; 2]>,

    /// The major version number of the subsystem.
    pub major_subsystem_version: StructField<[u8; 2]>,

    /// The minor version number of the subsystem.
    pub minor_subsystem_version: StructField<[u8; 2]>,

    /// Reserved, must be zero.
    pub win32_version_value: StructField<[u8; 4]>,

    /// The size (in bytes) of the image, including all headers, as the image is loaded in memory.
    /// It must be a multiple of `section_alignment`.
    pub size_of_image: StructField<[u8; 4]>,

    /// The combined size of an MS-DOS stub, PE header, and section headers rounded up to a multiple of `file_alignment`.
    pub size_of_headers: StructField<[u8; 4]>,

    /// The image file checksum.
    /// The algorithm for computing the checksum is incorporated into IMAGHELP.DLL.
    /// The following are checked for validation at load time: all drivers, any DLL loaded at boot time, and any DLL that is loaded into a critical Windows process.
    pub check_sum: StructField<[u8; 4]>,

    /// The subsystem that is required to run this image. For more information, see [`win_subsystem`](crate::header::win_subsystem) module.
    pub subsystem: StructField<[u8; 2]>,

    /// See [`dll_characteristics`](crate::header::dll_characteristics) module.
    pub dll_characteristics: StructField<[u8; 2]>,

    /// The size of the stack to reserve. Only `size_of_stack_commit` is committed; the rest is made available one page at a time until the reserve size is reached.
    pub size_of_stack_reserve: StructField<[u8; 8]>,

    /// The size of the stack to commit.
    pub size_of_stack_commit: StructField<[u8; 8]>,

    /// The size of the local heap space to reserve. Only `size_of_heap_commit` is committed; the rest is made available one page at a time until the reserve size is reached.
    pub size_of_heap_reserve: StructField<[u8; 8]>,

    /// The size of the local heap space to commit.
    pub size_of_heap_commit: StructField<[u8; 8]>,

    /// Reserved, must be zero.
    pub loader_flags: StructField<[u8; 4]>,

    /// The number of data-directory entries in the remainder of the optional header. Each describes a location and size.
    pub number_of_rva_and_sizes: StructField<[u8; 4]>,

    /// Address/size pairs for special tables that are found in the image file and are used by the operating system (for example, the import table and the export table).
    /// Note that the number of directories is not fixed. Before looking for a specific directory,
    /// check the `number_of_rva_and_sizes` field.
    pub data_directories: DataDirectories,
}

#[derive(Debug)]
pub struct DataDirectories {
    /// The export table address and size.
    pub export: Option<StructField<DataDirectory>>,

    /// The import table address and size.
    pub import: Option<StructField<DataDirectory>>,

    /// The resource table address and size.
    pub resource: Option<StructField<DataDirectory>>,

    /// The exception table address and size.
    pub exception: Option<StructField<DataDirectory>>,

    // The attribute certificate table address and size.
    /// This entry points to a table of attribute certificates.
    /// These certificates are not loaded into memory as part of the image.
    /// As such, the first field of this entry, which is normally an [`RVA`](RelativeVirtualAddress), is a file pointer instead.
    pub certificate: Option<StructField<DataDirectory>>,

    /// The base relocation table address and size.
    pub base_relocation: Option<StructField<DataDirectory>>,

    /// The debug data starting address and size.
    pub debug: Option<StructField<DataDirectory>>,

    /// Reserved, must be 0
    pub architecture: Option<StructField<DataDirectory>>,

    /// The [`RVA`](RelativeVirtualAddress) of the value to be stored in the global pointer register. The size member of this structure must be set to zero.
    pub global_ptr: Option<StructField<DataDirectory>>,

    /// The thread local storage (TLS) table address and size.
    pub tls_table: Option<StructField<DataDirectory>>,

    /// The load configuration table address and size.
    pub load_config_table: Option<StructField<DataDirectory>>,

    /// The bound import table address and size.
    pub bound_import: Option<StructField<DataDirectory>>,

    /// The import address table address and size.
    pub import_address_table: Option<StructField<DataDirectory>>,

    /// The delay import descriptor address and size.
    pub delay_import_descriptor: Option<StructField<DataDirectory>>,

    /// The CLR runtime header address and size.
    pub clr_runtime_header: Option<StructField<DataDirectory>>,
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
    fn read_array_field<const N: usize>(&mut self, relative_offset: u64) -> StructField<[u8; N]> {
        let pos = SeekFrom::Start(self.offset + relative_offset);
        let _ = self.reader.seek(pos);
        let mut data = [0u8; N];
        let _ = self.reader.read_exact(&mut data);
        StructField {
            abs_offset: self.offset + relative_offset,
            data,
        }
    }

    fn read_data_directory(
        &mut self,
        offset: u64,
        data_directory_type: DataDirectoryType,
    ) -> StructField<DataDirectory> {
        let virtual_address = self.read_array_field(offset);
        let size = self.read_array_field(offset + 4);
        StructField {
            abs_offset: self.offset + offset,
            data: DataDirectory {
                virtual_address,
                size,
                data_directory_type,
            },
        }
    }
}

#[derive(Debug)]
pub struct DataDirectory {
    /// The [`RVA`](crate::header::RelativeVirtualAddress) of the table
    pub virtual_address: StructField<[u8; 4]>,
    /// Size in bytes
    pub size: StructField<[u8; 4]>,
    pub data_directory_type: DataDirectoryType,
}
