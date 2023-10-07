pub mod characteristics;
pub mod dll_characteristics;
pub mod file_header;
pub mod machine_types;
pub mod optional_header;
pub mod section_flags;
pub mod section_header;
pub mod win_subsystem;

use std::fmt::{self, Debug, Display};

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

impl TryFrom<u16> for ImageType {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x010B => Ok(ImageType::Image32),
            0x020B => Ok(ImageType::Image64),
            0x0107 => Ok(ImageType::ImageRom),
            _ => panic!(),
        }
    }
}

impl TryFrom<[u8; 2]> for ImageType {
    type Error = &'static str;

    fn try_from(value: [u8; 2]) -> Result<Self, Self::Error> {
        let x = u16::from_le_bytes(value);
        ImageType::try_from(x)
    }
}

impl Display for ImageType {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    addr: u64,
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
