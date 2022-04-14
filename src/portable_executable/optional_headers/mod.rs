mod data_directories;

use data_directories::DataDirectories;

#[allow(non_snake_case)]
pub struct OptionalHeader {
    magic: PEImageType,
    major_linker_version: u8,
    minor_linker_version: u8,
    size_of_code: u32,
    size_of_initialized_data: u32,
    size_of_uninitialized_data: u32,
    address_of_entry_point: u32,
    base_of_code: u32,
    base_of_data: Option<u32>,
    image_base: u64,
    section_alignment: u32,
    file_alignment: u32,
    major_operating_system_version: u16,
    minor_operating_system_version: u16,
    major_image_version: u16,
    minor_image_version: u16,
    major_subsystem_version: u16,
    minor_subsystem_version: u16,
    win32_version_value: u32,
    size_of_image: u32,
    size_of_headers: u32,
    check_sum: u32,
    subsystem: Subsystem,
    dll_characteristics: u16,
    size_of_stack_reserve: u64,
    size_of_stack_commit: u64,
    size_of_heap_reserve: u64,
    size_of_heap_commit: u64,
    loader_flags: u32,
    number_of_RVA_and_sizes: u32,
    data_directories: DataDirectories,
}

impl TryFrom<[u8; 112]> for OptionalHeader {
    type Error = &'static str;
    fn try_from(buffer: [u8; 112]) -> Result<Self, Self::Error> {
        use PEImageType::*;

        let bytes_0_1 = [buffer[0], buffer[1]];
        let bytes_2 = buffer[2];
        let bytes_3 = buffer[3];
        let bytes_4_7 = [buffer[4], buffer[5], buffer[6], buffer[7]];
        let bytes_8_11 = [buffer[8], buffer[9], buffer[10], buffer[11]];
        let bytes_12_15 = [buffer[12], buffer[13], buffer[14], buffer[15]];
        let bytes_16_19 = [buffer[16], buffer[17], buffer[18], buffer[19]];
        let bytes_20_23 = [buffer[20], buffer[21], buffer[22], buffer[23]];
        let bytes_24_27 = [buffer[24], buffer[25], buffer[26], buffer[27]];
        let bytes_32_35 = [buffer[32], buffer[33], buffer[34], buffer[35]];
        let bytes_36_39 = [buffer[36], buffer[37], buffer[38], buffer[39]];
        let bytes_40_41 = [buffer[40], buffer[41]];
        let bytes_42_43 = [buffer[42], buffer[43]];
        let bytes_44_45 = [buffer[44], buffer[45]];
        let bytes_46_47 = [buffer[46], buffer[47]];
        let bytes_48_49 = [buffer[48], buffer[49]];
        let bytes_50_53 = [buffer[50], buffer[51], buffer[52], buffer[53]];
        let bytes_54_57 = [buffer[54], buffer[55], buffer[56], buffer[57]];
        let bytes_58_61 = [buffer[58], buffer[59], buffer[60], buffer[61]];

        let magic = PEImageType::try_from(bytes_0_1)?;
        let major_linker_version = bytes_2;
        let minor_linker_version = bytes_3;
        let size_of_code = u32::from_le_bytes(bytes_4_7);
        let size_of_initialized_data = u32::from_le_bytes(bytes_8_11);
        let size_of_uninitialized_data = u32::from_le_bytes(bytes_12_15);
        let address_of_entry_point = u32::from_le_bytes(bytes_16_19);
        let base_of_code = u32::from_le_bytes(bytes_20_23);
        let base_of_data = match magic {
            PE32 => Some(u32::from_le_bytes(bytes_24_27)),
            PE64 => None,
        };
        let win_offset: usize = match magic {
            PE32 => 28,
            PE64 => 24,
        };
        let image_base = match magic {
            PE32 => u64::from_le_bytes([
                buffer[win_offset],
                buffer[win_offset + 4],
                buffer[win_offset + 8],
                buffer[win_offset + 12],
                0,
                0,
                0,
                0,
            ]),
            PE64 => u64::from_le_bytes([
                buffer[win_offset],
                buffer[win_offset + 4],
                buffer[win_offset + 8],
                buffer[win_offset + 12],
                buffer[win_offset + 16],
                buffer[win_offset + 20],
                buffer[win_offset + 24],
                buffer[win_offset + 32],
            ]),
        };
        let section_alignment = u32::from_le_bytes(bytes_32_35);
        let file_alignment = u32::from_le_bytes(bytes_36_39);
        let major_operating_system_version = u16::from_le_bytes(bytes_40_41);
        let minor_operating_system_version = u16::from_le_bytes(bytes_42_43);
        let major_image_version = u16::from_le_bytes(bytes_44_45);
        let major_subsystem_version = u16::from_le_bytes(bytes_46_47);
        let minor_subsystem_version = u16::from_le_bytes(bytes_48_49);
        let win32_version_value = u32::from_le_bytes(bytes_50_53);
        let size_of_image = u32::from_le_bytes(bytes_54_57);
        let size_of_headers = u32::from_le_bytes(bytes_58_61);
        todo!()
    }
}

#[allow(non_camel_case_types)]
enum PEImageType {
    PE32,
    PE64,
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
