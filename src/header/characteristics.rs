use std::fmt::{self, Binary, Formatter};

#[derive(Debug, PartialEq)]
pub struct Characteristics {
    flags: Vec<bool>,
}

impl Characteristics {
    /// Image only, Windows CE, and Microsoft Windows NT and later. This indicates that the file
    /// does not contain base relocations and must therefore be loaded at its preferred base address.
    /// If the base address is not available, the loader reports an error. The default behavior of
    /// the linker is to strip base relocations from executable (EXE) files.
    pub const IMAGE_FILE_RELOCS_STRIPPED: u16 = 0x0001;
    /// Image only. This indicates that the image file is valid and can be run. If this flag is not set,
    /// it indicates a linker error.
    pub const IMAGE_FILE_EXECUTABLE_IMAGE: u16 = 0x0002;
    /// COFF line numbers have been removed. This flag is deprecated and should be zero.
    pub const IMAGE_FILE_LINE_NUMS_STRIPPED: u16 = 0x0004;
    /// COFF symbol table entries for local symbols have been removed. This flag is deprecated and
    /// should be zero.
    pub const IMAGE_FILE_LOCAL_SYMS_STRIPPED: u16 = 0x0008;
    /// Obsolete. Aggressively trim working set. This flag is deprecated for Windows 2000 and later and must be zero.
    pub const IMAGE_FILE_AGGRESSIVE_WS_TRIM: u16 = 0x0010;
    /// Application can handle > 2-GB addresses.
    pub const IMAGE_FILE_LARGE_ADDRESS_AWARE: u16 = 0x0020;
    /// This flag is reserved for future use.
    pub const IMAGE_FILE_RESERVED0: u16 = 0x0040;
    /// Little endian: the least significant bit (LSB) precedes the most significant bit (MSB) in
    /// memory. This flag is deprecated and should be zero.
    pub const IMAGE_FILE_BYTES_REVERSED_LO: u16 = 0x0080;
    /// Machine is based on a 32-bit-word architecture.
    pub const IMAGE_FILE_32BIT_MACHINE: u16 = 0x0100;
    /// Debugging information is removed from the image file.
    pub const IMAGE_FILE_DEBUG_STRIPPED: u16 = 0x0200;
    /// If the image is on removable media, fully load it and copy it to the swap file.
    pub const IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP: u16 = 0x0400;
    /// If the image is on network media, fully load it and copy it to the swap file.
    pub const IMAGE_FILE_NET_RUN_FROM_SWAP: u16 = 0x0800;
    /// The image file is a system file, not a user program.
    pub const IMAGE_FILE_SYSTEM: u16 = 0x1000;
    /// The image file is a dynamic-link library (DLL). Such files are considered executable files for
    /// almost all purposes, although they cannot be directly run.
    pub const IMAGE_FILE_DLL: u16 = 0x2000;
    /// The file should be run only on a uniprocessor machine.
    pub const IMAGE_FILE_UP_SYSTEM_ONLY: u16 = 0x4000;
    /// Big endian: the MSB precedes the LSB in memory. This flag is deprecated and should be zero.
    pub const IMAGE_FILE_BYTES_REVERSED_HI: u16 = 0x8000;

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

impl From<u16> for Characteristics {
    fn from(value: u16) -> Self {
        let mut flags: Vec<bool> = Vec::new();

        flags[0] = (value & Self::IMAGE_FILE_RELOCS_STRIPPED) == Self::IMAGE_FILE_RELOCS_STRIPPED;
        flags[1] = (value & Self::IMAGE_FILE_EXECUTABLE_IMAGE) == Self::IMAGE_FILE_EXECUTABLE_IMAGE;
        flags[2] =
            (value & Self::IMAGE_FILE_LINE_NUMS_STRIPPED) == Self::IMAGE_FILE_LINE_NUMS_STRIPPED;
        flags[3] =
            (value & Self::IMAGE_FILE_LOCAL_SYMS_STRIPPED) == Self::IMAGE_FILE_LOCAL_SYMS_STRIPPED;
        flags[4] =
            (value & Self::IMAGE_FILE_AGGRESSIVE_WS_TRIM) == Self::IMAGE_FILE_AGGRESSIVE_WS_TRIM;
        flags[5] =
            (value & Self::IMAGE_FILE_LARGE_ADDRESS_AWARE) == Self::IMAGE_FILE_LARGE_ADDRESS_AWARE;
        flags[6] = (value & Self::IMAGE_FILE_RESERVED0) == Self::IMAGE_FILE_RESERVED0;
        flags[7] =
            (value & Self::IMAGE_FILE_BYTES_REVERSED_LO) == Self::IMAGE_FILE_BYTES_REVERSED_LO;
        flags[8] = (value & Self::IMAGE_FILE_32BIT_MACHINE) == Self::IMAGE_FILE_32BIT_MACHINE;
        flags[9] = (value & Self::IMAGE_FILE_DEBUG_STRIPPED) == Self::IMAGE_FILE_DEBUG_STRIPPED;
        flags[10] = (value & Self::IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP)
            == Self::IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP;
        flags[11] =
            (value & Self::IMAGE_FILE_NET_RUN_FROM_SWAP) == Self::IMAGE_FILE_NET_RUN_FROM_SWAP;
        flags[12] = (value & Self::IMAGE_FILE_SYSTEM) == Self::IMAGE_FILE_SYSTEM;
        flags[13] = (value & Self::IMAGE_FILE_DLL) == Self::IMAGE_FILE_DLL;
        flags[14] = (value & Self::IMAGE_FILE_UP_SYSTEM_ONLY) == Self::IMAGE_FILE_UP_SYSTEM_ONLY;
        flags[15] =
            (value & Self::IMAGE_FILE_BYTES_REVERSED_HI) == Self::IMAGE_FILE_BYTES_REVERSED_HI;

        Self { flags }
    }
}

impl Binary for Characteristics {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:016b}", self.to_bits()))
    }
}
