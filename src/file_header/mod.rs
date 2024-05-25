use crate::StructField;
use chrono::NaiveDateTime;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

pub fn read_file_header<R: Read + Seek>(reader: &mut R, offset: u64) -> FileHeaderWrapper {
    let _ = reader.seek(SeekFrom::Start(offset));

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

    let file_header = FileHeader {
        offset,
        file_header_raw,
    };

    FileHeaderWrapper { file_header }
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
    fn machine(&self) -> u16 {
        u16::from_le_bytes(self.file_header_raw.machine)
    }

    fn number_of_sections(&self) -> u16 {
        u16::from_le_bytes(self.file_header_raw.number_of_sections)
    }

    fn time_date_stamp(&self) -> u32 {
        u32::from_le_bytes(self.file_header_raw.time_date_stamp)
    }

    fn pointer_to_symbol_table(&self) -> u32 {
        u32::from_le_bytes(self.file_header_raw.pointer_to_symbol_table)
    }

    fn number_of_symbols(&self) -> u32 {
        u32::from_le_bytes(self.file_header_raw.number_of_symbols)
    }

    fn size_of_optional_header(&self) -> u16 {
        u16::from_le_bytes(self.file_header_raw.size_of_optional_header)
    }

    fn characteristics(&self) -> u16 {
        u16::from_le_bytes(self.file_header_raw.characteristics)
    }
}

#[derive(Debug)]
pub struct FileHeaderWrapper {
    file_header: FileHeader,
}

impl FileHeaderWrapper {
    pub fn machine(&self) -> StructField<Machine, 2> {
        let offset = self.file_header.offset;
        let name = String::from("Machine");
        let raw_bytes = self.file_header.file_header_raw.machine;
        let value = Machine::from(self.file_header.machine());
        StructField {
            offset,
            name,
            raw_bytes,
            value,
        }
    }

    pub fn number_of_sections(&self) -> StructField<u16, 2> {
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

    pub fn time_date_stamp(&self) -> StructField<NaiveDateTime, 4> {
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

    pub fn pointer_to_symbol_table(&self) -> StructField<u32, 4> {
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

    pub fn number_of_symbols(&self) -> StructField<u32, 4> {
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

    pub fn size_of_optional_header(&self) -> StructField<u16, 2> {
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

    pub fn characteristics(&self) -> StructField<Characteristics, 2> {
        let offset = self.file_header.offset + 18;
        let name = String::from("Characteristics");
        let raw_bytes = self.file_header.file_header_raw.characteristics;
        let value = Characteristics::from(self.file_header.characteristics());
        StructField {
            offset,
            name,
            raw_bytes,
            value,
        }
    }
}

pub enum Machine {
    Unknown,
    AlphaAXP,
    Alpha64,
    MatsushitaAM33,
    X64,
    ARMLittleEndian,
    ARM64LittleEndian,
    ARMThumb2,
    EFIByteCode,
    Intel386,
    Itanium,
    LoongArch32,
    LoongArch64,
    MitsubishiM32R,
    MIPS16,
    MIPSFPU,
    MIPSFPU16,
    PowerPCLE,
    PowerPCFPU,
    MIPSLE,
    RISCV32,
    RISCV64,
    RISCV128,
    HitachiSH3,
    HitachiSH3DSP,
    HitachiSH4,
    HitachiSH5,
    Thumb,
    WCEMIPSV2,
}

impl From<u16> for Machine {
    fn from(value: u16) -> Self {
        match value {
            IMAGE_FILE_MACHINE_ALPHA => Self::AlphaAXP,
            IMAGE_FILE_MACHINE_ALPHA64 => Self::Alpha64,
            IMAGE_FILE_MACHINE_AM33 => Self::MatsushitaAM33,
            IMAGE_FILE_MACHINE_AMD64 => Self::X64,
            IMAGE_FILE_MACHINE_ARM => Self::ARMLittleEndian,
            IMAGE_FILE_MACHINE_ARM64 => Self::ARM64LittleEndian,
            IMAGE_FILE_MACHINE_ARMNT => Self::ARMThumb2,
            IMAGE_FILE_MACHINE_EBC => Self::EFIByteCode,
            IMAGE_FILE_MACHINE_I386 => Self::Intel386,
            IMAGE_FILE_MACHINE_IA64 => Self::Itanium,
            IMAGE_FILE_MACHINE_LOONGARCH32 => Self::LoongArch32,
            IMAGE_FILE_MACHINE_LOONGARCH64 => Self::LoongArch64,
            IMAGE_FILE_MACHINE_M32R => Self::MitsubishiM32R,
            IMAGE_FILE_MACHINE_MIPS16 => Self::MIPS16,
            IMAGE_FILE_MACHINE_MIPSFPU => Self::MIPSFPU,
            IMAGE_FILE_MACHINE_MIPSFPU16 => Self::MIPSFPU16,
            IMAGE_FILE_MACHINE_POWERPC => Self::PowerPCLE,
            IMAGE_FILE_MACHINE_POWERPCFP => Self::PowerPCFPU,
            IMAGE_FILE_MACHINE_R4000 => Self::MIPSLE,
            IMAGE_FILE_MACHINE_RISCV32 => Self::RISCV32,
            IMAGE_FILE_MACHINE_RISCV64 => Self::RISCV64,
            IMAGE_FILE_MACHINE_RISCV128 => Self::RISCV128,
            IMAGE_FILE_MACHINE_SH3 => Self::HitachiSH3,
            IMAGE_FILE_MACHINE_SH3DSP => Self::HitachiSH3DSP,
            IMAGE_FILE_MACHINE_SH4 => Self::HitachiSH4,
            IMAGE_FILE_MACHINE_SH5 => Self::HitachiSH5,
            IMAGE_FILE_MACHINE_THUMB => Self::Thumb,
            IMAGE_FILE_MACHINE_WCEMIPSV2 => Self::WCEMIPSV2,
            _ => Self::Unknown,
        }
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

pub const MACHINE_LIST: [u16; 29] = [
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

pub struct Characteristics {
    relocs_stripped: bool,
    executable_image: bool,
    line_nums_stripped: bool,
    local_syms_stripped: bool,
    agressive_ws_trim: bool,
    large_address_aware: bool,
    reserved: bool,
    bytes_reserved_lo: bool,
    x32_machine: bool,
    debug_stripped: bool,
    removable_run_from_swap: bool,
    net_run_from_swap: bool,
    system: bool,
    dynamic_link_library: bool,
    uniprocessor_system_only: bool,
    bytes_reserved_hi: bool,
}

impl From<u16> for Characteristics {
    fn from(value: u16) -> Self {
        let relocs_stripped = (value % 2) != 0;
        let executable_image = ((value >> 1) % 2) != 0;
        let line_nums_stripped = ((value >> 2) % 2) != 0;
        let local_syms_stripped = ((value >> 3) % 2) != 0;
        let agressive_ws_trim = ((value >> 4) % 2) != 0;
        let large_address_aware = ((value >> 5) % 2) != 0;
        let reserved = ((value >> 6) % 2) != 0;
        let bytes_reserved_lo = ((value >> 7) % 2) != 0;
        let x32_machine = ((value >> 7) % 2) != 0;
        let debug_stripped = ((value >> 8) % 2) != 0;
        let removable_run_from_swap = ((value >> 9) % 2) != 0;
        let net_run_from_swap = ((value >> 10) % 2) != 0;
        let system = ((value >> 11) % 2) != 0;
        let dynamic_link_library = ((value >> 12) % 2) != 0;
        let uniprocessor_system_only = ((value >> 13) % 2) != 0;
        let bytes_reserved_hi = ((value >> 14) % 2) != 0;
        
        Self {
            relocs_stripped,
            executable_image,
            line_nums_stripped,
            local_syms_stripped,
            agressive_ws_trim,
            large_address_aware,
            reserved,
            bytes_reserved_lo,
            x32_machine,
            debug_stripped,
            removable_run_from_swap,
            net_run_from_swap,
            system,
            dynamic_link_library,
            uniprocessor_system_only,
            bytes_reserved_hi,
        }
    }
}

pub const IMAGE_FILE_RELOCS_STRIPPED: u16 = 0x0001;
pub const IMAGE_FILE_EXECUTABLE_IMAGE: u16 = 0x0002;
pub const IMAGE_FILE_LINE_NUMS_STRIPPED: u16 = 0x0004;
pub const IMAGE_FILE_LOCAL_SYMS_STRIPPED: u16 = 0x0008;
pub const IMAGE_FILE_AGGRESSIVE_WS_TRIM: u16 = 0x0010;
pub const IMAGE_FILE_LARGE_ADDRESS_AWARE: u16 = 0x0020;
pub const IMAGE_FILE_RESERVED: u16 = 0x0040;
pub const IMAGE_FILE_BYTES_REVERSED_LO: u16 = 0x0080;
pub const IMAGE_FILE_32BIT_MACHINE: u16 = 0x0100;
pub const IMAGE_FILE_DEBUG_STRIPPED: u16 = 0x0200;
pub const IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP: u16 = 0x0400;
pub const IMAGE_FILE_NET_RUN_FROM_SWAP: u16 = 0x0800;
pub const IMAGE_FILE_SYSTEM: u16 = 0x1000;
pub const IMAGE_FILE_DLL: u16 = 0x2000;
pub const IMAGE_FILE_UP_SYSTEM_ONLY: u16 = 0x4000;
pub const IMAGE_FILE_BYTES_REVERSED_HI: u16 = 0x8000;
