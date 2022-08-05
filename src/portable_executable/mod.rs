use std::io::{self, ErrorKind, Read, Seek, SeekFrom};

use chrono::{DateTime, TimeZone, Utc};

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
#[allow(non_camel_case_types)]
pub enum MachineType {
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

#[allow(non_camel_case_types)]
enum Characteristics {
    RELOCS_STRIPPED,
    EXECUTABLE_IMAGE,
    LINE_NUMS_STRIPPED,
    SYMS_STRIPPED,
    AGGRESSIVE_WS_TRIM,
    LARGE_ADDRESS_AWARE,
    BYTES_REVERSED_LO,
    MACHINE_32BIT,
    DEBUG_STRIPPED,
    REMOVABLE_RUN_FROM_SWAP,
    RUN_FROM_SWAP,
    SYSTEM,
    DLL,
    SYSTEM_ONLY,
    BYTES_REVERSED_HI,
}

use ImageBase::*;
use PEImageType::*;

#[allow(non_snake_case)]
pub struct OptionalHeader {
    raw: Vec<u8>,
    image_type: PEImageType,
}

enum ImageBase {
    ImageBase32(u32),
    ImageBase64(u64),
}

impl OptionalHeader {
    fn magic(&self) -> u16 {
        u16::from_le_bytes([self.raw[0], self.raw[1]])
    }

    fn image_type(&self) -> &PEImageType {
        &self.image_type
    }

    fn major_linker_version(&self) -> u8 {
        self.raw[2]
    }

    fn minor_linker_version(&self) -> u8 {
        self.raw[3]
    }

    fn size_of_code(&self) -> u32 {
        u32::from_le_bytes([self.raw[4], self.raw[5], self.raw[6], self.raw[7]])
    }

    fn size_of_initialized_data(&self) -> u32 {
        u32::from_le_bytes([self.raw[8], self.raw[9], self.raw[10], self.raw[11]])
    }

    fn size_of_uninitialized_data(&self) -> u32 {
        u32::from_le_bytes([self.raw[12], self.raw[13], self.raw[14], self.raw[15]])
    }

    fn address_of_entry_point(&self) -> u32 {
        u32::from_le_bytes([self.raw[16], self.raw[17], self.raw[18], self.raw[19]])
    }

    fn base_of_code(&self) -> u32 {
        u32::from_le_bytes([self.raw[20], self.raw[21], self.raw[22], self.raw[23]])
    }

    fn base_of_data(&self) -> Option<u32> {
        match self.image_type() {
            PE32 => Some(u32::from_le_bytes([
                self.raw[24],
                self.raw[25],
                self.raw[26],
                self.raw[27],
            ])),
            PE64 => None,
        }
    }

    fn image_base(&self) -> ImageBase {
        match self.image_type() {
            PE32 => ImageBase32(u32::from_le_bytes([
                self.raw[28],
                self.raw[29],
                self.raw[30],
                self.raw[31],
            ])),
            PE64 => ImageBase64(u64::from_le_bytes([
                self.raw[28],
                self.raw[29],
                self.raw[30],
                self.raw[31],
                self.raw[32],
                self.raw[33],
                self.raw[34],
                self.raw[35],
            ])),
        }
    }

    fn section_alignment(&self) -> u32 {
        u32::from_le_bytes([self.raw[32], self.raw[33], self.raw[34], self.raw[35]])
    }

    fn file_alignment(&self) -> u32 {
        u32::from_le_bytes([self.raw[36], self.raw[37], self.raw[38], self.raw[39]])
    }
    fn major_operating_system_version(&self) -> u16 {
        u16::from_le_bytes([self.raw[40], self.raw[41]])
    }
    fn minor_operating_system_version(&self) -> u16 {
        u16::from_le_bytes([self.raw[42], self.raw[43]])
    }
    fn major_image_version(&self) -> u16 {
        u16::from_le_bytes([self.raw[44], self.raw[45]])
    }
    fn minor_image_version(&self) -> u16 {
        u16::from_le_bytes([self.raw[46], self.raw[47]])
    }
    fn major_subsystem_version(&self) -> u16 {
        u16::from_le_bytes([self.raw[48], self.raw[49]])
    }
    fn minor_subsystem_version(&self) -> u16 {
        u16::from_le_bytes([self.raw[50], self.raw[51]])
    }
    fn win32_version_value(&self) -> u32 {
        u32::from_le_bytes([self.raw[52], self.raw[53], self.raw[54], self.raw[55]])
    }
    fn size_of_image(&self) -> u32 {
        u32::from_le_bytes([self.raw[56], self.raw[57], self.raw[58], self.raw[59]])
    }
    fn size_of_headers(&self) -> u32 {
        u32::from_le_bytes([self.raw[60], self.raw[61], self.raw[62], self.raw[63]])
    }
    fn checksum(&self) -> u32 {
        u32::from_le_bytes([self.raw[64], self.raw[65], self.raw[66], self.raw[67]])
    }
    fn subsystem(&self) -> u16 {
        u16::from_le_bytes([self.raw[68], self.raw[69]])
    }
    fn dll_characteristics(&self) -> u16 {
        u16::from_le_bytes([self.raw[70], self.raw[71]])
    }
}

impl TryFrom<[u8; 112]> for OptionalHeader {
    type Error = &'static str;
    fn try_from(buffer: [u8; 112]) -> Result<Self, Self::Error> {
        match PEImageType::try_from([buffer[0], buffer[1]]) {
            Ok(PE32) => Ok(OptionalHeader {
                raw: buffer.to_vec(),
                image_type: PE32,
            }),
            Ok(PE64) => Ok(OptionalHeader {
                raw: buffer.to_vec(),
                image_type: PE64,
            }),
            Err(s) => panic!("{s}"),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
enum PEImageType {
    PE32,
    PE64,
    // ROM,
}

impl TryFrom<u16> for PEImageType {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x010B => Ok(Self::PE32),
            0x020B => Ok(Self::PE64),
            _ => Err("invalid image type"),
        }
    }
}

impl TryFrom<[u8; 2]> for PEImageType {
    type Error = &'static str;

    fn try_from(value: [u8; 2]) -> Result<Self, Self::Error> {
        match value {
            [0x0B, 0x01] => Ok(Self::PE32),
            [0x0B, 0x02] => Ok(Self::PE64),
            _ => Err("invalid image type"),
        }
    }
}

#[allow(non_camel_case_types)]
enum Subsystem {
    UNKNOWN,
    NATIVE,
    WINDOWS_GUI,
    WINDOWS_CUI,
    OS2_CUI,
    POSIX_CUI,
    NATIVE_WINDOWS,
    WINDOWS_CE_GUI,
    EFI_APPLICATION,
    EFI_BOOT_SERVICE_DRIVER,
    EFI_RUNTIME_DRIVER,
    EFI_ROM,
    XBOX,
    WINDOWS_BOOT_APPLICATION,
}

impl TryFrom<[u8; 2]> for Subsystem {
    type Error = &'static str;

    fn try_from(value: [u8; 2]) -> Result<Self, Self::Error> {
        match value {
            [0x00, 0x00] => Ok(Self::UNKNOWN),
            [0x01, 0x00] => Ok(Self::NATIVE),
            [0x02, 0x00] => Ok(Self::WINDOWS_GUI),
            [0x03, 0x00] => Ok(Self::WINDOWS_CUI),
            [0x05, 0x00] => Ok(Self::OS2_CUI),
            [0x07, 0x00] => Ok(Self::POSIX_CUI),
            [0x08, 0x00] => Ok(Self::NATIVE_WINDOWS),
            [0x09, 0x00] => Ok(Self::WINDOWS_CE_GUI),
            [0x0A, 0x00] => Ok(Self::EFI_APPLICATION),
            [0x0B, 0x00] => Ok(Self::EFI_BOOT_SERVICE_DRIVER),
            [0x0C, 0x00] => Ok(Self::EFI_RUNTIME_DRIVER),
            [0x0D, 0x00] => Ok(Self::EFI_ROM),
            [0x0E, 0x00] => Ok(Self::XBOX),
            [0x10, 0x00] => Ok(Self::WINDOWS_BOOT_APPLICATION),
            _ => Err("invalid subsystem type"),
        }
    }
}

pub struct DataDirectories {
    export: DataDir,
    import: DataDir,
    resource: DataDir,
    exception: DataDir,
    certificate: DataDir,
    base_relocation: DataDir,
    debug: DataDir,
    architecture: DataDir,
    global_ptr: DataDir,
    tls: DataDir,
    load_config: DataDir,
    bound_import: DataDir,
    iat: DataDir,
    delay_import_descriptor: DataDir,
    clr_runtime_header: DataDir,
    reserved: DataDir,
}

struct DataDir {
    virtual_address: u32,
    size: u32,
}

pub struct SectionTable {
    name: String,
    virtual_size: u32,
    virtual_address: u32,
    size_of_raw_data: u32,
    pointer_to_raw_data: u32,
    pointer_to_relocations: u32,
    pointer_to_line_numbers: u32,
    number_of_relocations: u32,
    number_of_line_numbers: u32,
    characteristics: u32,
}

pub struct PEHeaders {
    ms_dos_stub: Vec<u8>,
    signature: [u8; 4],
    coff_file_header: COFFFileHeader,
    optional_header: Option<OptionalHeader>,
    section_table: SectionTable,
}

#[derive(Clone, Copy, Debug)]
enum PEType {
    Object,
    Image32,
    Image64,
    ImageRom,
}

struct Reader<R: Read> {
    inner: R,
    pe_type: Option<PEType>,
}

impl<R: Read> Reader<R> {
    fn new(inner: R) -> Self {
        Self {
            inner,
            pe_type: None,
        }
    }

    fn with_type(inner: R, pe_type: PEType) -> Self {
        Self {
            inner,
            pe_type: Some(pe_type),
        }
    }

    fn pe_type(&self) -> Option<PEType> {
        self.pe_type
    }

    fn set_type(&mut self, pe_type: PEType) {
        self.pe_type = Some(pe_type);
    }
}

impl<R: Read + Seek> Reader<R> {
    pub fn decode(mut self) -> io::Result<PEHeaders> {
        todo!()
    }

    fn guess_type(&mut self) -> io::Result<()> {
        let mut first_2_bytes = [0u8; 2];
        self.inner.read(&mut first_2_bytes)?;
        if first_2_bytes == [b'M', b'Z'] {
            self.inner.seek(SeekFrom::Start(0x3C))?;
            let mut pe_signature_offset = [0u8; 4];
            self.inner.read(&mut pe_signature_offset)?;
            self.inner.seek(SeekFrom::Start(
                u32::from_le_bytes(pe_signature_offset) as u64
            ))?;
            let mut pe_signature = [0u8; 4];
            self.inner.read(&mut pe_signature)?;
            if pe_signature == [b'P', b'E', 0, 0] {
                self.inner.seek(SeekFrom::Current(20))?;
                let mut pe_magic = [0u8; 2];
                self.inner.read(&mut pe_magic)?;
                match pe_magic {
                    [0x0B, 0x01] => self.pe_type = Some(PEType::Image32).or(self.pe_type),
                    [0x0B, 0x02] => self.pe_type = Some(PEType::Image64).or(self.pe_type),
                    [0x07, 0x01] => self.pe_type = Some(PEType::ImageRom).or(self.pe_type),
                    _ => return Err(io::Error::from(ErrorKind::InvalidData)),
                }
                return Ok(());
            } else {
                return Err(io::Error::from(ErrorKind::InvalidData));
            }
        } else if MachineType::try_from(first_2_bytes).is_ok() {
            self.pe_type = Some(PEType::Object).or(self.pe_type);
            return Ok(());
        } else {
            return Err(io::Error::from(ErrorKind::InvalidData));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coff_parsing() {
        let test_coff: [u8; 20] = [
            0x4C, 0x01, 0x03, 0x00, 0xDC, 0x52, 0xDB, 0xFE, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0xE0, 0x00, 0x22, 0x00,
        ];

        let parsed = COFFFileHeader {
            machine: MachineType::I386,
            number_of_sections: 0x03,
            time_date_stamp: Utc.timestamp(4275786460, 0),
            pointer_to_symbol_table: None,
            number_of_symbols: 0,
            size_of_optional_header: 0xE0,
            characteristics: 0x22,
        };
        assert_eq!(parsed, COFFFileHeader::try_from(test_coff).unwrap())
    }
}
