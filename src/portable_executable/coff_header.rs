use chrono::{DateTime, Utc, TimeZone};

use super::Characteristics;

#[allow(non_camel_case_types)]
pub struct COFFFileHeader {
    machine: MachineType,
    number_of_sections: u16,
    time_date_stamp: DateTime<Utc>,
    pointer_to_symbol_table: Option<u32>,
    number_of_symbols: u32,
    size_of_optional_header: u16,
    characteristics: u16,
}

impl TryFrom<[u8; 20]> for COFFFileHeader {
    type Error = &'static str;
    fn try_from(buffer: [u8; 20]) -> Result<Self, Self::Error> {
        let bytes_0_1 = [buffer[0], buffer[1]];
        let bytes_2_3 = [buffer[2], buffer[3]];
        let bytes_4_7 = [buffer[4], buffer[5], buffer[6], buffer[7]];
        let bytes_8_11 = [buffer[8], buffer[9], buffer[10], buffer[11]];
        let bytes_12_15 = [buffer[12], buffer[13], buffer[14], buffer[15]];
        let bytes_16_17 = [buffer[16], buffer[17]];
        let bytes_18_19 = [buffer[18], buffer[19]];

        let machine = MachineType::try_from(bytes_0_1)?;
        let number_of_sections = u16::from_le_bytes(bytes_2_3);
        if number_of_sections > 96 {
            return Err("number of sections must not exceed 96");
        }
        let time_date_stamp = Utc.timestamp(u32::from_le_bytes(bytes_4_7) as i64, 0);
        let pointer_to_symbol_table = match bytes_8_11 {
            [0, 0, 0, 0] => None,
            _ => Some(u32::from_le_bytes(bytes_8_11)),
        };
        let number_of_symbols = u32::from_le_bytes(bytes_12_15);
        let size_of_optional_header = u16::from_le_bytes(bytes_16_17);
        let characteristics = u16::from_le_bytes(bytes_18_19);
        Ok(Self {
            machine,
            number_of_sections,
            time_date_stamp,
            pointer_to_symbol_table,
            number_of_symbols,
            size_of_optional_header,
            characteristics,
        })
    }
}

#[allow(non_camel_case_types)]
enum MachineType {
    UNKNOWN = 0x0000, // The content of this field is assumed to be applicable to any machine type
    AM33 = 0x01D3,    // Matsushita AM33
    AMD64 = 0x8664,   // x64
    ARM = 0x01C0,     // ARM little endian
    ARM64 = 0xAA64,   // ARM64 little endian
    ARMNT = 0x01C4,   // ARM Thumb-2 little endian
    EBC = 0x0EBC,     // EFI byte code
    I386 = 0x014C,    // Intel 386 or later processors and compatible processors
    IA64 = 0x0200,    // Intel Itanium processor family
    LOONGARCH32 = 0x6232, // LoongArch 32-bit processor family
    LOONGARCH64 = 0x6264, // LoongArch 64-bit processor family
    M32R = 0x9041,    // Mitsubishi M32R little endian
    MIPS16 = 0x0266,  // MIPS16
    MIPSFPU = 0x0366, // MIPS with FPU
    MIPSFPU16 = 0x0466, //MIPS16 with FPU
    POWERPC = 0x01F0, // Power PC little endian
    POWERPCFP = 0x01F1, // Power PC with floating point support
    R4000 = 0x0166,   //MIPS little endian
    RISCV32 = 0x5032, //RISC-V 32-bit address space
    RISCV64 = 0x5064, //RISC-V 64-bit address space
    RISCV128 = 0x5128, //RISC-V 128-bit address space
    SH3 = 0x01A2,     // Hitachi SH3
    SH3DSP = 0x01A3,  // Hitachi SH3 DSP
    SH4 = 0x01A6,     // Hitachi SH4
    SH5 = 0x01A8,     //Hitachi SH5
    THUMB = 0x01C2,   //Thumb
    WCEMIPSV2 = 0x0169, //MIPS little-endian WCE v2
}

impl TryFrom<[u8; 2]> for MachineType {
    type Error = &'static str;

    fn try_from(value: [u8; 2]) -> Result<Self, Self::Error> {
        match value {
            [0x00, 0x00] => Ok(Self::UNKNOWN),
            [0xD3, 0x01] => Ok(Self::AM33),
            [0x64, 0x86] => Ok(Self::AMD64),
            [0xC0, 0x01] => Ok(Self::ARM),
            [0x64, 0xAA] => Ok(Self::ARM64),
            [0xC4, 0x01] => Ok(Self::ARMNT),
            [0xBC, 0x0E] => Ok(Self::EBC),
            [0x4C, 0x01] => Ok(Self::I386),
            [0x00, 0x02] => Ok(Self::IA64),
            [0x32, 0x62] => Ok(Self::LOONGARCH32),
            [0x64, 0x62] => Ok(Self::LOONGARCH64),
            [0x41, 0x90] => Ok(Self::M32R),
            [0x66, 0x02] => Ok(Self::MIPS16),
            [0x66, 0x03] => Ok(Self::MIPSFPU),
            [0x66, 0x04] => Ok(Self::MIPSFPU16),
            [0xF0, 0x01] => Ok(Self::POWERPC),
            [0xF1, 0x01] => Ok(Self::POWERPCFP),
            [0x66, 0x01] => Ok(Self::R4000),
            [0x32, 0x50] => Ok(Self::RISCV32),
            [0x64, 0x50] => Ok(Self::RISCV64),
            [0x28, 0x51] => Ok(Self::RISCV128),
            [0xA2, 0x01] => Ok(Self::SH3),
            [0xA3, 0x01] => Ok(Self::SH3DSP),
            [0xA6, 0x01] => Ok(Self::SH4),
            [0xA8, 0x01] => Ok(Self::SH5),
            [0xC2, 0x01] => Ok(Self::THUMB),
            [0x69, 0x01] => Ok(Self::WCEMIPSV2),
            _ => Err("invalid machine type"),
        }
    }
}
