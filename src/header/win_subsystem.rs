//! The following values defined for the [`subsystem`](crate::header::optional_header::OptionalHeader#structfield.subsystem) field of the [`OptionalHeader`](crate::header::optional_header::OptionalHeader)
//! determine which Windows subsystem (if any) is required to run the image.

pub struct WinSubSystem {
    flags: [bool; 16],
}

impl WinSubSystem {
    /// An unknown subsystem
    pub const IMAGE_SUBSYSTEM_UNKNOWN: u16 = 0x0000;
    /// Device drivers and native Windows processes
    pub const IMAGE_SUBSYSTEM_NATIVE: u16 = 0x0001;
    /// The Windows graphical user interface (GUI) subsystem
    pub const IMAGE_SUBSYSTEM_WINDOWS_GUI: u16 = 0x0002;
    /// The Windows character subsystem
    pub const IMAGE_SUBSYSTEM_WINDOWS_CUI: u16 = 0x0003;
    /// The OS/2 character subsystem
    pub const IMAGE_SUBSYSTEM_OS2_CUI: u16 = 0x0005;
    /// The Posix character subsystem
    pub const IMAGE_SUBSYSTEM_POSIX_CUI: u16 = 0x0007;
    /// Native Win9x driver
    pub const IMAGE_SUBSYSTEM_NATIVE_WINDOWS: u16 = 0x0008;
    /// Windows CE
    pub const IMAGE_SUBSYSTEM_WINDOWS_CE_GUI: u16 = 0x0009;
    /// An Extensible Firmware Interface (EFI) application
    pub const IMAGE_SUBSYSTEM_EFI_APPLICATION: u16 = 0x000A;
    /// An EFI driver with boot services
    pub const IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER: u16 = 0x000B;
    /// An EFI driver with run-time services
    pub const IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER: u16 = 0x000C;
    /// An EFI ROM image
    pub const IMAGE_SUBSYSTEM_EFI_ROM: u16 = 0x000D;
    /// XBOX
    pub const IMAGE_SUBSYSTEM_XBOX: u16 = 0x000E;
    /// Windows boot application.
    pub const IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION: u16 = 0x0010;
}
