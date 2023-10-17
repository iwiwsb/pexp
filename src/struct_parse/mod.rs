use std::fmt::Debug;

use crate::header::{characteristics::Characteristics, ImageType};
use chrono::NaiveDateTime;

use crate::header::machine_types::Machine;

#[derive(Debug, PartialEq)]
pub struct StructField<T: Debug> {
    pub abs_offset: u64,
    pub data: T,
}

impl StructField<[u8; 1]> {
    pub fn byte(&self) -> u8 {
        self.data[0]
    }
}

impl StructField<[u8; 2]> {
    pub fn as_u16_le(&self) -> u16 {
        u16::from_le_bytes(self.data)
    }

    pub fn as_machine(&self) -> Machine {
        Machine::from(self.as_u16_le())
    }

    pub fn as_characteristics(&self) -> Characteristics {
        todo!()
    }

    pub fn as_image_type(&self) -> ImageType {
        match self.as_u16_le() {
            0x010B => ImageType::Image32,
            0x020B => ImageType::Image64,
            0x0107 => ImageType::ImageRom,
            _ => ImageType::ImageUnknown,
        }
    }
}

impl StructField<[u8; 4]> {
    pub fn as_u32_le(&self) -> u32 {
        u32::from_le_bytes(self.data)
    }

    pub fn as_datetime(&self) -> NaiveDateTime {
        NaiveDateTime::from_timestamp(self.as_u32_le() as i64, 0)
    }
}

impl StructField<[u8; 8]> {
    pub fn as_u64_le(&self) -> u64 {
        u64::from_le_bytes(self.data)
    }
}
