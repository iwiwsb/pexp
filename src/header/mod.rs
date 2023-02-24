use crate::{machine_types::Machine, section_flags::SectionFlags};
use chrono::{DateTime, TimeZone, Utc};
use std::{
    fmt::Display,
    io::{self, Read, Seek, SeekFrom},
};

/// The file is an executable image of 32-bit application
pub const IMAGE_NT_OPTIONAL_HDR32_MAGIC: [u8; 2] = [0x0B, 0x01];
/// The file is an executable image of 64-bit application
pub const IMAGE_NT_OPTIONAL_HDR64_MAGIC: [u8; 2] = [0x0B, 0x02];
/// The file is a ROM image.
pub const IMAGE_ROM_OPTIONAL_HDR_MAGIC: [u8; 2] = [0x07, 0x01];
/// Size of COFF File Header
pub const FILE_HEADER_SIZE: u64 = 20;

pub enum ImageType {
    /// Represents 32-bit PE image
    Image32 = 0x010B,
    /// Represents 64-bit PE image
    Image64 = 0x020B,
    /// Represents ROM PE Image
    ImageRom = 0x0107,
}

/// COFF File Header structure
#[derive(Debug)]
pub struct FileHeader {
    machine: [u8; 2],
    number_of_sections: [u8; 2],
    time_date_stamp: [u8; 4],
    pointer_to_symbol_table: [u8; 4],
    number_of_symbols: [u8; 4],
    size_of_optional_header: [u8; 2],
    characteristics: [u8; 2],
}

impl FileHeader {
    pub fn new<R: Read + Seek>(reader: &mut R, file_header_offset: u64) -> Self {
        read_file_header(reader, file_header_offset).unwrap()
    }

    fn machine(&self) -> Machine {
        Machine::from(self.machine)
    }

    fn number_of_sections(&self) -> u16 {
        u16::from_le_bytes(self.number_of_sections)
    }

    fn time_date_stamp(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(u32::from_le_bytes(self.time_date_stamp) as i64, 0)
            .unwrap()
    }

    fn pointer_to_symbol_table(&self) -> u32 {
        u32::from_le_bytes(self.pointer_to_symbol_table)
    }

    fn number_of_symbols(&self) -> u32 {
        u32::from_le_bytes(self.number_of_symbols)
    }

    fn size_of_optional_header(&self) -> u16 {
        u16::from_le_bytes(self.size_of_optional_header)
    }

    fn characteristics(&self) -> u16 {
        u16::from_le_bytes(self.characteristics)
    }
}

impl Display for FileHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Machine: {0}\n", self.machine()))
            .unwrap();
        f.write_fmt(format_args!(
            "Number of sections: {0}\n",
            self.number_of_sections()
        ))
        .unwrap();
        f.write_fmt(format_args!("Timestamp: {0}\n", self.time_date_stamp()))
            .unwrap();
        f.write_fmt(format_args!(
            "Pointer to symbol table: {0:X}\n",
            self.pointer_to_symbol_table()
        ))
        .unwrap();
        f.write_fmt(format_args!(
            "Number of symbols: {0}\n",
            self.number_of_symbols()
        ))
        .unwrap();
        f.write_fmt(format_args!(
            "Size of optional header: {0}\n",
            self.size_of_optional_header()
        ))
        .unwrap();
        f.write_fmt(format_args!(
            "Characteristics: {0:X}\n",
            self.characteristics()
        ))
    }
}

/// Optional Header structure
#[derive(Debug)]
pub struct OptionalHeader {
    magic: [u8; 2],
    major_linker_version: [u8; 1],
    minor_linker_version: [u8; 1],
    size_of_code: [u8; 4],
    size_of_initialized_data: [u8; 4],
    size_of_uninitialized_data: [u8; 4],
    address_of_entry_point: [u8; 4],
    base_of_code: [u8; 4],
    base_of_data: Option<[u8; 4]>,
    image_base: [u8; 8],
    section_alignment: [u8; 4],
    file_alignment: [u8; 4],
    major_operating_system_version: [u8; 2],
    minor_operating_system_version: [u8; 2],
    major_image_version: [u8; 2],
    minor_image_version: [u8; 2],
    major_subsystem_version: [u8; 2],
    minor_subsystem_version: [u8; 2],
    win32_version_value: [u8; 4],
    size_of_image: [u8; 4],
    size_of_headers: [u8; 4],
    check_sum: [u8; 4],
    subsystem: [u8; 2],
    dll_characteristics: [u8; 2],
    size_of_stack_reserve: [u8; 8],
    size_of_stack_commit: [u8; 8],
    size_of_heap_reserve: [u8; 8],
    size_of_heap_commit: [u8; 8],
    loader_flags: [u8; 4],
    number_of_rva_and_sizes: [u8; 4],
    data_directories: Vec<DataDir>,
}

/// Iterator over data directories
pub type DataDirectories<'a> = std::slice::Iter<'a, DataDir>;

impl OptionalHeader {
    pub fn new<R: Read + Seek>(reader: &mut R, opt_header_offset: u64) -> Self {
        read_optional_header(reader, opt_header_offset).unwrap()
    }

    pub fn image_type(&self) -> ImageType {
        match self.magic {
            IMAGE_NT_OPTIONAL_HDR32_MAGIC => ImageType::Image32,
            IMAGE_NT_OPTIONAL_HDR64_MAGIC => ImageType::Image64,
            IMAGE_ROM_OPTIONAL_HDR_MAGIC => ImageType::ImageRom,
            _ => panic!(),
        }
    }

    pub fn number_of_rva_and_sizes(&self) -> u32 {
        u32::from_le_bytes(self.number_of_rva_and_sizes)
    }

    pub fn data_directories(&self) -> DataDirectories {
        self.data_directories.iter()
    }
}

/// Data directory structure
#[derive(Debug)]
pub struct DataDir {
    virtual_address: [u8; 4],
    size: [u8; 4],
}

impl DataDir {
    pub fn new<R: Read + Seek>(reader: &mut R, data_dir_offset: u64) -> Self {
        read_data_dir(reader, data_dir_offset).unwrap()
    }
}

/// Section header structure
///
/// The basic unit of code or data within a PE or COFF file.
/// For example, all code in an object file can be combined within a single section or (depending on compiler behavior) each function can occupy its own section.
/// With more sections, there is more file overhead, but the linker is able to link in code more selectively.
/// A section is similar to a segment in Intel 8086 architecture.
/// All the raw data in a section must be loaded contiguously.
/// In addition, an image file can contain a number of sections, such as `.tls` or `.reloc` , which have special purposes.
#[derive(Debug)]
pub struct Section {
    name: [u8; 8],
    virtual_size: [u8; 4],
    virtual_address: [u8; 4],
    size_of_raw_data: [u8; 4],
    pointer_to_raw_data: [u8; 4],
    pointer_to_relocations: [u8; 4],
    pointer_to_linenumbers: [u8; 4],
    number_of_relocations: [u8; 2],
    number_of_linenumbers: [u8; 2],
    characteristics: [u8; 4],
}

impl Section {
    pub fn name(&self) -> String {
        self.name.iter().map(|&b| b as char).collect()
    }

    pub fn characteristics(&self) -> SectionFlags {
        todo!()
    }
}

fn read_file_header<R: Read + Seek>(
    reader: &mut R,
    file_header_offset: u64,
) -> io::Result<FileHeader> {
    let mut machine = [0u8; 2];
    let mut number_of_sections = [0u8; 2];
    let mut time_date_stamp = [0u8; 4];
    let mut pointer_to_symbol_table = [0u8; 4];
    let mut number_of_symbols = [0u8; 4];
    let mut size_of_optional_header = [0u8; 2];
    let mut characteristics = [0u8; 2];

    reader.seek(SeekFrom::Start(file_header_offset))?;
    reader.read_exact(&mut machine)?;
    reader.read_exact(&mut number_of_sections)?;
    reader.read_exact(&mut time_date_stamp)?;
    reader.read_exact(&mut pointer_to_symbol_table)?;
    reader.read_exact(&mut number_of_symbols)?;
    reader.read_exact(&mut size_of_optional_header)?;
    reader.read_exact(&mut characteristics)?;

    Ok(FileHeader {
        machine,
        number_of_sections,
        time_date_stamp,
        pointer_to_symbol_table,
        number_of_symbols,
        size_of_optional_header,
        characteristics,
    })
}

fn read_optional_header<R: Read + Seek>(
    reader: &mut R,
    opt_header_offset: u64,
) -> io::Result<OptionalHeader> {
    let mut magic = [0u8; 2];
    let mut major_linker_version = [0u8; 1];
    let mut minor_linker_version = [0u8; 1];
    let mut size_of_code = [0u8; 4];
    let mut size_of_initialized_data = [0u8; 4];
    let mut size_of_uninitialized_data = [0u8; 4];
    let mut address_of_entry_point = [0u8; 4];
    let mut base_of_code = [0u8; 4];
    let mut base_of_data: Option<[u8; 4]> = Some([0u8; 4]);
    let mut image_base = [0u8; 8];
    let mut section_alignment = [0u8; 4];
    let mut file_alignment = [0u8; 4];
    let mut major_operating_system_version = [0u8; 2];
    let mut minor_operating_system_version = [0u8; 2];
    let mut major_image_version = [0u8; 2];
    let mut minor_image_version = [0u8; 2];
    let mut major_subsystem_version = [0u8; 2];
    let mut minor_subsystem_version = [0u8; 2];
    let mut win32_version_value = [0u8; 4];
    let mut size_of_image = [0u8; 4];
    let mut size_of_headers = [0u8; 4];
    let mut check_sum = [0u8; 4];
    let mut subsystem = [0u8; 2];
    let mut dll_characteristics = [0u8; 2];
    let mut size_of_stack_reserve = [0u8; 8];
    let mut size_of_stack_commit = [0u8; 8];
    let mut size_of_heap_reserve = [0u8; 8];
    let mut size_of_heap_commit = [0u8; 8];
    let mut loader_flags = [0u8; 4];
    let mut number_of_rva_and_sizes = [0u8; 4];
    let mut data_directories: Vec<DataDir> = Vec::new();

    reader.seek(SeekFrom::Start(opt_header_offset))?;
    reader.read_exact(&mut magic)?;
    reader.read_exact(&mut major_linker_version)?;
    reader.read_exact(&mut minor_linker_version)?;
    reader.read_exact(&mut size_of_code)?;
    reader.read_exact(&mut size_of_initialized_data)?;
    reader.read_exact(&mut size_of_uninitialized_data)?;
    reader.read_exact(&mut address_of_entry_point)?;
    reader.read_exact(&mut base_of_code)?;
    base_of_data = match magic {
        IMAGE_NT_OPTIONAL_HDR32_MAGIC | IMAGE_ROM_OPTIONAL_HDR_MAGIC => {
            let mut buf = [0u8; 4];
            reader.read_exact(&mut buf)?;
            Some(buf)
        }
        IMAGE_NT_OPTIONAL_HDR64_MAGIC => None,
        _ => panic!(),
    };
    reader.read_exact(&mut image_base)?;
    reader.read_exact(&mut section_alignment)?;
    reader.read_exact(&mut file_alignment)?;
    reader.read_exact(&mut major_operating_system_version)?;
    reader.read_exact(&mut minor_operating_system_version)?;
    reader.read_exact(&mut major_image_version)?;
    reader.read_exact(&mut minor_image_version)?;
    reader.read_exact(&mut major_subsystem_version)?;
    reader.read_exact(&mut minor_subsystem_version)?;
    reader.read_exact(&mut win32_version_value)?;
    reader.read_exact(&mut size_of_image)?;
    reader.read_exact(&mut size_of_headers)?;
    reader.read_exact(&mut check_sum)?;
    reader.read_exact(&mut subsystem)?;
    reader.read_exact(&mut dll_characteristics)?;
    reader.read_exact(&mut size_of_stack_reserve)?;
    reader.read_exact(&mut size_of_stack_commit)?;
    reader.read_exact(&mut size_of_heap_reserve)?;
    reader.read_exact(&mut size_of_heap_commit)?;
    reader.read_exact(&mut loader_flags)?;
    reader.read_exact(&mut number_of_rva_and_sizes)?;

    let data_dir_offset = reader.seek(SeekFrom::Current(0))?;
    for _ in 0..u32::from_le_bytes(number_of_rva_and_sizes) {
        let value = read_data_dir(reader, data_dir_offset)?;
        data_directories.push(value);
    }

    Ok(OptionalHeader {
        magic,
        major_linker_version,
        minor_linker_version,
        size_of_code,
        size_of_initialized_data,
        size_of_uninitialized_data,
        address_of_entry_point,
        base_of_code,
        base_of_data,
        image_base,
        section_alignment,
        file_alignment,
        major_operating_system_version,
        minor_operating_system_version,
        major_image_version,
        minor_image_version,
        major_subsystem_version,
        minor_subsystem_version,
        win32_version_value,
        size_of_image,
        size_of_headers,
        check_sum,
        subsystem,
        dll_characteristics,
        size_of_stack_reserve,
        size_of_stack_commit,
        size_of_heap_reserve,
        size_of_heap_commit,
        loader_flags,
        number_of_rva_and_sizes,
        data_directories,
    })
}

fn read_data_dir<R: Read + Seek>(reader: &mut R, data_dir_offset: u64) -> io::Result<DataDir> {
    let mut virtual_address: [u8; 4] = [0u8; 4];
    let mut size: [u8; 4] = [0u8; 4];
    reader.seek(SeekFrom::Start(data_dir_offset))?;
    reader.read_exact(&mut virtual_address)?;
    reader.read_exact(&mut size)?;
    Ok(DataDir {
        virtual_address,
        size,
    })
}
