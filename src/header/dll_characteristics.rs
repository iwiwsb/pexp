//! The following values are defined for the [`dll_characteristics`](crate::header::optional_header::OptionalHeader#structfield.dll_characteristics) field of
//! the [`OptionalHeader`](crate::header::optional_header::OptionalHeader).

use std::fmt::{Binary, LowerHex, UpperHex};

#[derive(Debug)]
pub struct DllCharacteristics {
    flags: [bool; 16],
}

impl DllCharacteristics {
    /// Image can handle a high entropy 64-bit virtual address space
    pub const IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA: u16 = 0x0020;
    /// DLL can be relocated at load time
    pub const IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE: u16 = 0x0040;
    /// Code Integrity checks are enforced
    pub const IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY: u16 = 0x0080;
    /// Image is NX compatible
    pub const IMAGE_DLLCHARACTERISTICS_NX_COMPAT: u16 = 0x0100;
    /// Isolation aware, but do not isolate the image
    pub const IMAGE_DLLCHARACTERISTICS_NO_ISOLATION: u16 = 0x0200;
    /// Does not use structured exception (SE) handling. No SE handler may be called in this image
    pub const IMAGE_DLLCHARACTERISTICS_NO_SEH: u16 = 0x0400;
    /// Do not bind the image
    pub const IMAGE_DLLCHARACTERISTICS_NO_BIND: u16 = 0x0800;
    /// Image must execute in an [AppContainer](https://learn.microsoft.com/en-us/windows/win32/secauthz/appcontainer-isolation)
    pub const IMAGE_DLLCHARACTERISTICS_APPCONTAINER: u16 = 0x1000;
    /// A WDM driver
    pub const IMAGE_DLLCHARACTERISTICS_WDM_DRIVER: u16 = 0x2000;
    /// Image supports [Control Flow Guard](https://learn.microsoft.com/en-us/windows/win32/secbp/control-flow-guard)
    pub const IMAGE_DLLCHARACTERISTICS_GUARD_CF: u16 = 0x4000;
    /// Terminal Server aware
    pub const IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE: u16 = 0x8000;

    pub fn to_bits(&self) -> u16 {
        (self.flags[0] as u16) << 15
            | (self.flags[1] as u16) << 14
            | (self.flags[2] as u16) << 13
            | (self.flags[3] as u16) << 12
            | (self.flags[4] as u16) << 11
            | (self.flags[5] as u16) << 10
            | (self.flags[6] as u16) << 9
            | (self.flags[7] as u16) << 8
            | (self.flags[8] as u16) << 7
            | (self.flags[9] as u16) << 6
            | (self.flags[10] as u16) << 5
            | (self.flags[11] as u16) << 4
            | (self.flags[12] as u16) << 3
            | (self.flags[13] as u16) << 2
            | (self.flags[14] as u16) << 1
            | (self.flags[15] as u16)
    }
}

impl From<u16> for DllCharacteristics {
    fn from(value: u16) -> Self {
        let mut flags = [false; 16];
        flags[0] = (value & 0x0001) == 0x0001;
        flags[1] = (value & 0x0002) == 0x0002;
        flags[2] = (value & 0x0004) == 0x0004;
        flags[3] = (value & 0x0008) == 0x0008;
        flags[4] = (value & 0x0010) == 0x0010;
        flags[5] = (value & Self::IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA)
            == Self::IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA;
        flags[6] = (value & Self::IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE)
            == Self::IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE;
        flags[7] = (value & Self::IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY)
            == Self::IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY;
        flags[8] = (value & Self::IMAGE_DLLCHARACTERISTICS_NX_COMPAT)
            == Self::IMAGE_DLLCHARACTERISTICS_NX_COMPAT;
        flags[9] = (value & Self::IMAGE_DLLCHARACTERISTICS_NO_ISOLATION)
            == Self::IMAGE_DLLCHARACTERISTICS_NO_ISOLATION;
        flags[10] = (value & Self::IMAGE_DLLCHARACTERISTICS_NO_SEH)
            == Self::IMAGE_DLLCHARACTERISTICS_NO_SEH;
        flags[11] = (value & Self::IMAGE_DLLCHARACTERISTICS_NO_BIND)
            == Self::IMAGE_DLLCHARACTERISTICS_NO_BIND;
        flags[12] = (value & Self::IMAGE_DLLCHARACTERISTICS_APPCONTAINER)
            == Self::IMAGE_DLLCHARACTERISTICS_APPCONTAINER;
        flags[13] = (value & Self::IMAGE_DLLCHARACTERISTICS_WDM_DRIVER)
            == Self::IMAGE_DLLCHARACTERISTICS_WDM_DRIVER;
        flags[14] = (value & Self::IMAGE_DLLCHARACTERISTICS_GUARD_CF)
            == Self::IMAGE_DLLCHARACTERISTICS_GUARD_CF;
        flags[15] = (value & Self::IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE)
            == Self::IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE;
        Self { flags }
    }
}

impl Binary for DllCharacteristics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:016b}", self.to_bits())
    }
}

impl UpperHex for DllCharacteristics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:02X}", self.to_bits())
    }
}

impl LowerHex for DllCharacteristics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:02x}", self.to_bits())
    }
}
