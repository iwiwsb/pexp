use crate::header::machine_types::Machine;
use crate::struct_parse::StructField;
use chrono::NaiveDateTime;
use std::fmt::{Debug, Display};
use std::io::{Read, Seek, SeekFrom};

pub struct FileHeaderReader<R: Read + Seek> {
    offset: u64,
    buffer: R,
}

#[allow(non_snake_case)]
impl<R> FileHeaderReader<R>
where
    R: Read + Seek,
{
    pub fn new(offset: u64, buffer: R) -> Self {
        Self { offset, buffer }
    }

    pub fn read_file_header(&mut self) -> FileHeader {
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

    pub fn read_signature(&mut self) -> StructField<[u8; 4]> {
        let offset = self.offset;
        let buf = self.read_array(offset);

        StructField {
            offset,
            bytes: buf.to_vec(),
            data: buf,
        }
    }

    pub fn read_machine(&mut self) -> StructField<Machine> {
        let offset = self.offset + 4;
        let buf = self.read_array(offset);
        let raw_bytes = buf.to_vec();
        let data = Machine::try_from(buf).unwrap();

        StructField {
            offset,
            bytes: raw_bytes,
            data,
        }
    }

    pub fn read_number_of_sections(&mut self) -> StructField<u16> {
        self.read_u16_le_field(self.offset + 6)
    }

    pub fn read_time_date_stamp(&mut self) -> StructField<NaiveDateTime> {
        let offset = self.offset + 8;
        let buf = self.read_array(offset);
        let timestamp = u32::from_le_bytes(buf);
        let bytes = buf.to_vec();
        let data = NaiveDateTime::from_timestamp_opt(timestamp as i64, 0).unwrap();

        StructField {
            offset,
            bytes,
            data,
        }
    }

    pub fn read_pointer_to_symbol_table(&mut self) -> StructField<u32> {
        self.read_u32_le_field(self.offset + 12)
    }

    pub fn read_number_of_symbols(&mut self) -> StructField<u32> {
        self.read_u32_le_field(self.offset + 16)
    }

    pub fn read_size_of_optional_header(&mut self) -> StructField<u16> {
        self.read_u16_le_field(self.offset + 20)
    }

    pub fn read_characteristics(&mut self) -> StructField<u16> {
        self.read_u16_le_field(self.offset + 22)
    }

    fn read_u16_le_field(&mut self, offset: u64) -> StructField<u16> {
        let offset = self.offset + offset;
        let buf = self.read_array(offset);
        let data = u16::from_le_bytes(buf);
        let bytes = buf.to_vec();

        StructField {
            offset,
            bytes,
            data,
        }
    }

    fn read_u32_le_field(&mut self, offset: u64) -> StructField<u32> {
        let offset = self.offset + offset;
        let buf = self.read_array(offset);
        let bytes = buf.to_vec();
        let data = u32::from_le_bytes(buf);

        StructField {
            offset,
            bytes,
            data,
        }
    }

    fn read_array<const N: usize>(&mut self, offset: u64) -> [u8; N] {
        let pos = SeekFrom::Start(self.offset + offset);
        let _ = self.buffer.seek(pos);
        let mut buf = [0u8; N];
        let _ = self.buffer.read_exact(&mut buf);
        buf
    }
}

/// COFF File Header structure
#[derive(Debug)]
pub struct FileHeader {
    pub signature: StructField<[u8; 4]>,
    /// Identifies the type of target machine. For more information, see [`machine_types`](crate::header::machine_types).
    pub machine: StructField<Machine>,
    /// Indicates the size of the section table, which immediately follows the headers.
    pub number_of_sections: StructField<u16>,
    /// The low 32 bits of the number of seconds since 00:00 January 1, 1970 (a C run-time time_t value), which indicates when the file was created.
    pub time_date_stamp: StructField<NaiveDateTime>,
    /// The file offset of the COFF symbol table, or zero if no COFF symbol table is present.
    /// This value should be zero for an image because COFF debugging information is deprecated.
    pub pointer_to_symbol_table: StructField<u32>,
    /// The number of entries in the symbol table.
    /// This data can be used to locate the string table, which immediately follows the symbol table.
    /// This value should be zero for an image because COFF debugging information is deprecated.
    pub number_of_symbols: StructField<u32>,
    /// The size of the [`OptionalHeader`](crate::header::optional_header::OptionalHeader), which is required for executable files but not for object files.
    /// This value should be zero for an object file.
    pub size_of_optional_header: StructField<u16>,
    /// The flags that indicate the attributes of the file. For specific flag values, see [`characteristics`](crate::header::characteristics)
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
        let _ = writeln!(f, "Timestamp, {:?}", self.time_date_stamp);
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
