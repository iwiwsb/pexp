use std::fmt::Debug;

#[derive(Debug)]
pub struct StructField<T: Debug> {
    pub offset: u64,
    pub data: T,
}

pub trait ReadU16LE {
    fn read_u16_le(&mut self, offset: u64) -> u16;
}

pub trait ReadU32LE {
    fn read_u32_le(&mut self, offset: u64) -> u32;
}
