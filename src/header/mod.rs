pub mod characteristics;
pub mod dll_characteristics;
pub mod machine_types;
pub mod section_flags;
pub mod win_subsystem;

use crate::struct_parse::StructField;
use std::{
    fmt::{self, Debug, Display},
    io::Cursor,
    ops::Add,
};

/// The file is an executable image of 32-bit application
pub const IMAGE_NT_OPTIONAL_HDR32_MAGIC: [u8; 2] = [0x0B, 0x01];
/// The file is an executable image of 64-bit application
pub const IMAGE_NT_OPTIONAL_HDR64_MAGIC: [u8; 2] = [0x0B, 0x02];
/// The file is a ROM image.
pub const IMAGE_ROM_OPTIONAL_HDR_MAGIC: [u8; 2] = [0x07, 0x01];
/// Size of COFF File Header
pub const FILE_HEADER_SIZE: u64 = 20;

#[derive(Debug)]
pub enum ImageType {
    /// Represents 32-bit PE image
    Image32 = 0x010B,
    /// Represents 64-bit PE image
    Image64 = 0x020B,
    /// Represents ROM PE Image
    ImageRom = 0x0107,
}

impl Display for ImageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

pub mod file_header;
pub mod optioonal_header;

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
    offset: u64,
    buffer: Cursor<Vec<u8>>,
}

impl Section {
    /// An 8-byte, null-padded UTF-8 encoded string.
    /// If the string is exactly 8 characters long, there is no terminating null.
    /// For longer names, this field contains a slash (/) that is followed by an ASCII representation of a decimal number that is an offset into the string table.
    /// Executable images do not use a string table and do not support section names longer than 8 characters.
    /// Long names in object files are truncated if they are emitted to an executable file.
    pub fn name(&self) -> StructField<String> {
        todo!()
    }

    /// The total size of the section when loaded into memory.
    /// If this value is greater than `size_of_raw_data`, the section is zero-padded.
    /// This field is valid only for executable images and should be set to zero for object files.
    pub fn virtual_size(&self) -> u32 {
        todo!()
    }

    /// For executable images, the address of the first byte of the section relative to the image base when the section is loaded into memory.
    /// For object files, this field is the address of the first byte before relocation is applied; for simplicity, compilers should set this to zero.
    /// Otherwise, it is an arbitrary value that is subtracted from offsets during relocation.
    pub fn virtual_address(&self, image_base: VirtualAddress) -> RelativeVirtualAddress {
        todo!()
    }

    /// The size of the section (for object files) or the size of the initialized data on disk (for image files).
    /// For executable images, this must be a multiple of [`file_alignment`](OptionalHeader#structfield.file_alignment) from the [`OptionalHeader`].
    /// If this is less than `virtual_size`, the remainder of the section is zero-filled.
    /// Because the `size_of_raw_data` field is rounded but the `virtual_size` field is not, it is possible for `size_of_raw_data` to be greater than `virtual_size` as well.
    /// When a section contains only uninitialized data, this field should be zero.
    pub fn size_of_raw_data(&self) -> u32 {
        todo!()
    }

    /// The file pointer to the first page of the section within the COFF file.
    /// For executable images, this must be a multiple of [`file_alignment`](OptionalHeader#structfield.file_alignment) from the [`OptionalHeader`].
    /// For object files, the value should be aligned on a 4-byte boundary for best performance.
    /// When a section contains only uninitialized data, this field should be zero.
    pub fn pointer_to_raw_data(&self) -> u64 {
        todo!()
    }

    /// The file pointer to the beginning of relocation entries for the section.
    /// This is set to zero for executable images or if there are no relocations.
    pub fn pointer_to_relocations(&self) -> u64 {
        todo!()
    }

    /// The file pointer to the beginning of line-number entries for the section.
    /// This is set to zero if there are no COFF line numbers.
    /// This value should be zero for an image because COFF debugging information is deprecated.
    pub fn pointer_to_linenumbers(&self) -> u64 {
        todo!()
    }

    /// The number of relocation entries for the section.
    /// This is set to zero for executable images.
    pub fn number_of_relocations(&self) -> u16 {
        todo!()
    }

    /// The number of line-number entries for the section.
    /// This value should be zero for an image because COFF debugging information is deprecated.
    pub fn number_of_linenumbers(&self) -> u16 {
        todo!()
    }

    /// The [flags](section_flags) that describe the characteristics of the section.
    pub fn characteristics(&self) -> [u8; 4] {
        todo!()
    }
}

/// Relavive virtual address (RVA)
///
/// In an image file, this is the address of an item after it is loaded into memory, with the base address of the image file subtracted from it.
/// The RVA of an item almost always differs from its position within the file on disk (file pointer).
/// In an object file, an RVA is less meaningful because memory locations are not assigned.
/// In this case, an RVA would be an address within a section (described later in this table), to which a relocation is later applied during linking.
/// For simplicity, a compiler should just set the first RVA in each section to zero.
#[derive(Debug)]
pub struct RelativeVirtualAddress {
    addr: VirtualAddress,
    image_base: VirtualAddress,
}

impl Display for RelativeVirtualAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// Virtual address (VA)
///
/// Same as [RVA](RelativeVirtualAddress), except that the base address of the image file is not subtracted.
/// The address is called a VA because Windows creates a distinct VA space for each process, independent of physical memory.
/// For almost all purposes, a VA should be considered just an address.
/// A VA is not as predictable as an [RVA](RelativeVirtualAddress) because the loader might not load the image at its preferred location.
#[derive(Debug)]
pub struct VirtualAddress {
    addr: u64,
}

impl Add for VirtualAddress {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let addr = self.addr + rhs.addr;
        Self { addr }
    }
}

impl From<RelativeVirtualAddress> for VirtualAddress {
    fn from(value: RelativeVirtualAddress) -> Self {
        let addr: u64 = (value.addr + value.image_base).into();
        Self { addr }
    }
}

impl From<VirtualAddress> for u64 {
    fn from(value: VirtualAddress) -> Self {
        value.addr
    }
}

impl From<u64> for VirtualAddress {
    fn from(value: u64) -> Self {
        VirtualAddress { addr: value }
    }
}

impl From<u32> for VirtualAddress {
    fn from(value: u32) -> Self {
        Self { addr: value.into() }
    }
}

impl From<[u8; 4]> for VirtualAddress {
    fn from(value: [u8; 4]) -> Self {
        Self {
            addr: u32::from_le_bytes(value).into(),
        }
    }
}

impl From<[u8; 8]> for VirtualAddress {
    fn from(value: [u8; 8]) -> Self {
        Self {
            addr: u64::from_le_bytes(value),
        }
    }
}
