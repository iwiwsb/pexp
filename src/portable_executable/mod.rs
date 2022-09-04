use std::io::{self, BufRead, ErrorKind, Read, Seek, SeekFrom};

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

#[derive(PartialEq, Debug)]
pub struct COFFFileHeader {
    raw: Vec<u8>,
}

impl COFFFileHeader {
    pub fn machine(&self) -> Result<MachineType, &str> {
        MachineType::try_from([self.raw[0], self.raw[1]])
    }

    pub fn number_of_sections(&self) -> u16 {
        u16::from_le_bytes([self.raw[2], self.raw[3]])
    }

    pub fn time_date_stamp(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(
                i64::from_le_bytes([
                    self.raw[4],
                    self.raw[5],
                    self.raw[6],
                    self.raw[7],
                    0,
                    0,
                    0,
                    0,
                ]),
                0,
            ),
            Utc,
        )
    }

    pub fn pointer_to_symbol_table(&self) -> u32 {
        u32::from_le_bytes([self.raw[8], self.raw[9], self.raw[10], self.raw[11]])
    }

    pub fn number_of_symbols(&self) -> u32 {
        u32::from_le_bytes([self.raw[12], self.raw[13], self.raw[14], self.raw[15]])
    }

    pub fn size_of_optional_header(&self) -> u16 {
        u16::from_le_bytes([self.raw[16], self.raw[17]])
    }

    pub fn characteristics(&self) -> Characteristics {
        Characteristics(u16::from_le_bytes([self.raw[18], self.raw[19]]))
    }
}

pub struct Characteristics(u16);

impl Characteristics {
    pub fn relocs_stripped(&self) -> bool {
        self.0 & 0b1 == 1
    }

    pub fn executable_image(&self) -> bool {
        (self.0 >> 1) & 0b1 == 1
    }

    pub fn line_nums_stripped(&self) -> bool {
        (self.0 >> 2) & 0b1 == 1
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

use PEImageType::*;

#[allow(non_snake_case)]
pub struct OptionalHeader {
    raw: Vec<u8>,
    image_type: PEImageType,
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
            ROM => todo!(),
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

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
enum PEImageType {
    PE32,
    PE64,
    ROM,
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
    coff_file_header: COFFFileHeader,
    optional_header: Option<OptionalHeader>,
    section_table: SectionTable,
}

#[derive(Clone, Copy, Debug)]
enum PEType {
    Object,
    Image,
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

impl<R: BufRead + Seek> Reader<R> {
    pub fn decode(mut self) -> io::Result<PEHeaders> {
        let mut mz = [0u8; 2];
        self.inner.read(&mut mz)?;
        let petype = if mz == [b'M', b'Z'] {
            PEType::Image
        } else if MachineType::try_from(mz).is_ok() {
            PEType::Object
        } else {
            return Err(io::Error::from(ErrorKind::InvalidData));
        };
        match petype {
            PEType::Image => self.inner.seek(SeekFrom::Start(0x3C)),
            PEType::Object => self.inner.seek(SeekFrom::Start(0)),
        };
        todo!()
    }

    fn guess_type(&mut self) -> io::Result<()> {
        let mut first_2_bytes = [0u8; 2];
        self.inner.read(&mut first_2_bytes)?;
        if first_2_bytes == [b'M', b'Z'] {
            self.pe_type = Some(PEType::Image).or(self.pe_type);
        } else if MachineType::try_from(first_2_bytes).is_ok() {
            self.pe_type = Some(PEType::Object).or(self.pe_type);
        } else {
            return Err(io::Error::from(ErrorKind::InvalidData));
        }
        return Ok(());
    }
}

#[cfg(test)]
mod tests {}
