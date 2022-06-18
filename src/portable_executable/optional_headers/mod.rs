mod data_directories;

use data_directories::DataDirectories;
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
