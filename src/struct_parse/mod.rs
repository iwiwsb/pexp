use std::fmt::Debug;

use chrono::NaiveDateTime;

use crate::header::machine_types::Machine;

#[derive(Debug, PartialEq)]
pub struct StructField<T: Debug> {
    pub offset: u64,
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
}

impl StructField<[u8; 4]> {
    pub fn as_u32_le(&self) -> u32 {
        u32::from_le_bytes(self.data)
    }

    pub fn as_datetime(&self) -> NaiveDateTime {
        NaiveDateTime::from_timestamp(self.as_u32_le() as i64, 0)
    }
}

pub trait ReadU16LE {
    fn read_u16_le(&mut self, offset: u64) -> u16;
}

pub trait ReadU32LE {
    fn read_u32_le(&mut self, offset: u64) -> u32;
}
