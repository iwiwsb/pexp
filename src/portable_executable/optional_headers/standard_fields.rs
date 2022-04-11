pub struct StandardFields {
    magic: PEImageType,
    major_linker_version: u8,
    minor_linker_version: u8,
    size_of_code: u32,
    size_of_initialized_data: u32,
    size_of_uninitialized_data: u32,
    address_of_entry_point: u32,
    base_of_code: u32,
    base_of_data: Option<u32>,
}

impl TryFrom<[u8; 28]> for StandardFields {
    type Error = &'static str;
    fn try_from(buffer: [u8; 28]) -> Result<Self, Self::Error> {
        let bytes_0_1 = [buffer[0], buffer[1]];
        let bytes_2 = buffer[2];
        let bytes_3 = buffer[3];
        let bytes_4_7 = [buffer[4], buffer[5], buffer[6], buffer[7]];
        let bytes_8_11 = [buffer[8], buffer[9], buffer[10], buffer[11]];
        let bytes_12_15 = [buffer[12], buffer[13], buffer[14], buffer[15]];
        let bytes_16_19 = [buffer[16], buffer[17], buffer[18], buffer[19]];
        let bytes_20_23 = [buffer[20], buffer[21], buffer[22], buffer[23]];
        let bytes_24_27 = [buffer[24], buffer[25], buffer[26], buffer[27]];

        let magic = PEImageType::try_from(bytes_0_1)?;
        let major_linker_version = bytes_2;
        let minor_linker_version = bytes_3;
        let size_of_code = u32::from_le_bytes(bytes_4_7);
        let size_of_initialized_data = u32::from_le_bytes(bytes_8_11);
        let size_of_uninitialized_data = u32::from_le_bytes(bytes_12_15);
        let address_of_entry_point = u32::from_le_bytes(bytes_16_19);
        let base_of_code = u32::from_le_bytes(bytes_20_23);
        let base_of_data = match magic {
            PEImageType::PE32 => Some(u32::from_le_bytes(bytes_24_27)),
            PEImageType::PE64 => None,
        };
        Ok(Self {
            magic,
            major_linker_version,
            minor_linker_version,
            size_of_code,
            size_of_initialized_data,
            size_of_uninitialized_data,
            address_of_entry_point,
            base_of_code,
            base_of_data,
        })
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
