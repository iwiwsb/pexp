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

struct StandardFieldsRaw {
    magic: [u8; 2],
    major_linker_version: [u8; 1],
    minor_linker_version: [u8; 1],
    size_of_code: [u8; 4],
    size_of_initialized_data: [u8; 4],
    size_of_uninitialized_data: [u8; 4],
    address_of_entry_point: [u8; 4],
    base_of_code: [u8; 4],
}

struct WinSpecificFields32Raw {
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
}

struct OptionalHeaderRaw32 {
    std_fields: StandardFieldsRaw,
    base_of_data: [u8; 4],
    win_specific_fields: WinSpecificFields32Raw,
    data_directories: Vec<DataDirectoryRaw>,
}

struct WinSpecificFields64Raw {
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
}

struct OptionalHeaderRaw64 {
    std_fields: StandardFieldsRaw,
    win_specific_fields: WinSpecificFields64Raw,
    data_directories: Vec<DataDirectoryRaw>,
}

struct DataDirectoryRaw {
    virtual_address: [u8; 4],
    size: [u8; 4],
}

struct StandardFields {
    offset: u64,
    std_fields_raw: StandardFieldsRaw,
}

impl StandardFields {
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
}
