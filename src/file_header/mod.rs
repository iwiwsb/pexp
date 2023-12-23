use crate::StructField;
use chrono::NaiveDateTime;
use std::io::{Read, Seek};

fn read_file_header<R: Read + Seek>(reader: &mut R) -> FileHeader {
    let mut machine = [0u8; 2];
    let mut number_of_sections = [0u8; 2];
    let mut time_date_stamp = [0u8; 4];
    let mut pointer_to_symbol_table = [0u8; 4];
    let mut number_of_symbols = [0u8; 4];
    let mut size_of_optional_header = [0u8; 2];
    let mut characteristics = [0u8; 2];

    let _ = reader.read_exact(&mut machine);
    let _ = reader.read_exact(&mut number_of_sections);
    let _ = reader.read_exact(&mut time_date_stamp);
    let _ = reader.read_exact(&mut pointer_to_symbol_table);
    let _ = reader.read_exact(&mut number_of_symbols);
    let _ = reader.read_exact(&mut size_of_optional_header);
    let _ = reader.read_exact(&mut characteristics);

    let file_header_raw = FileHeaderRaw {
        machine,
        number_of_sections,
        time_date_stamp,
        pointer_to_symbol_table,
        number_of_symbols,
        size_of_optional_header,
        characteristics,
    };

    let offset = reader
        .stream_position()
        .expect("Stream position should working");

    FileHeader {
        offset,
        file_header_raw,
    }
}

#[derive(Debug)]
struct FileHeaderRaw {
    machine: [u8; 2],
    number_of_sections: [u8; 2],
    time_date_stamp: [u8; 4],
    pointer_to_symbol_table: [u8; 4],
    number_of_symbols: [u8; 4],
    size_of_optional_header: [u8; 2],
    characteristics: [u8; 2],
}

#[derive(Debug)]
struct FileHeader {
    offset: u64,
    file_header_raw: FileHeaderRaw,
}

impl FileHeader {
    pub fn get_offset(&self) -> u64 {
        self.offset
    }

    pub fn machine(&self) -> u16 {
        u16::from_le_bytes(self.file_header_raw.machine)
    }

    pub fn number_of_sections(&self) -> u16 {
        u16::from_le_bytes(self.file_header_raw.number_of_sections)
    }

    pub fn time_date_stamp(&self) -> u32 {
        u32::from_le_bytes(self.file_header_raw.time_date_stamp)
    }

    pub fn pointer_to_symbol_table(&self) -> u32 {
        u32::from_le_bytes(self.file_header_raw.pointer_to_symbol_table)
    }

    pub fn number_of_symbols(&self) -> u32 {
        u32::from_le_bytes(self.file_header_raw.number_of_symbols)
    }

    pub fn size_of_optional_header(&self) -> u16 {
        u16::from_le_bytes(self.file_header_raw.size_of_optional_header)
    }

    pub fn characteristics(&self) -> u16 {
        u16::from_le_bytes(self.file_header_raw.characteristics)
    }
}

#[derive(Debug)]
pub struct FileHeaderWrapper {
    file_header: FileHeader,
}

impl FileHeaderWrapper {
    fn machine(&self) -> StructField<Machine, 2> {
        let offset = self.file_header.offset;
        let name = String::from("Machine");
        let raw_bytes = self.file_header.file_header_raw.machine;
        todo!()
    }

    fn number_of_sections(&self) -> StructField<u16, 2> {
        let offset = self.file_header.offset + 2;
        let name = String::from("Num. of sections");
        let raw_bytes = self.file_header.file_header_raw.number_of_sections;
        let value = self.file_header.number_of_sections();
        StructField {
            offset,
            name,
            raw_bytes,
            value,
        }
    }

    fn time_date_stamp(&self) -> StructField<NaiveDateTime, 4> {
        let offset = self.file_header.offset + 4;
        let name = String::from("Time date stamp");
        let raw_bytes = self.file_header.file_header_raw.time_date_stamp;
        let value = NaiveDateTime::from_timestamp(self.file_header.time_date_stamp() as i64, 0);
        StructField {
            offset,
            name,
            raw_bytes,
            value,
        }
    }

    fn pointer_to_symbol_table(&self) -> StructField<u32, 4> {
        let offset = self.file_header.offset + 8;
        let name = String::from("Pointer to symbol table");
        let raw_bytes = self.file_header.file_header_raw.pointer_to_symbol_table;
        let value = self.file_header.pointer_to_symbol_table();
        StructField {
            offset,
            name,
            raw_bytes,
            value,
        }
    }

    fn number_of_symbols(&self) -> StructField<u32, 4> {
        let offset = self.file_header.offset + 12;
        let name = String::from("Number of symbols");
        let raw_bytes = self.file_header.file_header_raw.number_of_symbols;
        let value = self.file_header.number_of_symbols();
        StructField {
            offset,
            name,
            raw_bytes,
            value,
        }
    }

    fn size_of_optional_header(&self) -> StructField<u16, 2> {
        let offset = self.file_header.offset + 16;
        let name = String::from("Size of optional header");
        let raw_bytes = self.file_header.file_header_raw.size_of_optional_header;
        let value = self.file_header.size_of_optional_header();
        StructField {
            offset,
            name,
            raw_bytes,
            value,
        }
    }

    fn characteristics(&self) -> StructField<Characteristics, 2> {
        let offset = self.file_header.offset + 18;
        let name = String::from("Characteristics");
        let raw_bytes = self.file_header.file_header_raw.characteristics;
        todo!()
    }
}

enum Machine {}

impl From<u16> for Machine {
    fn from(value: u16) -> Self {
        todo!()
    }
}

/// Alpha AXP, 32-bit address space
const IMAGE_FILE_MACHINE_ALPHA: u16 = 0x0184;
/// Alpha 64, 64-bit address space
const IMAGE_FILE_MACHINE_ALPHA64: u16 = 0x0284;
/// Matsushita AM33
const IMAGE_FILE_MACHINE_AM33: u16 = 0x01D3;
/// x64
const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664;
/// ARM little endian
const IMAGE_FILE_MACHINE_ARM: u16 = 0x01C0;
/// ARM64 little endian
const IMAGE_FILE_MACHINE_ARM64: u16 = 0xaa64;
/// ARM Thumb-2 little endian
const IMAGE_FILE_MACHINE_ARMNT: u16 = 0x01C4;
/// AXP 64 (Same as Alpha 64)
const IMAGE_FILE_MACHINE_AXP64: u16 = 0x0284;
/// EFI byte code
const IMAGE_FILE_MACHINE_EBC: u16 = 0x0EBC;
/// Intel 386 or later processors and compatible processors
const IMAGE_FILE_MACHINE_I386: u16 = 0x014C;
/// Intel Itanium processor family
const IMAGE_FILE_MACHINE_IA64: u16 = 0x0200;
/// LoongArch 32-bit processor family
const IMAGE_FILE_MACHINE_LOONGARCH32: u16 = 0x6232;
/// LoongArch 64-bit processor family
const IMAGE_FILE_MACHINE_LOONGARCH64: u16 = 0x6264;
/// Mitsubishi M32R little endian
const IMAGE_FILE_MACHINE_M32R: u16 = 0x9041;
/// MIPS16
const IMAGE_FILE_MACHINE_MIPS16: u16 = 0x0266;
/// MIPS with FPU
const IMAGE_FILE_MACHINE_MIPSFPU: u16 = 0x0366;
/// MIPS16 with FPU
const IMAGE_FILE_MACHINE_MIPSFPU16: u16 = 0x0466;
/// Power PC little endian
const IMAGE_FILE_MACHINE_POWERPC: u16 = 0x01F0;
/// Power PC with floating point support
const IMAGE_FILE_MACHINE_POWERPCFP: u16 = 0x01F1;
/// MIPS little endian
const IMAGE_FILE_MACHINE_R4000: u16 = 0x0166;
/// RISC-V 32-bit address space
const IMAGE_FILE_MACHINE_RISCV32: u16 = 0x5032;
/// RISC-V 64-bit address space
const IMAGE_FILE_MACHINE_RISCV64: u16 = 0x5064;
/// RISC-V 128-bit address space
const IMAGE_FILE_MACHINE_RISCV128: u16 = 0x5128;
/// Hitachi SH3
const IMAGE_FILE_MACHINE_SH3: u16 = 0x01A2;
/// Hitachi SH3 DSP
const IMAGE_FILE_MACHINE_SH3DSP: u16 = 0x01A3;
/// Hitachi SH4
const IMAGE_FILE_MACHINE_SH4: u16 = 0x01A6;
/// Hitachi SH5
const IMAGE_FILE_MACHINE_SH5: u16 = 0x01A8;
/// Thumb
const IMAGE_FILE_MACHINE_THUMB: u16 = 0x01C2;
/// MIPS little-endian WCE v2
const IMAGE_FILE_MACHINE_WCEMIPSV2: u16 = 0x0169;

const MACHINE: [u16; 29] = [
    IMAGE_FILE_MACHINE_ALPHA,
    IMAGE_FILE_MACHINE_ALPHA64,
    IMAGE_FILE_MACHINE_AM33,
    IMAGE_FILE_MACHINE_AMD64,
    IMAGE_FILE_MACHINE_ARM,
    IMAGE_FILE_MACHINE_ARM64,
    IMAGE_FILE_MACHINE_ARMNT,
    IMAGE_FILE_MACHINE_AXP64,
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

struct Characteristics;
