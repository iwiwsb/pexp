//! The `machine` field has one of the following values, which specify the CPU type.
//! An image file can be run only on the specified machine or on a system that emulates the specified machine.

use std::fmt::Display;

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
    IMAGE_FILE_MACHINE_UNKNOWN,
    IMAGE_FILE_MACHINE_ALPHA,
    IMAGE_FILE_MACHINE_ALPHA64,
    IMAGE_FILE_MACHINE_AM33,
    IMAGE_FILE_MACHINE_AMD64,
    IMAGE_FILE_MACHINE_ARM,
    IMAGE_FILE_MACHINE_ARM64,
    IMAGE_FILE_MACHINE_ARMNT,
    IMAGE_FILE_MACHINE_EBC,
    IMAGE_FILE_MACHINE_I386,
    IMAGE_FILE_MACHINE_IA64,
    IMAGE_FILE_MACHINE_LOONGARCH32,
    IMAGE_FILE_MACHINE_LOONGARCH64,
    IMAGE_FILE_MACHINE_M32R,
    IMAGE_FILE_MACHINE_MIPS16,
    IMAGE_FILE_MACHINE_MIPSFPU,
    IMAGE_FILE_MACHINE_MIPSFPU16,
    IMAGE_FILE_MACHINE_POWERPC,
    IMAGE_FILE_MACHINE_POWERPCFP,
    IMAGE_FILE_MACHINE_R4000,
    IMAGE_FILE_MACHINE_RISCV32,
    IMAGE_FILE_MACHINE_RISCV64,
    IMAGE_FILE_MACHINE_RISCV128,
    IMAGE_FILE_MACHINE_SH3,
    IMAGE_FILE_MACHINE_SH3DSP,
    IMAGE_FILE_MACHINE_SH4,
    IMAGE_FILE_MACHINE_SH5,
    IMAGE_FILE_MACHINE_THUMB,
    IMAGE_FILE_MACHINE_WCEMIPSV2,
];

#[repr(u16)]
#[derive(Debug, Clone, PartialEq)]
pub enum Machine {
    Unknown = IMAGE_FILE_MACHINE_UNKNOWN,
    Alpha = IMAGE_FILE_MACHINE_ALPHA,
    Aplpha64 = IMAGE_FILE_MACHINE_ALPHA64,
    AM33 = IMAGE_FILE_MACHINE_AM33,
    AMD64 = IMAGE_FILE_MACHINE_AMD64,
    ARM = IMAGE_FILE_MACHINE_ARM,
    ARM64 = IMAGE_FILE_MACHINE_ARM64,
    ARMNT = IMAGE_FILE_MACHINE_ARMNT,
    EBC = IMAGE_FILE_MACHINE_EBC,
    I386 = IMAGE_FILE_MACHINE_I386,
    IA64 = IMAGE_FILE_MACHINE_IA64,
    LoongArch = IMAGE_FILE_MACHINE_LOONGARCH32,
    LoongArch64 = IMAGE_FILE_MACHINE_LOONGARCH64,
    M32R = IMAGE_FILE_MACHINE_M32R,
    MIPS16 = IMAGE_FILE_MACHINE_MIPS16,
    MIPSFPU = IMAGE_FILE_MACHINE_MIPSFPU,
    MIPSFPU16 = IMAGE_FILE_MACHINE_MIPSFPU16,
    PowerPC = IMAGE_FILE_MACHINE_POWERPC,
    PowerPCFP = IMAGE_FILE_MACHINE_POWERPCFP,
    R4000 = IMAGE_FILE_MACHINE_R4000,
    RISCV32 = IMAGE_FILE_MACHINE_RISCV32,
    RISCV64 = IMAGE_FILE_MACHINE_RISCV64,
    RISCV128 = IMAGE_FILE_MACHINE_RISCV128,
    SH3 = IMAGE_FILE_MACHINE_SH3,
    SH3DSP = IMAGE_FILE_MACHINE_SH3DSP,
    SH4 = IMAGE_FILE_MACHINE_SH4,
    SH5 = IMAGE_FILE_MACHINE_SH5,
    Thumb = IMAGE_FILE_MACHINE_THUMB,
    WCEMIPSV2 = IMAGE_FILE_MACHINE_WCEMIPSV2,
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = match self {
            Self::Unknown => "Any machine type",
            Self::Alpha => "Alpha AXP, 32-bit address space",
            Self::Aplpha64 => "Alpha 64, 64-bit address space",
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
            0x0284 => Self::Aplpha64,
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
