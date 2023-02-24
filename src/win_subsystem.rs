//! The following values defined for the [`subsystem`](crate::header::OptionalHeader#structfield.subsystem) field of the [`OptionalHeader`](crate::header::OptionalHeader)
//! determine which Windows subsystem (if any) is required to run the image.

/// An unknown subsystem
pub const IMAGE_SUBSYSTEM_UNKNOWN: [u8; 2] = [0x00, 0x00];
/// Device drivers and native Windows processes
pub const IMAGE_SUBSYSTEM_NATIVE: [u8; 2] = [0x01, 0x00];
/// The Windows graphical user interface (GUI) subsystem
pub const IMAGE_SUBSYSTEM_WINDOWS_GUI: [u8; 2] = [0x02, 0x00];
/// The Windows character subsystem
pub const IMAGE_SUBSYSTEM_WINDOWS_CUI: [u8; 2] = [0x03, 0x00];
/// The OS/2 character subsystem
pub const IMAGE_SUBSYSTEM_OS2_CUI: [u8; 2] = [0x05, 0x00];
/// The Posix character subsystem
pub const IMAGE_SUBSYSTEM_POSIX_CUI: [u8; 2] = [0x07, 0x00];
/// Native Win9x driver
pub const IMAGE_SUBSYSTEM_NATIVE_WINDOWS: [u8; 2] = [0x08, 0x00];
/// Windows CE
pub const IMAGE_SUBSYSTEM_WINDOWS_CE_GUI: [u8; 2] = [0x09, 0x00];
/// An Extensible Firmware Interface (EFI) application
pub const IMAGE_SUBSYSTEM_EFI_APPLICATION: [u8; 2] = [0x0A, 0x00];
/// An EFI driver with boot services
pub const IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER: [u8; 2] = [0x0B, 0x00];
/// An EFI driver with run-time services
pub const IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER: [u8; 2] = [0x0C, 0x00];
/// An EFI ROM image
pub const IMAGE_SUBSYSTEM_EFI_ROM: [u8; 2] = [0x0D, 0x00];
/// XBOX
pub const IMAGE_SUBSYSTEM_XBOX: [u8; 2] = [0x0E, 0x00];
/// Windows boot application.
pub const IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION: [u8; 2] = [0x10, 0x00];

pub struct WinSubsystem([u8; 2]);
