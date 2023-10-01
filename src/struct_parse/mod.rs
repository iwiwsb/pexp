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
        let _ = write!(f, "{:00}\t", self.offset,);
        let _ = write!(
            f,
            "{:00X} {:00X} {:00X} {:00X}\t",
            bytes[0], bytes[1], bytes[2], bytes[3]
        );
        let _ = write!(f, "{:0000X}    ", u32::from_le_bytes(bytes));
        write!(f, "")
    }
}

impl Display for StructField<u16> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = [self.raw_bytes[0], self.raw_bytes[1]];
        let _ = write!(f, "{:00}\t", self.offset,);
        let _ = write!(f, "{:00X} {:00X}\t\t", bytes[0], bytes[1]);
        let _ = write!(f, "{}\t", u16::from_le_bytes(bytes));
        write!(f, "")
    }
}

impl Display for StructField<Machine> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = [self.raw_bytes[0], self.raw_bytes[1]];
        let _ = write!(f, "{:00}\t", self.offset,);
        let _ = write!(f, "{:00X} {:00X}\t\t", bytes[0], bytes[1]);
        let _ = write!(f, "{:00X}\t", u16::from_le_bytes(bytes));
        write!(f, "{}", self.meaning)
    }
}

trait ReadU16LE {
    fn read_u16_le(&mut self) -> u16;
}

trait ParseStruct {
    fn parse_struct(offset: usize, buffer: Vec<u8>) -> Self
    where
        Self: Sized,
    {
        todo!()
    }
}
