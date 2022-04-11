#[allow(non_snake_case)]
pub struct WindowsSpecificFields {
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
