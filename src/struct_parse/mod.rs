use chrono::{DateTime, Utc};

use crate::header::machine_types::Machine;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct StructField<T: fmt::Debug> {
    pub offset: u64,
    pub bytes: Vec<u8>,
    pub data: T,
}

impl Display for StructField<[u8; 4]> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = [self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3]];
        let _ = write!(f, "{:00}, ", self.offset,);
        let _ = write!(
            f,
            "0x{:00X} 0x{:00X} 0x{:00X} 0x{:00X}, ",
            bytes[0], bytes[1], bytes[2], bytes[3]
        );
        let _ = write!(f, "{:X?}, ", u32::from_le_bytes(bytes).swap_bytes());
        write!(f, "")
    }
}

impl Display for StructField<u16> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = [self.bytes[0], self.bytes[1]];
        let _ = write!(f, "{:00}, ", self.offset,);
        let _ = write!(f, "0x{:00X} 0x{:00X}, ", bytes[0], bytes[1]);
        let _ = write!(f, "{}, ", self.data);
        write!(f, " ")
    }
}

impl Display for StructField<u32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = [self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3]];
        let _ = write!(f, "{:00}, ", self.offset,);
        let _ = write!(
            f,
            "0x{:00X} 0x{:00X} 0x{:00X} 0x{:00X}, ",
            bytes[0], bytes[1], self.bytes[2], self.bytes[3]
        );
        write!(f, "{}, ", self.data)
    }
}

impl Display for StructField<DateTime<Utc>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = [self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3]];
        let _ = write!(f, "{:00}, ", self.offset,);
        let _ = write!(
            f,
            "0x{:00X} 0x{:00X} 0x{:00X} 0x{:00X}, ",
            bytes[0], bytes[1], self.bytes[2], self.bytes[3]
        );
        let _ = write!(f, "{}, ", self.data);
        write!(f, "")
    }
}

impl Display for StructField<Machine> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = [self.bytes[0], self.bytes[1]];
        let _ = write!(f, "{:00}, ", self.offset);
        let _ = write!(f, "0x{:00X} 0x{:00X}, ", bytes[0], bytes[1]);
        write!(f, "0x{:00X}, ", self.data)
    }
}

pub trait ReadU16LE {
    fn read_u16_le(&self, relative_offset: usize) -> StructField<u16>;
}

pub trait ReadU32LE {
    fn read_u32_le(&self, relative_offset: usize) -> StructField<u32>;
}
