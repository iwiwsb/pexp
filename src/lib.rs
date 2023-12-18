use chrono::NaiveDateTime;
use std::io::{Read, Seek};

#[derive(Debug)]
struct StructField<T, const N: usize> {
    offset: u64,
    name: String,
    raw_bytes: [u8; N],
    value: T,
}

fn read_file_header<R: Read + Seek>(reader: &mut R) -> FileHeader {
    let mut machine = [0u8; 2];
    let mut number_of_sections = [0u8; 2];
    let mut time_date_stamp = [0u8; 4];
    let mut pointer_to_symbol_table = [0u8; 4];
    let mut number_of_symbols = [0u8; 4];
    let mut size_of_optional_header = [0u8; 2];
    let mut characteristics = [0u8; 2];

    let _ = reader.read_exact(&mut machine);
    let _ = reader.read_exact(&mut number_of_sections);
    let _ = reader.read_exact(&mut time_date_stamp);
    let _ = reader.read_exact(&mut pointer_to_symbol_table);
    let _ = reader.read_exact(&mut number_of_symbols);
    let _ = reader.read_exact(&mut size_of_optional_header);
    let _ = reader.read_exact(&mut characteristics);

    let file_header_raw = FileHeaderRaw {
        machine,
        number_of_sections,
        time_date_stamp,
        pointer_to_symbol_table,
        number_of_symbols,
        size_of_optional_header,
        characteristics,
    };

    let offset = reader.stream_position().expect("Stream position should working");

    FileHeader {
        offset,
        file_header_raw,
    }
}

#[derive(Debug)]
struct FileHeaderRaw {
    machine: [u8; 2],
    number_of_sections: [u8; 2],
    time_date_stamp: [u8; 4],
    pointer_to_symbol_table: [u8; 4],
    number_of_symbols: [u8; 4],
    size_of_optional_header: [u8; 2],
    characteristics: [u8; 2],
}

#[derive(Debug)]
struct FileHeader {
    offset: u64,
    file_header_raw: FileHeaderRaw,
}

impl FileHeader {
    pub fn get_offset(&self) -> u64 {
        self.offset
    }

    pub fn machine(&self) -> u16 {
        u16::from_le_bytes(self.file_header_raw.machine)
    }

    pub fn number_of_sections(&self) -> u16 {
        u16::from_le_bytes(self.file_header_raw.number_of_sections)
    }

    pub fn time_date_stamp(&self) -> u32 {
        u32::from_le_bytes(self.file_header_raw.time_date_stamp)
    }

    pub fn pointer_to_symbol_table(&self) -> u32 {
        u32::from_le_bytes(self.file_header_raw.pointer_to_symbol_table)
    }

    pub fn number_of_symbols(&self) -> u32 {
        u32::from_le_bytes(self.file_header_raw.number_of_symbols)
    }

    pub fn size_of_optional_header(&self) -> u16 {
        u16::from_le_bytes(self.file_header_raw.size_of_optional_header)
    }

    pub fn characteristics(&self) -> u16 {
        u16::from_le_bytes(self.file_header_raw.characteristics)
    }
}

#[derive(Debug)]
struct FileHeaderWrapper {
    file_header: FileHeader,
}

impl FileHeaderWrapper {
    fn machine(&self) -> StructField<Machine, 2> {
        let offset = self.file_header.offset;
        let name = String::from("Machine");
        let raw_bytes = self.file_header.file_header_raw.machine;
        todo!()
    }

    fn number_of_sections(&self) -> StructField<u16, 2> {
        let offset = self.file_header.offset + 2;
        let name = String::from("Num. of sections");
        let raw_bytes = self.file_header.file_header_raw.number_of_sections;
        let value = self.file_header.number_of_sections();
        StructField {
            offset,
            name,
            raw_bytes,
            value,
        }
    }

    fn time_date_stamp(&self) -> StructField<NaiveDateTime, 4> {
        let offset = self.file_header.offset + 4;
        let name = String::from("Time date stamp");
        let raw_bytes = self.file_header.file_header_raw.time_date_stamp;
        let value = NaiveDateTime::from_timestamp(self.file_header.time_date_stamp() as i64, 0);
        StructField {
            offset,
            name,
            raw_bytes,
            value,
        }
    }

    fn pointer_to_symbol_table(&self) -> StructField<u32, 4> {
        let offset = self.file_header.offset + 8;
        let name = String::from("Pointer to symbol table");
        let raw_bytes = self.file_header.file_header_raw.pointer_to_symbol_table;
        let value = self.file_header.pointer_to_symbol_table();
        StructField {
            offset,
            name,
            raw_bytes,
            value,
        }
    }

    fn number_of_symbols(&self) -> StructField<u32, 4> {
        let offset = self.file_header.offset + 12;
        let name = String::from("Number of symbols");
        let raw_bytes = self.file_header.file_header_raw.number_of_symbols;
        let value = self.file_header.number_of_symbols();
        StructField {
            offset,
            name,
            raw_bytes,
            value,
        }
    }

    fn size_of_optional_header(&self) -> StructField<u16, 2> {
        let offset = self.file_header.offset + 16;
        let name = String::from("Size of optional header");
        let raw_bytes = self.file_header.file_header_raw.size_of_optional_header;
        let value = self.file_header.size_of_optional_header();
        StructField {
            offset,
            name,
            raw_bytes,
            value,
        }
    }

    fn characteristics(&self) -> StructField<Characteristics, 2> {
        let offset = self.file_header.offset + 18;
        todo!()
    }
}

enum Machine {

}

struct Characteristics;
