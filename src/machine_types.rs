//! The [machine](crate::header::FileHeader#structfield.machine) field has one of the following values, which specify the CPU type.
//! An image file can be run only on the specified machine or on a system that emulates the specified machine.

use std::fmt::Display;

/// The content of this field is assumed to be applicable to any machine type
pub const IMAGE_FILE_MACHINE_UNKNOWN: [u8; 2] = [0x00, 0x00];
/// Matsushita AM33
pub const IMAGE_FILE_MACHINE_AM33: [u8; 2] = [0xD3, 0x01];
/// x64
pub const IMAGE_FILE_MACHINE_AMD64: [u8; 2] = [0x64, 0x86];
/// ARM little endian
pub const IMAGE_FILE_MACHINE_ARM: [u8; 2] = [0xC0, 0x01];
/// ARM64 little endian
pub const IMAGE_FILE_MACHINE_ARM64: [u8; 2] = [0x64, 0xAA];
/// ARM Thumb-2 little endian
pub const IMAGE_FILE_MACHINE_ARMNT: [u8; 2] = [0xC4, 0x01];
/// EFI byte code
pub const IMAGE_FILE_MACHINE_EBC: [u8; 2] = [0xBC, 0x0E];
/// Intel 386 or later processors and compatible processors
pub const IMAGE_FILE_MACHINE_I386: [u8; 2] = [0x4C, 0x01];
/// Intel Itanium processor family
pub const IMAGE_FILE_MACHINE_IA64: [u8; 2] = [0x00, 0x02];
/// LoongArch 32-bit processor family
pub const IMAGE_FILE_MACHINE_LOONGARCH32: [u8; 2] = [0x32, 0x62];
/// LoongArch 64-bit processor family
pub const IMAGE_FILE_MACHINE_LOONGARCH64: [u8; 2] = [0x64, 0x62];
/// Mitsubishi M32R little endian
pub const IMAGE_FILE_MACHINE_M32R: [u8; 2] = [0x41, 0x90];
/// MIPS16
pub const IMAGE_FILE_MACHINE_MIPS16: [u8; 2] = [0x66, 0x02];
/// MIPS with FPU
pub const IMAGE_FILE_MACHINE_MIPSFPU: [u8; 2] = [0x66, 0x03];
/// MIPS16 with FPU
pub const IMAGE_FILE_MACHINE_MIPSFPU16: [u8; 2] = [0x66, 0x04];
/// Power PC little endian
pub const IMAGE_FILE_MACHINE_POWERPC: [u8; 2] = [0xF0, 0x01];
/// Power PC with floating point support
pub const IMAGE_FILE_MACHINE_POWERPCFP: [u8; 2] = [0xF1, 0x01];
/// MIPS little endian
pub const IMAGE_FILE_MACHINE_R4000: [u8; 2] = [0x66, 0x01];
/// RISC-V 32-bit address space
pub const IMAGE_FILE_MACHINE_RISCV32: [u8; 2] = [0x32, 0x50];
/// RISC-V 64-bit address space
pub const IMAGE_FILE_MACHINE_RISCV64: [u8; 2] = [0x64, 0x50];
/// RISC-V 128-bit address space
pub const IMAGE_FILE_MACHINE_RISCV128: [u8; 2] = [0x28, 0x51];
/// Hitachi SH3
pub const IMAGE_FILE_MACHINE_SH3: [u8; 2] = [0xA2, 0x01];
/// Hitachi SH3 DSP
pub const IMAGE_FILE_MACHINE_SH3DSP: [u8; 2] = [0xA3, 0x01];
/// Hitachi SH4
pub const IMAGE_FILE_MACHINE_SH4: [u8; 2] = [0xA6, 0x01];
/// Hitachi SH5
pub const IMAGE_FILE_MACHINE_SH5: [u8; 2] = [0xA8, 0x01];
/// Thumb
pub const IMAGE_FILE_MACHINE_THUMB: [u8; 2] = [0xC2, 0x01];
/// MIPS little-endian WCE v2
pub const IMAGE_FILE_MACHINE_WCEMIPSV2: [u8; 2] = [0x69, 0x01];

pub struct Machine([u8; 2]);

impl From<[u8; 2]> for Machine {
    fn from(value: [u8; 2]) -> Self {
        Self(value)
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = match self.0 {
            IMAGE_FILE_MACHINE_UNKNOWN => "Any machine type",
            IMAGE_FILE_MACHINE_AM33 => "Matsushita AM33",
            IMAGE_FILE_MACHINE_AMD64 => "x64",
            IMAGE_FILE_MACHINE_ARM => "ARM little endian",
            IMAGE_FILE_MACHINE_ARM64 => "ARM64 little endian",
            IMAGE_FILE_MACHINE_ARMNT => "ARM Thumb-2 little endian",
            IMAGE_FILE_MACHINE_EBC => "EFI byte code",
            IMAGE_FILE_MACHINE_I386 => "Intel 386 or later processors and compatible processors",
            IMAGE_FILE_MACHINE_IA64 => "Intel Itanium processor family",
            IMAGE_FILE_MACHINE_LOONGARCH32 => "LoongArch 32-bit processor family",
            IMAGE_FILE_MACHINE_LOONGARCH64 => "LoongArch 64-bit processor family",
            IMAGE_FILE_MACHINE_M32R => "Mitsubishi M32R little endian",
            IMAGE_FILE_MACHINE_MIPS16 => "MIPS16",
            IMAGE_FILE_MACHINE_MIPSFPU => "MIPS with FPU",
            IMAGE_FILE_MACHINE_MIPSFPU16 => "MIPS16 with FPU",
            IMAGE_FILE_MACHINE_POWERPC => "Power PC little endian",
            IMAGE_FILE_MACHINE_POWERPCFP => "Power PC with floating point support",
            IMAGE_FILE_MACHINE_R4000 => "MIPS little endian",
            IMAGE_FILE_MACHINE_RISCV32 => "RISC-V 32-bit address space",
            IMAGE_FILE_MACHINE_RISCV64 => "RISC-V 64-bit address space",
            IMAGE_FILE_MACHINE_RISCV128 => "RISC-V 128-bit address space",
            IMAGE_FILE_MACHINE_SH3 => "Hitachi SH3",
            IMAGE_FILE_MACHINE_SH3DSP => "Hitachi SH3 DSP",
            IMAGE_FILE_MACHINE_SH4 => "Hitachi SH4",
            IMAGE_FILE_MACHINE_SH5 => "Hitachi SH5",
            IMAGE_FILE_MACHINE_THUMB => "Thumb",
            IMAGE_FILE_MACHINE_WCEMIPSV2 => "MIPS little-endian WCE v2",
            _ => return Err(std::fmt::Error),
        };
        f.write_str(data)
    }
}
