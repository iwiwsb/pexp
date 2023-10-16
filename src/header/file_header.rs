use crate::struct_parse::StructField;
use std::fmt::{Debug, Display, Formatter};
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
        let offset = 0;
        let data = self.read_array(offset);
        StructField {
            abs_offset: offset + self.offset,
            data,
        }
    }

    pub fn read_machine(&mut self) -> StructField<[u8; 2]> {
        let offset = 4;
        let data = self.read_array(offset);
        StructField {
            abs_offset: offset + self.offset,
            data,
        }
    }

    pub fn read_number_of_sections(&mut self) -> StructField<[u8; 2]> {
        let offset = 6;
        let data = self.read_array(offset);
        StructField {
            abs_offset: offset + self.offset,
            data,
        }
    }

    pub fn read_time_date_stamp(&mut self) -> StructField<[u8; 4]> {
        let offset = 8;
        let data = self.read_array(offset);
        StructField {
            abs_offset: offset + self.offset,
            data,
        }
    }

    pub fn read_pointer_to_symbol_table(&mut self) -> StructField<[u8; 4]> {
        let offset = 12;
        let data = self.read_array(offset);
        StructField {
            abs_offset: offset + self.offset,
            data,
        }
    }

    pub fn read_number_of_symbols(&mut self) -> StructField<[u8; 4]> {
        let offset = 16;
        let data = self.read_array(offset);
        StructField {
            abs_offset: offset + self.offset,
            data,
        }
    }

    pub fn read_size_of_optional_header(&mut self) -> StructField<[u8; 2]> {
        let offset = 20;
        let data = self.read_array(offset);
        StructField {
            abs_offset: offset + self.offset,
            data,
        }
    }

    pub fn read_characteristics(&mut self) -> StructField<[u8; 2]> {
        let offset = 22;
        let data = self.read_array(offset);
        StructField {
            abs_offset: offset + self.offset,
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

impl Display for FileHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let signature_data = self.signature.data;
        let _ = writeln!(f, "File Header:");
        let _ = writeln!(
            f,
            "  {:#04X}  Signature: {:#04X} {:#04X} {:#04X} {:#04X}",
            self.signature.abs_offset,
            signature_data[0],
            signature_data[1],
            signature_data[2],
            signature_data[3]
        );
        let _ = writeln!(
            f,
            "  {:#04X}  Machine: {}",
            self.machine.abs_offset,
            self.machine.as_machine()
        );
        let _ = writeln!(
            f,
            "  {:#04X}  Number of sections: {}",
            self.number_of_sections.abs_offset,
            self.number_of_sections.as_u16_le()
        );
        let _ = writeln!(
            f,
            "  {:#04X}  Time and date: {}",
            self.time_date_stamp.abs_offset,
            self.time_date_stamp.as_datetime()
        );
        let _ = writeln!(
            f,
            "  {:#04X}  Pointer to symbol table: {}",
            self.pointer_to_symbol_table.abs_offset,
            self.pointer_to_symbol_table.as_u32_le()
        );
        let _ = writeln!(
            f,
            "  {:#X}  Number of symbols: {}",
            self.number_of_symbols.abs_offset,
            self.number_of_symbols.as_u32_le()
        );
        let _ = writeln!(
            f,
            "  {:#X}  Size of optional header: {} bytes",
            self.size_of_optional_header.abs_offset,
            self.size_of_optional_header.as_u16_le()
        );
        writeln!(
            f,
            "  {:#X}  Characteristics: {:#04X}",
            self.characteristics.abs_offset,
            self.characteristics.as_u16_le()
        )
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_file_header_reading() {
        let file_header = FileHeader {
            signature: StructField {
                abs_offset: 0,
                data: [b'P', b'E', 0, 0],
            },
            machine: StructField {
                abs_offset: 4,
                data: [0x64, 0x86],
            },
            number_of_sections: StructField {
                abs_offset: 6,
                data: [6, 0],
            },
            time_date_stamp: StructField {
                abs_offset: 8,
                data: [0x10, 0xC4, 0x40, 0x03],
            },
            pointer_to_symbol_table: StructField {
                abs_offset: 12,
                data: [0x00, 0x00, 0x00, 0x00],
            },
            number_of_symbols: StructField {
                abs_offset: 16,
                data: [0x00, 0x00, 0x00, 0x00],
            },
            size_of_optional_header: StructField {
                abs_offset: 20,
                data: [0xF0, 0x00],
            },
            characteristics: StructField {
                abs_offset: 22,
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
