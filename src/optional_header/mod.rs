use crate::StructField;

pub enum WindowsSubsystem {
    Unknown,
    Native,
    WindowsConsoleUI,
    OS2ConsoleUI,
    POSIXConsoleUI,
    NativeWindows,
    WindowsCEGraphicalUI,
    EFIApplication,
    EFIBootServiceDriver,
    EFIRuntimeDriver,
    EFIROM,
    Xbox,
    WindowsBootApplication,
}

const IMAGE_SUBSYSTEM_UNKNOWN: u16 = 0;
const IMAGE_SUBSYSTEM_NATIVE: u16 = 1;
const IMAGE_SUBSYSTEM_WINDOWS_GUI: u16 = 2;
const IMAGE_SUBSYSTEM_WINDOWS_CUI: u16 = 3;
const IMAGE_SUBSYSTEM_OS2_CUI: u16 = 5;
const IMAGE_SUBSYSTEM_POSIX_CUI: u16 = 7;
const IMAGE_SUBSYSTEM_NATIVE_WINDOWS: u16 = 8;
const IMAGE_SUBSYSTEM_WINDOWS_CE_GUI: u16 = 9;
const IMAGE_SUBSYSTEM_EFI_APPLICATION: u16 = 10;
const IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER: u16 = 11;
const IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER: u16 = 12;
const IMAGE_SUBSYSTEM_EFI_ROM: u16 = 13;
const IMAGE_SUBSYSTEM_XBOX: u16 = 14;
const IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION: u16 = 16;

struct OptionalHeader32Raw {
    magic: [u8; 2],
    major_linker_version: [u8; 1],
    minor_linker_version: [u8; 1],
    size_of_code: [u8; 4],
    size_of_initialized_data: [u8; 4],
    size_of_uninitialized_data: [u8; 4],
    address_of_entry_point: [u8; 4],
    base_of_code: [u8; 4],
    base_of_data: [u8; 4],
    image_base: [u8; 4],
    section_alignment: [u8; 4],
    file_alignment: [u8; 4],
    major_os_version: [u8; 2],
    minor_os_version: [u8; 2],
    major_image_version: [u8; 2],
    minor_image_version: [u8; 2],
    major_subsystem_version: [u8; 2],
    minor_subsystem_version: [u8; 2],
    win32_version_value: [u8; 4],
    size_of_image: [u8; 4],
    size_of_headers: [u8; 4],
    checksum: [u8; 4],
    subsystem: [u8; 2],
    dll_characteristics: [u8; 2],
    size_of_stack_reserve: [u8; 4],
    size_of_stack_commit: [u8; 4],
    size_of_heap_reserve: [u8; 4],
    size_of_heap_commit: [u8; 4],
    loader_flags: [u8; 4],
    number_of_rva_and_sizes: [u8; 4],
    data_directories: Vec<DataDirectoryRaw>,
}

struct OptionalHeader32 {
    offset: u64,
    optional_header_32_raw: OptionalHeader32Raw,
}

impl OptionalHeader32 {
    fn magic(&self) -> u16 {
        u16::from_le_bytes(self.optional_header_32_raw.magic)
    }

    fn major_linker_version(&self) -> u8 {
        u8::from_le_bytes(self.optional_header_32_raw.major_linker_version)
    }

    fn minor_linker_version(&self) -> u8 {
        u8::from_le_bytes(self.optional_header_32_raw.minor_linker_version)
    }

    fn size_of_code(&self) -> u32 {
        u32::from_le_bytes(self.optional_header_32_raw.size_of_code)
    }
    fn size_of_initialized_data(&self) -> u32 {
        u32::from_le_bytes(self.optional_header_32_raw.size_of_initialized_data)
    }

    fn size_of_uninitialized_data(&self) -> u32 {
        u32::from_le_bytes(self.optional_header_32_raw.size_of_uninitialized_data)
    }

    fn address_of_entry_point(&self) -> u32 {
        u32::from_le_bytes(self.optional_header_32_raw.address_of_entry_point)
    }

    fn base_of_code(&self) -> u32 {
        u32::from_le_bytes(self.optional_header_32_raw.base_of_code)
    }

    fn base_of_data(&self) -> u32 {
        u32::from_le_bytes(self.optional_header_32_raw.base_of_data)
    }

    fn image_base(&self) -> u32 {
        u32::from_le_bytes(self.optional_header_32_raw.image_base)
    }

    fn section_alignment(&self) -> u32 {
        u32::from_le_bytes(self.optional_header_32_raw.section_alignment)
    }

    fn file_alignment(&self) -> u32 {
        u32::from_le_bytes(self.optional_header_32_raw.file_alignment)
    }

    fn major_os_version(&self) -> u16 {
        u16::from_le_bytes(self.optional_header_32_raw.major_os_version)
    }

    fn minor_os_version(&self) -> u16 {
        u16::from_le_bytes(self.optional_header_32_raw.minor_os_version)
    }

    fn major_image_version(&self) -> u16 {
        u16::from_le_bytes(self.optional_header_32_raw.major_image_version)
    }

    fn minor_image_version(&self) -> u16 {
        u16::from_le_bytes(self.optional_header_32_raw.minor_image_version)
    }

    fn major_subsystem_version(&self) -> u16 {
        u16::from_le_bytes(self.optional_header_32_raw.major_subsystem_version)
    }

    fn minor_subsystem_version(&self) -> u16 {
        u16::from_le_bytes(self.optional_header_32_raw.minor_subsystem_version)
    }

    fn win32_version_value(&self) -> u32 {
        u32::from_le_bytes(self.optional_header_32_raw.win32_version_value)
    }

    fn size_of_image(&self) -> u32 {
        u32::from_le_bytes(self.optional_header_32_raw.size_of_image)
    }

    fn size_of_headers(&self) -> u32 {
        u32::from_le_bytes(self.optional_header_32_raw.size_of_headers)
    }

    fn checksum(&self) -> u32 {
        u32::from_le_bytes(self.optional_header_32_raw.checksum)
    }

    fn subsystem(&self) -> u16 {
        u16::from_le_bytes(self.optional_header_32_raw.subsystem)
    }

    fn dll_characteristics(&self) -> u16 {
        u16::from_le_bytes(self.optional_header_32_raw.dll_characteristics)
    }

    fn size_of_stack_reserve(&self) -> u32 {
        u32::from_le_bytes(self.optional_header_32_raw.size_of_stack_reserve)
    }

    fn size_of_stack_commit(&self) -> u32 {
        u32::from_le_bytes(self.optional_header_32_raw.size_of_stack_commit)
    }

    fn size_of_heap_reserve(&self) -> u32 {
        u32::from_le_bytes(self.optional_header_32_raw.size_of_heap_reserve)
    }

    fn size_of_heap_commit(&self) -> u32 {
        u32::from_le_bytes(self.optional_header_32_raw.size_of_heap_commit)
    }

    fn loader_flags(&self) -> u32 {
        u32::from_le_bytes(self.optional_header_32_raw.loader_flags)
    }

    fn number_of_rva_and_sizes(&self) -> u32 {
        u32::from_le_bytes(self.optional_header_32_raw.number_of_rva_and_sizes)
    }

    fn data_directories(&self) -> Vec<DataDirectory> {
        todo!()
    }
}

struct OptionalHeader32Wrapper {
    optional_header_32: OptionalHeader32,
}

struct OptionalHeader64Raw {
    magic: [u8; 2],
    major_linker_version: [u8; 1],
    minor_linker_version: [u8; 1],
    size_of_code: [u8; 4],
    size_of_initialized_data: [u8; 4],
    size_of_uninitialized_data: [u8; 4],
    address_of_entry_point: [u8; 4],
    base_of_code: [u8; 4],
    image_base: [u8; 8],
    section_alignment: [u8; 4],
    file_alignment: [u8; 4],
    major_os_version: [u8; 2],
    minor_os_version: [u8; 2],
    major_image_version: [u8; 2],
    minor_image_version: [u8; 2],
    major_subsystem_version: [u8; 2],
    minor_subsystem_version: [u8; 2],
    win32_version_value: [u8; 4],
    size_of_image: [u8; 4],
    size_of_headers: [u8; 4],
    checksum: [u8; 4],
    subsystem: [u8; 2],
    dll_characteristics: [u8; 2],
    size_of_stack_reserve: [u8; 8],
    size_of_stack_commit: [u8; 8],
    size_of_heap_reserve: [u8; 8],
    size_of_heap_commit: [u8; 8],
    loader_flags: [u8; 4],
    number_of_rva_and_sizes: [u8; 4],
    data_directories: Vec<DataDirectoryRaw>,
}

struct OptionalHeader64 {
    offset: u64,
    optional_header_64_raw: OptionalHeader64Raw,
}

impl OptionalHeader64 {
    fn magic(&self) -> u16 {
        todo!()
    }

    fn major_linker_version(&self) -> u8 {
        todo!()
    }

    fn minor_linker_version(&self) -> u8 {
        todo!()
    }

    fn size_of_code(&self) -> u32 {
        todo!()
    }
    fn size_of_initialized_data(&self) -> u32 {
        todo!()
    }

    fn size_of_uninitialized_data(&self) -> u32 {
        todo!()
    }

    fn address_of_entry_point(&self) -> u32 {
        todo!()
    }

    fn base_of_code(&self) -> u32 {
        todo!()
    }

    fn image_base(&self) -> u64 {
        todo!()
    }

    fn section_alignment(&self) -> u32 {
        todo!()
    }

    fn file_alignment(&self) -> u32 {
        todo!()
    }

    fn major_os_version(&self) -> u16 {
        todo!()
    }

    fn minor_os_version(&self) -> u16 {
        todo!()
    }

    fn major_image_version(&self) -> u16 {
        todo!()
    }

    fn minor_image_version(&self) -> u16 {
        todo!()
    }

    fn major_subsystem_version(&self) -> u16 {
        todo!()
    }

    fn minor_subsystem_version(&self) -> u16 {
        todo!()
    }

    fn win32_version_value(&self) -> u32 {
        todo!()
    }

    fn size_of_image(&self) -> u32 {
        todo!()
    }

    fn size_of_headers(&self) -> u32 {
        todo!()
    }

    fn checksum(&self) -> u32 {
        todo!()
    }

    fn subsystem(&self) -> u16 {
        todo!()
    }

    fn dll_characteristics(&self) -> u16 {
        todo!()
    }

    fn size_of_stack_reserve(&self) -> u64 {
        todo!()
    }

    fn size_of_stack_commit(&self) -> u64 {
        todo!()
    }

    fn size_of_heap_reserve(&self) -> u64 {
        todo!()
    }

    fn size_of_heap_commit(&self) -> u64 {
        todo!()
    }

    fn loader_flags(&self) -> u32 {
        todo!()
    }

    fn number_of_rva_and_sizes(&self) -> u32 {
        todo!()
    }

    fn data_directories(&self) -> Vec<DataDirectory> {
        todo!()
    }
}

struct OptionalHeader64Wrapper {
    optional_header_64: OptionalHeader64,
}

struct DataDirectoryRaw {
    virtual_address: [u8; 4],
    size: [u8; 4],
}

struct DataDirectory {
    offset: u64,
    data_directory_raw: DataDirectoryRaw,
}

struct DataDirectoryWrapper {
    data_directory: DataDirectory,
}

impl DataDirectoryWrapper {
    pub fn virtual_address(&self) -> StructField<u32, 4> {
        todo!()
    }

    pub fn size(&self) -> StructField<u32, 4> {
        todo!()
    }
}

struct DllCharacteristics {}
