//! This module is responsible for parsing COFF header

use chrono::NaiveDateTime;
use pexp::ReadArray;
use std::fmt::Display;
use std::io::Read;

/// COFF File Header structure
#[derive(Debug, PartialEq)]
struct FileHeader {
    /// Identifies the type of target machine. For more information, see [`machine_types`](crate::header::machine_types).
    machine: [u8; 2],
    /// Indicates the size of the section table, which immediately follows the headers.
    number_of_sections: [u8; 2],
    /// The low 32 bits of the number of seconds since 00:00 January 1, 1970 (a C run-time time_t value), which indicates when the file was created.
    time_date_stamp: [u8; 4],
    /// The file offset of the COFF symbol table, or zero if no COFF symbol table is present.
    /// This value should be zero for an image because COFF debugging information is deprecated.
    pointer_to_symbol_table: [u8; 4],
    /// The number of entries in the symbol table.
    /// This data can be used to locate the string table, which immediately follows the symbol table.
    /// This value should be zero for an image because COFF debugging information is deprecated.
    number_of_symbols: [u8; 4],
    /// The size of the [`OptionalHeader`](crate::header::optional_header::OptionalHeader), which is required for executable files but not for object files.
    /// This value should be zero for an object file.
    size_of_optional_header: [u8; 2],
    /// The flags that indicate the attributes of the file. For specific flag values, see [`characteristics`](crate::header::characteristics)
    characteristics: [u8; 2],
}

impl FileHeader {
    fn read_from<R: Read>(reader: &mut R) -> Self {
        let machine = Self::read_array(reader);
        let number_of_sections = Self::read_array(reader);
        let time_date_stamp = Self::read_array(reader);
        let pointer_to_symbol_table = Self::read_array(reader);
        let number_of_symbols = Self::read_array(reader);
        let size_of_optional_header = Self::read_array(reader);
        let characteristics = Self::read_array(reader);
        Self {
            machine,
            number_of_sections,
            time_date_stamp,
            pointer_to_symbol_table,
            number_of_symbols,
            size_of_optional_header,
            characteristics,
        }
    }
}

impl ReadArray for FileHeader {}

struct FileHeaderParser {
    inner_header: FileHeader,
}

impl FileHeaderParser {
    pub fn new(inner_header: FileHeader) -> Self {
        Self { inner_header }
    }

    pub fn machine(&self) -> u16 {
        u16::from_le_bytes(self.inner_header.machine)
    }

    pub fn number_of_sections(&self) -> u16 {
        u16::from_le_bytes(self.inner_header.number_of_sections)
    }

    pub fn time_date_stamp(&self) -> u32 {
        u32::from_le_bytes(self.inner_header.time_date_stamp)
    }

    pub fn pointer_to_symbol_table(&self) -> u32 {
        u32::from_le_bytes(self.inner_header.pointer_to_symbol_table)
    }

    pub fn number_of_symbols(&self) -> u32 {
        u32::from_le_bytes(self.inner_header.number_of_symbols)
    }

    pub fn size_of_optional_header(&self) -> u16 {
        u16::from_le_bytes(self.inner_header.size_of_optional_header)
    }

    pub fn characteristics(&self) -> u16 {
        u16::from_le_bytes(self.inner_header.characteristics)
    }
}

struct FileHeaderAnalyzer {
    inner_parser: FileHeaderParser,
}

impl FileHeaderAnalyzer {
    pub fn new(inner_parser: FileHeaderParser) -> Self {
        Self { inner_parser }
    }

    pub fn machine(&self) -> Machine {
        Machine::from(self.inner_parser.machine())
    }

    pub fn time_date_stamp(&self) -> NaiveDateTime {
        NaiveDateTime::from_timestamp(self.inner_parser.time_date_stamp() as i64, 0)
    }

    pub fn characteristics(&self) -> Characteristics {
        Characteristics::from(self.inner_parser.characteristics())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Machine {
    Unknown,
    Alpha,
    Alpha64,
    AM33,
    AMD64,
    ARM,
    ARM64,
    ARMNT,
    EBC,
    I386,
    IA64,
    LoongArch,
    LoongArch64,
    M32R,
    MIPS16,
    MIPSFPU,
    MIPSFPU16,
    PowerPC,
    PowerPCFP,
    R4000,
    RISCV32,
    RISCV64,
    RISCV128,
    SH3,
    SH3DSP,
    SH4,
    SH5,
    Thumb,
    WCEMIPSV2,
}

impl Machine {
    /// The content of this field is assumed to be applicable to any machine type
    pub const IMAGE_FILE_MACHINE_UNKNOWN: u16 = 0;
    // Alpha AXP, 32-bit address space
    pub const IMAGE_FILE_MACHINE_ALPHA: u16 = 0x0184;
    // Alpha 64, 64-bit address space
    pub const IMAGE_FILE_MACHINE_ALPHA64: u16 = 0x0284;
    /// Matsushita AM33
    pub const IMAGE_FILE_MACHINE_AM33: u16 = 0x01D3;
    /// x64
    pub const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664;
    /// ARM little endian
    pub const IMAGE_FILE_MACHINE_ARM: u16 = 0x01C0;
    /// ARM64 little endian
    pub const IMAGE_FILE_MACHINE_ARM64: u16 = 0xAA64;
    /// ARM Thumb-2 little endian
    pub const IMAGE_FILE_MACHINE_ARMNT: u16 = 0x01C4;
    /// EFI byte code
    pub const IMAGE_FILE_MACHINE_EBC: u16 = 0x0EBC;
    /// Intel 386 or later processors and compatible processors
    pub const IMAGE_FILE_MACHINE_I386: u16 = 0x014C;
    /// Intel Itanium processor family
    pub const IMAGE_FILE_MACHINE_IA64: u16 = 0x0200;
    /// LoongArch 32-bit processor family
    pub const IMAGE_FILE_MACHINE_LOONGARCH32: u16 = 0x6232;
    /// LoongArch 64-bit processor family
    pub const IMAGE_FILE_MACHINE_LOONGARCH64: u16 = 0x6264;
    /// Mitsubishi M32R little endian
    pub const IMAGE_FILE_MACHINE_M32R: u16 = 0x9041;
    /// MIPS16
    pub const IMAGE_FILE_MACHINE_MIPS16: u16 = 0x0266;
    /// MIPS with FPU
    pub const IMAGE_FILE_MACHINE_MIPSFPU: u16 = 0x0366;
    /// MIPS16 with FPU
    pub const IMAGE_FILE_MACHINE_MIPSFPU16: u16 = 0x0466;
    /// Power PC little endian
    pub const IMAGE_FILE_MACHINE_POWERPC: u16 = 0x01F0;
    /// Power PC with floating point support
    pub const IMAGE_FILE_MACHINE_POWERPCFP: u16 = 0x01F1;
    /// MIPS little endian
    pub const IMAGE_FILE_MACHINE_R4000: u16 = 0x0166;
    /// RISC-V 32-bit address space
    pub const IMAGE_FILE_MACHINE_RISCV32: u16 = 0x5032;
    /// RISC-V 64-bit address space
    pub const IMAGE_FILE_MACHINE_RISCV64: u16 = 0x5064;
    /// RISC-V 128-bit address space
    pub const IMAGE_FILE_MACHINE_RISCV128: u16 = 0x5128;
    /// Hitachi SH3
    pub const IMAGE_FILE_MACHINE_SH3: u16 = 0x01A2;
    /// Hitachi SH3 DSP
    pub const IMAGE_FILE_MACHINE_SH3DSP: u16 = 0x01A3;
    /// Hitachi SH4
    pub const IMAGE_FILE_MACHINE_SH4: u16 = 0x01A6;
    /// Hitachi SH5
    pub const IMAGE_FILE_MACHINE_SH5: u16 = 0x01A8;
    /// Thumb
    pub const IMAGE_FILE_MACHINE_THUMB: u16 = 0x01C2;
    /// MIPS little-endian WCE v2
    pub const IMAGE_FILE_MACHINE_WCEMIPSV2: u16 = 0x0169;

    pub const MACHINE_TYPES: [u16; 29] = [
        Self::IMAGE_FILE_MACHINE_UNKNOWN,
        Self::IMAGE_FILE_MACHINE_ALPHA,
        Self::IMAGE_FILE_MACHINE_ALPHA64,
        Self::IMAGE_FILE_MACHINE_AM33,
        Self::IMAGE_FILE_MACHINE_AMD64,
        Self::IMAGE_FILE_MACHINE_ARM,
        Self::IMAGE_FILE_MACHINE_ARM64,
        Self::IMAGE_FILE_MACHINE_ARMNT,
        Self::IMAGE_FILE_MACHINE_EBC,
        Self::IMAGE_FILE_MACHINE_I386,
        Self::IMAGE_FILE_MACHINE_IA64,
        Self::IMAGE_FILE_MACHINE_LOONGARCH32,
        Self::IMAGE_FILE_MACHINE_LOONGARCH64,
        Self::IMAGE_FILE_MACHINE_M32R,
        Self::IMAGE_FILE_MACHINE_MIPS16,
        Self::IMAGE_FILE_MACHINE_MIPSFPU,
        Self::IMAGE_FILE_MACHINE_MIPSFPU16,
        Self::IMAGE_FILE_MACHINE_POWERPC,
        Self::IMAGE_FILE_MACHINE_POWERPCFP,
        Self::IMAGE_FILE_MACHINE_R4000,
        Self::IMAGE_FILE_MACHINE_RISCV32,
        Self::IMAGE_FILE_MACHINE_RISCV64,
        Self::IMAGE_FILE_MACHINE_RISCV128,
        Self::IMAGE_FILE_MACHINE_SH3,
        Self::IMAGE_FILE_MACHINE_SH3DSP,
        Self::IMAGE_FILE_MACHINE_SH4,
        Self::IMAGE_FILE_MACHINE_SH5,
        Self::IMAGE_FILE_MACHINE_THUMB,
        Self::IMAGE_FILE_MACHINE_WCEMIPSV2,
    ];
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = match self {
            Self::Unknown => "Any machine type",
            Self::Alpha => "Alpha AXP, 32-bit address space",
            Self::Alpha64 => "Alpha 64, 64-bit address space",
            Self::AM33 => "Matsushita AM33",
            Self::AMD64 => "x64",
            Self::ARM => "ARM little endian",
            Self::ARM64 => "ARM64 little endian",
            Self::ARMNT => "ARM Thumb-2 little endian",
            Self::EBC => "EFI byte code",
            Self::I386 => "Intel 386 or later processors and compatible processors",
            Self::IA64 => "Intel Itanium processor family",
            Self::LoongArch => "LoongArch 32-bit processor family",
            Self::LoongArch64 => "LoongArch 64-bit processor family",
            Self::M32R => "Mitsubishi M32R little endian",
            Self::MIPS16 => "MIPS16",
            Self::MIPSFPU => "MIPS with FPU",
            Self::MIPSFPU16 => "MIPS16 with FPU",
            Self::PowerPC => "Power PC little endian",
            Self::PowerPCFP => "Power PC with floating point support",
            Self::R4000 => "MIPS little endian",
            Self::RISCV32 => "RISC-V 32-bit address space",
            Self::RISCV64 => "RISC-V 64-bit address space",
            Self::RISCV128 => "RISC-V 128-bit address space",
            Self::SH3 => "Hitachi SH3",
            Self::SH3DSP => "Hitachi SH3 DSP",
            Self::SH4 => "Hitachi SH4",
            Self::SH5 => "Hitachi SH5",
            Self::Thumb => "Thumb",
            Self::WCEMIPSV2 => "MIPS little-endian WCE v2",
        };
        f.write_str(data)
    }
}

impl From<u16> for Machine {
    fn from(value: u16) -> Self {
        match value {
            0x0184 => Self::Alpha,
            0x0284 => Self::Alpha64,
            0x01D3 => Self::AM33,
            0x8664 => Self::AMD64,
            0x01C0 => Self::ARM,
            0xAA64 => Self::ARM64,
            0x01C4 => Self::ARMNT,
            0x0EBC => Self::EBC,
            0x014C => Self::I386,
            0x0200 => Self::IA64,
            0x6232 => Self::LoongArch,
            0x6264 => Self::LoongArch64,
            0x9041 => Self::M32R,
            0x0266 => Self::MIPS16,
            0x0366 => Self::MIPSFPU,
            0x0466 => Self::MIPSFPU16,
            0x01F0 => Self::PowerPC,
            0x01F1 => Self::PowerPCFP,
            0x0166 => Self::R4000,
            0x5032 => Self::RISCV32,
            0x5063 => Self::RISCV64,
            0x5128 => Self::RISCV128,
            0x01A2 => Self::SH3,
            0x01A3 => Self::SH3DSP,
            0x01A6 => Self::SH4,
            0x01A8 => Self::SH5,
            0x01C2 => Self::Thumb,
            0x0169 => Self::WCEMIPSV2,
            _ => Self::Unknown,
        }
    }
}

use std::fmt::{self, Binary, Formatter};

#[derive(Debug, PartialEq)]
pub struct Characteristics {
    flags: [bool; 16],
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
        let mut flags = [false; 16];

        flags[0] = (value & Self::IMAGE_FILE_RELOCS_STRIPPED) == Self::IMAGE_FILE_RELOCS_STRIPPED;
        flags[1] = (value & Self::IMAGE_FILE_EXECUTABLE_IMAGE) == Self::IMAGE_FILE_EXECUTABLE_IMAGE;
        flags[2] =
            (value & Self::IMAGE_FILE_LINE_NUMS_STRIPPED) == Self::IMAGE_FILE_LINE_NUMS_STRIPPED;
        flags[3] =
            (value & Self::IMAGE_FILE_LOCAL_SYMS_STRIPPED) == Self::IMAGE_FILE_LOCAL_SYMS_STRIPPED;
        flags[4] =
            (value & Self::IMAGE_FILE_AGGRESSIVE_WS_TRIM) == Self::IMAGE_FILE_AGGRESSIVE_WS_TRIM;
        flags[5] =
            value & Self::IMAGE_FILE_LARGE_ADDRESS_AWARE == Self::IMAGE_FILE_LARGE_ADDRESS_AWARE;
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
