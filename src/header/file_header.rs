use crate::struct_parse::{ReadU16LE, ReadU32LE, StructField};
use std::fmt::Debug;
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
        let data = self.read_array(offset);
        StructField { offset, data }
    }

    pub fn read_machine(&mut self) -> StructField<[u8; 2]> {
        let offset = self.offset + 4;
        let data = self.read_array(offset);
        StructField { offset, data }
    }

    pub fn read_number_of_sections(&mut self) -> StructField<[u8; 2]> {
        let offset = self.offset + 6;
        let data = self.read_array(offset);
        StructField { offset, data }
    }

    pub fn read_time_date_stamp(&mut self) -> StructField<[u8; 4]> {
        let offset = self.offset + 8;
        let data = self.read_array(offset);
        StructField { offset, data }
    }

    pub fn read_pointer_to_symbol_table(&mut self) -> StructField<[u8; 4]> {
        let offset = self.offset + 12;
        let data = self.read_array(offset);
        StructField { offset, data }
    }

    pub fn read_number_of_symbols(&mut self) -> StructField<[u8; 4]> {
        let offset = self.offset + 16;
        let data = self.read_array(offset);
        StructField { offset, data }
    }

    pub fn read_size_of_optional_header(&mut self) -> StructField<[u8; 2]> {
        let offset = self.offset + 20;
        let data = self.read_array(offset);
        StructField { offset, data }
    }

    pub fn read_characteristics(&mut self) -> StructField<[u8; 2]> {
        let offset = self.offset + 22;
        let data = self.read_array(offset);
        StructField { offset, data }
    }

    fn read_array<const N: usize>(&mut self, offset: u64) -> [u8; N] {
        let pos = SeekFrom::Start(self.offset + offset);
        let _ = self.buffer.seek(pos);
        let mut buf = [0u8; N];
        let _ = self.buffer.read_exact(&mut buf);
        buf
    }
}

impl<R: Read + Seek> ReadU16LE for FileHeaderReader<R> {
    fn read_u16_le(&mut self, offset: u64) -> u16 {
        let offset = self.offset + offset;
        let buf = self.read_array(offset);
        let data = u16::from_le_bytes(buf);
        data
    }
}

impl<R: Read + Seek> ReadU32LE for FileHeaderReader<R> {
    fn read_u32_le(&mut self, offset: u64) -> u32 {
        let offset = self.offset + offset;
        let buf = self.read_array(offset);
        let data = u32::from_le_bytes(buf);
        data
    }
}

/// COFF File Header structure
#[derive(Debug, PartialEq)]
pub struct FileHeader {
    pub signature: StructField<[u8; 4]>,
    /// Identifies the type of target machine. For more information, see [`machine_types`](crate::header::machine_types).
    pub machine: StructField<[u8; 2]>,
    /// Indicates the size of the section table, which immediately follows the headers.
    pub number_of_sections: StructField<[u8; 2]>,
    /// The low 32 bits of the number of seconds since 00:00 January 1, 1970 (a C run-time time_t value), which indicates when the file was created.
    pub time_date_stamp: StructField<[u8; 4]>,
    /// The file offset of the COFF symbol table, or zero if no COFF symbol table is present.
    /// This value should be zero for an image because COFF debugging information is deprecated.
    pub pointer_to_symbol_table: StructField<[u8; 4]>,
    /// The number of entries in the symbol table.
    /// This data can be used to locate the string table, which immediately follows the symbol table.
    /// This value should be zero for an image because COFF debugging information is deprecated.
    pub number_of_symbols: StructField<[u8; 4]>,
    /// The size of the [`OptionalHeader`](crate::header::optional_header::OptionalHeader), which is required for executable files but not for object files.
    /// This value should be zero for an object file.
    pub size_of_optional_header: StructField<[u8; 2]>,
    /// The flags that indicate the attributes of the file. For specific flag values, see [`characteristics`](crate::header::characteristics)
    pub characteristics: StructField<[u8; 2]>,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_file_header_reading() {
        let file_header = FileHeader {
            signature: StructField {
                offset: 0,
                data: [b'P', b'E', 0, 0],
            },
            machine: StructField {
                offset: 4,
                data: [0x64, 0x86],
            },
            number_of_sections: StructField {
                offset: 6,
                data: [6, 0],
            },
            time_date_stamp: StructField {
                offset: 8,
                data: [0x10, 0xC4, 0x40, 0x03],
            },
            pointer_to_symbol_table: StructField {
                offset: 12,
                data: [0x00, 0x00, 0x00, 0x00],
            },
            number_of_symbols: StructField {
                offset: 16,
                data: [0x00, 0x00, 0x00, 0x00],
            },
            size_of_optional_header: StructField {
                offset: 20,
                data: [0xF0, 0x00],
            },
            characteristics: StructField {
                offset: 22,
                data: [0x22, 0x00],
            },
        };

        let buf: Vec<u8> = vec![
            0x50, 0x45, 0x00, 0x00, 0x64, 0x86, 0x06, 0x00, 0x10, 0xC4, 0x40, 0x03, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x00, 0x22, 0x00,
        ];
        let cur = Cursor::new(buf);
        let read_file_header = FileHeaderReader::new(0, cur).read_file_header();
        assert_eq!(read_file_header, file_header);
    }
}
