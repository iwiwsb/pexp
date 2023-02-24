pub const IMAGE_SUBSYSTEM_UNKNOWN: [u8; 2] = [0x00, 0x00];
pub const IMAGE_SUBSYSTEM_NATIVE: [u8; 2] = [0x01, 0x00];
pub const IMAGE_SUBSYSTEM_WINDOWS_GUI: [u8; 2] = [0x02, 0x00];
pub const IMAGE_SUBSYSTEM_WINDOWS_CUI: [u8; 2] = [0x03, 0x00];
pub const IMAGE_SUBSYSTEM_OS2_CUI: [u8; 2] = [0x05, 0x00];
pub const IMAGE_SUBSYSTEM_POSIX_CUI: [u8; 2] = [0x07, 0x00];
pub const IMAGE_SUBSYSTEM_NATIVE_WINDOWS: [u8; 2] = [0x08, 0x00];
pub const IMAGE_SUBSYSTEM_WINDOWS_CE_GUI: [u8; 2] = [0x09, 0x00];
pub const IMAGE_SUBSYSTEM_EFI_APPLICATION: [u8; 2] = [0x0A, 0x00];
pub const IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER: [u8; 2] = [0x0B, 0x00];
pub const IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER: [u8; 2] = [0x0C, 0x00];
pub const IMAGE_SUBSYSTEM_EFI_ROM: [u8; 2] = [0x0D, 0x00];
pub const IMAGE_SUBSYSTEM_XBOX: [u8; 2] = [0x0E, 0x00];
pub const IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION: [u8; 2] = [0x10, 0x00];

pub struct WinSubsystem([u8; 2]);