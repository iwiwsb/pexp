pub const IMAGE_FILE_RELOCS_STRIPPED: [u8; 2] = [0x01, 0x00]; // Image only, Windows CE, and Microsoft Windows NT and later. This indicates that the file does not contain base relocations and must therefore be loaded at its preferred base address. If the base address is not available, the loader reports an error. The default behavior of the linker is to strip base relocations from executable (EXE) files.
pub const IMAGE_FILE_EXECUTABLE_IMAGE: [u8; 2] = [0x02, 0x00]; // Image only. This indicates that the image file is valid and can be run. If this flag is not set, it indicates a linker error.
pub const IMAGE_FILE_LINE_NUMS_STRIPPED: [u8; 2] = [0x04, 0x00]; // COFF line numbers have been removed. This flag is deprecated and should be zero.
pub const IMAGE_FILE_LOCAL_SYMS_STRIPPED: [u8; 2] = [0x08, 0x00]; // COFF symbol table entries for local symbols have been removed. This flag is deprecated and should be zero.
pub const IMAGE_FILE_AGGRESSIVE_WS_TRIM: [u8; 2] = [0x10, 0x00]; // Obsolete. Aggressively trim working set. This flag is deprecated for Windows 2000 and later and must be zero.
pub const IMAGE_FILE_LARGE_ADDRESS_AWARE: [u8; 2] = [0x20, 0x00]; // Application can handle > 2-GB addresses.
pub const IMAGE_FILE_RESERVED0: [u8; 2] = [0x40, 0x00]; // This flag is reserved for future use.
pub const IMAGE_FILE_BYTES_REVERSED_LO: [u8; 2] = [0x80, 0x00]; // Little endian: the least significant bit (LSB) precedes the most significant bit (MSB) in memory. This flag is deprecated and should be zero.
pub const IMAGE_FILE_32BIT_MACHINE: [u8; 2] = [0x00, 0x01]; // Machine is based on a 32-bit-word architecture.
pub const IMAGE_FILE_DEBUG_STRIPPED: [u8; 2] = [0x00, 0x02]; // Debugging information is removed from the image file.
pub const IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP: [u8; 2] = [0x00, 0x04]; // If the image is on removable media, fully load it and copy it to the swap file.
pub const IMAGE_FILE_NET_RUN_FROM_SWAP: [u8; 2] = [0x00, 0x08]; // If the image is on network media, fully load it and copy it to the swap file.
pub const IMAGE_FILE_SYSTEM: [u8; 2] = [0x00, 0x10]; // The image file is a system file, not a user program.
pub const IMAGE_FILE_DLL: [u8; 2] = [0x00, 0x20]; // The image file is a dynamic-link library (DLL). Such files are considered executable files for almost all purposes, although they cannot be directly run.
pub const IMAGE_FILE_UP_SYSTEM_ONLY: [u8; 2] = [0x00, 0x40]; // The file should be run only on a uniprocessor machine.
pub const IMAGE_FILE_BYTES_REVERSED_HI: [u8; 2] = [0x00, 0x80]; // Big endian: the MSB precedes the LSB in memory. This flag is deprecated and should be zero.

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
    pub _32_bit_machine: bool,
    pub debug_stripped: bool,
    pub removable_run_from_swap: bool,
    pub net_run_from_swap: bool,
    pub system: bool,
    pub dll: bool,
    pub up_system_only: bool,
    pub bytes_reserved_hi: bool,
}

impl From<u16> for Characteristics {
    fn from(value: u16) -> Self {
        let relocs_stripped = value & u16::from_le_bytes(IMAGE_FILE_RELOCS_STRIPPED) != 0;
        let executable = value & u16::from_le_bytes(IMAGE_FILE_EXECUTABLE_IMAGE) != 0;
        let line_nums_stripped = value & u16::from_le_bytes(IMAGE_FILE_LINE_NUMS_STRIPPED) != 0;
        let local_syms_stripped = value & u16::from_le_bytes(IMAGE_FILE_LOCAL_SYMS_STRIPPED) != 0;
        let aggressive_ws_trim = value & u16::from_le_bytes(IMAGE_FILE_AGGRESSIVE_WS_TRIM) != 0;
        let large_address_aware = value & u16::from_le_bytes(IMAGE_FILE_LARGE_ADDRESS_AWARE) != 0;
        let reserved0 = value & u16::from_le_bytes(IMAGE_FILE_RESERVED0) != 0;
        let bytes_reserved_lo = value & u16::from_le_bytes(IMAGE_FILE_BYTES_REVERSED_LO) != 0;
        let _32_bit_machine = value & u16::from_le_bytes(IMAGE_FILE_32BIT_MACHINE) != 0;
        let debug_stripped = value & u16::from_le_bytes(IMAGE_FILE_DEBUG_STRIPPED) != 0;
        let removable_run_from_swap =
            value & u16::from_le_bytes(IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP) != 0;
        let net_run_from_swap = value & u16::from_le_bytes(IMAGE_FILE_NET_RUN_FROM_SWAP) != 0;
        let system = value & u16::from_le_bytes(IMAGE_FILE_SYSTEM) != 0;
        let dll = value & u16::from_le_bytes(IMAGE_FILE_DLL) != 0;
        let up_system_only = value & u16::from_le_bytes(IMAGE_FILE_UP_SYSTEM_ONLY) != 0;
        let bytes_reserved_hi = value & u16::from_le_bytes(IMAGE_FILE_BYTES_REVERSED_HI) != 0;

        Self {
            relocs_stripped,
            executable,
            line_nums_stripped,
            local_syms_stripped,
            aggressive_ws_trim,
            large_address_aware,
            reserved0,
            bytes_reserved_lo,
            _32_bit_machine,
            debug_stripped,
            removable_run_from_swap,
            net_run_from_swap,
            system,
            dll,
            up_system_only,
            bytes_reserved_hi,
        }
    }
}

impl From<[u8; 2]> for Characteristics {
    fn from(value: [u8; 2]) -> Self {
        let _u16 = u16::from_le_bytes(value);
        Self::from(_u16)
    }
}

#[cfg(test)]
mod tests {
    use super::Characteristics;

    #[test]
    fn test_executable_large_addr() {
        let left = Characteristics {
            relocs_stripped: false,
            executable: true,
            line_nums_stripped: false,
            local_syms_stripped: false,
            aggressive_ws_trim: false,
            large_address_aware: true,
            reserved0: false,
            bytes_reserved_lo: false,
            _32_bit_machine: false,
            debug_stripped: false,
            removable_run_from_swap: false,
            net_run_from_swap: false,
            system: false,
            dll: false,
            up_system_only: false,
            bytes_reserved_hi: false,
        };
        let right = Characteristics::from(0x22);
        assert_eq!(left, right);
    }
}
