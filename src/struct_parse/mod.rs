use std::fmt::Debug;

#[derive(Debug)]
pub struct StructField<T: Debug> {
    pub offset: u64,
    pub data: T,
}

impl<T: Debug> StructField<T> {
    pub fn new(offset: u64, data: T) -> Self {
        Self { offset, data }
    }
}

impl StructField<u16> {
    pub fn raw_le_bytes(&self) -> Vec<u8> {
        self.data.to_le_bytes().to_vec()
    }
}

impl StructField<u32> {
    pub fn raw_le_bytes(&self) -> Vec<u8> {
        self.data.to_le_bytes().to_vec()
    }
}

impl StructField<u64> {
    pub fn raw_le_bytes(&self) -> Vec<u8> {
        self.data.to_le_bytes().to_vec()
    }
}

pub trait ReadU16LE {
    fn read_u16_le(&mut self, offset: u64) -> u16;
}

pub trait ReadU32LE {
    fn read_u32_le(&mut self, offset: u64) -> u32;
}
