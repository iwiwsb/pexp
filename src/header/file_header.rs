use crate::struct_parse::{ReadU32LE, StructField};
use crate::{header::machine_types::Machine, struct_parse::ReadU16LE};
use chrono::{DateTime, NaiveDateTime, Utc};
use std::fmt::Display;

pub struct FileHeaderBuffer {
    offset: usize,
    buffer: Vec<u8>,
}

#[allow(non_snake_case)]
impl FileHeaderBuffer {
    pub fn new(offset: usize, buffer: Vec<u8>) -> Self {
        Self { offset, buffer }
    }

    pub fn read_file_header(&self) -> FileHeader {
        let signature = self.read_signature();
        let machine = self.read_machine();
        let number_of_sections = self.read_number_of_sections();
        let time_date_stamp = self.read_time_date_stamp();
        let pointer_to_symbol_table = self.read_pointer_to_symbol_table();
        let number_of_symbols = self.read_number_of_symbols();
        let size_of_optional_header = self.read_size_of_optional_header();
        let characteristics = self.read_characteristics();

        FileHeader {
            signature,
            machine,
            number_of_sections,
            time_date_stamp,
            pointer_to_symbol_table,
            number_of_symbols,
            size_of_optional_header,
            characteristics,
        }
    }

    pub fn read_signature(&self) -> StructField<[u8; 4]> {
        let relative_offset = 0;
        let offset = self.offset + relative_offset;
        let raw_bytes = self.buffer[relative_offset..relative_offset + 4].to_vec();
        let data = [raw_bytes[0], raw_bytes[1], raw_bytes[2], raw_bytes[3]];
        let meaning = "".to_string();

        StructField {
            offset,
            raw_bytes,
            data,
            meaning,
        }
    }

    pub fn read_machine(&self) -> StructField<Machine> {
        let relative_offset = 4;
        let _u16 = self.read_u16_le(relative_offset);
        let raw_bytes = _u16.raw_bytes;
        let data = Machine::try_from([raw_bytes[0], raw_bytes[1]]).unwrap();
        let offset = self.offset + relative_offset;
        let meaning = data.to_string();

        StructField {
            offset,
            raw_bytes,
            data,
            meaning,
        }
    }

    fn read_number_of_sections(&self) -> StructField<u16> {
        self.read_u16_le(6)
    }

    fn read_time_date_stamp(&self) -> StructField<DateTime<Utc>> {
        let relative_offset = 8;
        let raw_bytes = self.buffer[relative_offset..relative_offset + 4].to_vec();
        let secs = u32::from_le_bytes([raw_bytes[0], raw_bytes[1], raw_bytes[2], raw_bytes[3]]);
        let nsecs = 0;
        let data = DateTime::from_utc(NaiveDateTime::from_timestamp(secs as i64, nsecs), Utc);
        let offset = self.offset + relative_offset;
        let meaning = data.format("%v").to_string();

        StructField {
            offset,
            raw_bytes,
            data,
            meaning,
        }
    }

    fn read_pointer_to_symbol_table(&self) -> StructField<u32> {
        self.read_u32_le(12)
    }

    fn read_number_of_symbols(&self) -> StructField<u32> {
        self.read_u32_le(16)
    }

    fn read_size_of_optional_header(&self) -> StructField<u16> {
        self.read_u16_le(20)
    }

    fn read_characteristics(&self) -> StructField<u16> {
        self.read_u16_le(22)
    }
}

impl ReadU16LE for FileHeaderBuffer {
    fn read_u16_le(&self, relative_offset: usize) -> StructField<u16> {
        let raw_bytes = self.buffer[relative_offset..relative_offset + 2].to_vec();
        let data = u16::from_le_bytes([raw_bytes[0], raw_bytes[1]]);
        let offset = self.offset + relative_offset;
        let meaning = data.to_string();

        StructField {
            offset,
            raw_bytes,
            data,
            meaning,
        }
    }
}

impl ReadU32LE for FileHeaderBuffer {
    fn read_u32_le(&self, relative_offset: usize) -> StructField<u32> {
        let raw_bytes = self.buffer[relative_offset..relative_offset + 4].to_vec();
        let data = u32::from_le_bytes([raw_bytes[0], raw_bytes[1], raw_bytes[2], raw_bytes[3]]);
        let offset = self.offset + relative_offset;
        let meaning = data.to_string();

        StructField {
            offset,
            raw_bytes,
            data,
            meaning,
        }
    }
}

/// COFF File Header structure
#[derive(Debug)]
pub struct FileHeader {
    pub signature: StructField<[u8; 4]>,
    /// Identifies the type of target machine. For more information, see [`machine_types`](machine_types).
    pub machine: StructField<Machine>,
    /// Indicates the size of the section table, which immediately follows the headers.
    pub number_of_sections: StructField<u16>,
    /// The low 32 bits of the number of seconds since 00:00 January 1, 1970 (a C run-time time_t value), which indicates when the file was created.
    pub time_date_stamp: StructField<DateTime<Utc>>,
    /// The file offset of the COFF symbol table, or zero if no COFF symbol table is present.
    /// This value should be zero for an image because COFF debugging information is deprecated.
    pub pointer_to_symbol_table: StructField<u32>,
    /// The number of entries in the symbol table.
    /// This data can be used to locate the string table, which immediately follows the symbol table.
    /// This value should be zero for an image because COFF debugging information is deprecated.
    pub number_of_symbols: StructField<u32>,
    /// The size of the [`OptionalHeader`](crate::header::OptionalHeader), which is required for executable files but not for object files.
    /// This value should be zero for an object file.
    pub size_of_optional_header: StructField<u16>,
    /// The flags that indicate the attributes of the file. For specific flag values, see [`characteristics`](characteristics)
    pub characteristics: StructField<u16>,
}

trait ParseStruct {
    fn parse_struct(offset: usize, buffer: Vec<u8>) -> Self;
}

impl ParseStruct for FileHeader {
    fn parse_struct(_offset: usize, _buffer: Vec<u8>) -> Self {
        todo!()
    }
}

impl Display for FileHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "File header:");
        let _ = writeln!(
            f,
            "Field name, Offset (dec), Raw value (hex), Value, Meaning"
        );
        let _ = writeln!(f, "Signature, {}", self.signature);
        let _ = writeln!(f, "Machine, {}", self.machine);
        let _ = writeln!(f, "Num. of Sections, {}", self.number_of_sections);
        let _ = writeln!(f, "Timestamp, {}", self.time_date_stamp);
        let _ = writeln!(
            f,
            "Pointer to symbol table, {}",
            self.pointer_to_symbol_table
        );
        let _ = writeln!(f, "Number of symbols, {}", self.number_of_symbols);
        let _ = writeln!(
            f,
            "Size of optional header, {}",
            self.size_of_optional_header
        );
        writeln!(f, "Characteristics, {}", self.characteristics)
    }
}
