use std::fmt::{self, Binary, Formatter};

#[derive(Debug, PartialEq)]
pub struct Characteristics {
    pub relocs_stripped: bool,
    pub executable: bool,
    pub line_nums_stripped: bool,
    pub local_syms_stripped: bool,
    pub aggressive_ws_trim: bool,
    pub large_address_aware: bool,
    pub reserved0: bool,
    pub bytes_reserved_lo: bool,
    pub machine_32_bit: bool,
    pub debug_stripped: bool,
    pub removable_run_from_swap: bool,
    pub net_run_from_swap: bool,
    pub system: bool,
    pub dynamic_link_library: bool,
    pub up_system_only: bool,
    pub bytes_reserved_hi: bool,
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
        (self.relocs_stripped as u16) << 15
            | (self.executable as u16) << 14
            | (self.line_nums_stripped as u16) << 13
            | (self.local_syms_stripped as u16) << 12
            | (self.aggressive_ws_trim as u16) << 11
            | (self.large_address_aware as u16) << 10
            | (self.reserved0 as u16) << 9
            | (self.bytes_reserved_lo as u16) << 8
            | (self.machine_32_bit as u16) << 7
            | (self.debug_stripped as u16) << 6
            | (self.removable_run_from_swap as u16) << 5
            | (self.net_run_from_swap as u16) << 4
            | (self.system as u16) << 3
            | (self.dynamic_link_library as u16) << 2
            | (self.up_system_only as u16) << 1
            | (self.bytes_reserved_hi as u16)
    }
}

impl From<u16> for Characteristics {
    fn from(value: u16) -> Self {
        let relocs_stripped =
            (value & Self::IMAGE_FILE_RELOCS_STRIPPED) == Self::IMAGE_FILE_RELOCS_STRIPPED;
        let executable =
            (value & Self::IMAGE_FILE_EXECUTABLE_IMAGE) == Self::IMAGE_FILE_EXECUTABLE_IMAGE;
        let line_nums_stripped =
            (value & Self::IMAGE_FILE_LINE_NUMS_STRIPPED) == Self::IMAGE_FILE_LINE_NUMS_STRIPPED;
        let local_syms_stripped =
            (value & Self::IMAGE_FILE_LOCAL_SYMS_STRIPPED) == Self::IMAGE_FILE_LOCAL_SYMS_STRIPPED;
        let aggressive_ws_trim =
            (value & Self::IMAGE_FILE_AGGRESSIVE_WS_TRIM) == Self::IMAGE_FILE_AGGRESSIVE_WS_TRIM;
        let large_address_aware =
            (value & Self::IMAGE_FILE_LARGE_ADDRESS_AWARE) == Self::IMAGE_FILE_LARGE_ADDRESS_AWARE;
        let reserved0 = (value & Self::IMAGE_FILE_RESERVED0) == Self::IMAGE_FILE_RESERVED0;
        let bytes_reserved_lo =
            (value & Self::IMAGE_FILE_BYTES_REVERSED_LO) == Self::IMAGE_FILE_BYTES_REVERSED_LO;
        let machine_32_bit =
            (value & Self::IMAGE_FILE_32BIT_MACHINE) == Self::IMAGE_FILE_32BIT_MACHINE;
        let debug_stripped =
            (value & Self::IMAGE_FILE_DEBUG_STRIPPED) == Self::IMAGE_FILE_DEBUG_STRIPPED;
        let removable_run_from_swap = (value & Self::IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP)
            == Self::IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP;
        let net_run_from_swap =
            (value & Self::IMAGE_FILE_NET_RUN_FROM_SWAP) == Self::IMAGE_FILE_NET_RUN_FROM_SWAP;
        let system = (value & Self::IMAGE_FILE_SYSTEM) == Self::IMAGE_FILE_SYSTEM;
        let dynamic_link_library = (value & Self::IMAGE_FILE_DLL) == Self::IMAGE_FILE_DLL;
        let up_system_only =
            (value & Self::IMAGE_FILE_UP_SYSTEM_ONLY) == Self::IMAGE_FILE_UP_SYSTEM_ONLY;
        let bytes_reserved_hi =
            (value & Self::IMAGE_FILE_BYTES_REVERSED_HI) == Self::IMAGE_FILE_BYTES_REVERSED_HI;

        Self {
            relocs_stripped,
            executable,
            line_nums_stripped,
            local_syms_stripped,
            aggressive_ws_trim,
            large_address_aware,
            reserved0,
            bytes_reserved_lo,
            machine_32_bit,
            debug_stripped,
            removable_run_from_swap,
            net_run_from_swap,
            system,
            dynamic_link_library,
            up_system_only,
            bytes_reserved_hi,
        }
    }
}

impl Binary for Characteristics {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:016b}", self.to_bits()))
    }
}
