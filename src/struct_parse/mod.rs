use chrono::{DateTime, Utc};

use crate::header::machine_types::Machine;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct StructField<T: fmt::Debug> {
    pub offset: usize,
    pub raw_bytes: Vec<u8>,
    pub data: T,
    pub meaning: String,
}

impl Display for StructField<[u8; 4]> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = [
            self.raw_bytes[0],
            self.raw_bytes[1],
            self.raw_bytes[2],
            self.raw_bytes[3],
        ];
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
        let bytes = [self.raw_bytes[0], self.raw_bytes[1]];
        let _ = write!(f, "{:00}, ", self.offset,);
        let _ = write!(f, "0x{:00X} 0x{:00X}, ", bytes[0], bytes[1]);
        let _ = write!(f, "{}, ", self.data);
        write!(f, " ")
    }
}

impl Display for StructField<u32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = [
            self.raw_bytes[0],
            self.raw_bytes[1],
            self.raw_bytes[2],
            self.raw_bytes[3],
        ];
        let _ = write!(f, "{:00}, ", self.offset,);
        let _ = write!(
            f,
            "0x{:00X} 0x{:00X} 0x{:00X} 0x{:00X}, ",
            bytes[0], bytes[1], self.raw_bytes[2], self.raw_bytes[3]
        );
        let _ = write!(f, "{}, ", self.data);
        write!(f, "{}", self.meaning)
    }
}

impl Display for StructField<DateTime<Utc>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = [
            self.raw_bytes[0],
            self.raw_bytes[1],
            self.raw_bytes[2],
            self.raw_bytes[3],
        ];
        let _ = write!(f, "{:00}, ", self.offset,);
        let _ = write!(
            f,
            "0x{:00X} 0x{:00X} 0x{:00X} 0x{:00X}, ",
            bytes[0], bytes[1], self.raw_bytes[2], self.raw_bytes[3]
        );
        let _ = write!(f, "{}, ", self.data);
        write!(f, "")
    }
}

impl Display for StructField<Machine> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = [self.raw_bytes[0], self.raw_bytes[1]];
        let _ = write!(f, "{:00}, ", self.offset);
        let _ = write!(f, "0x{:00X} 0x{:00X}, ", bytes[0], bytes[1]);
        let _ = write!(f, "0x{:00X}, ", self.data);
        write!(f, "{}", self.meaning)
    }
}

pub trait ReadU16LE {
    fn read_u16_le(&self, relative_offset: usize) -> StructField<u16>;
}

pub trait ReadU32LE {
    fn read_u32_le(&self, relative_offset: usize) -> StructField<u32>;
}
