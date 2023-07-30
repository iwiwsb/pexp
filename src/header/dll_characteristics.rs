//! The following values are defined for the [`dll_characteristics`](crate::header::OptionalHeader#structfield.dll_characteristics) field of the [`OptionalHeader`](crate::header::OptionalHeader).

/// Image can handle a high entropy 64-bit virtual address space
pub const IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA: [u8; 2] = [0x20, 0x00];
/// DLL can be relocated at load time
pub const IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE: [u8; 2] = [0x40, 0x00];
/// Code Integrity checks are enforced
pub const IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY: [u8; 2] = [0x80, 0x00];
/// Image is NX compatible
pub const IMAGE_DLLCHARACTERISTICS_NX_COMPAT: [u8; 2] = [0x00, 0x01];
/// Isolation aware, but do not isolate the image
pub const IMAGE_DLLCHARACTERISTICS_NO_ISOLATION: [u8; 2] = [0x00, 0x02];
/// Does not use structured exception (SE) handling. No SE handler may be called in this image
pub const IMAGE_DLLCHARACTERISTICS_NO_SEH: [u8; 2] = [0x00, 0x04];
/// Do not bind the image
pub const IMAGE_DLLCHARACTERISTICS_NO_BIND: [u8; 2] = [0x00, 0x08];
/// Image must execute in an [AppContainer](https://learn.microsoft.com/en-us/windows/win32/secauthz/appcontainer-isolation)
pub const IMAGE_DLLCHARACTERISTICS_APPCONTAINER: [u8; 2] = [0x00, 0x10];
/// A WDM driver
pub const IMAGE_DLLCHARACTERISTICS_WDM_DRIVER: [u8; 2] = [0x00, 0x20];
/// Image supports [Control Flow Guard](https://learn.microsoft.com/en-us/windows/win32/secbp/control-flow-guard)
pub const IMAGE_DLLCHARACTERISTICS_GUARD_CF: [u8; 2] = [0x00, 0x40];
/// Terminal Server aware
pub const IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE: [u8; 2] = [0x00, 0x80];
