/// The section should not be padded to the next boundary. This flag is obsolete and is replaced by [`IMAGE_SCN_ALIGN_1BYTES`]. Valid only for object files.
pub const IMAGE_SCN_TYPE_NO_PAD: [u8; 4] = [0x08, 0x00, 0x00, 0x00];
/// The section contains executable code.
pub const IMAGE_SCN_CNT_CODE: [u8; 4] = [0x20, 0x00, 0x00, 0x00];
/// The section contains initialized data.
pub const IMAGE_SCN_CNT_INITIALIZED_DATA: [u8; 4] = [0x40, 0x00, 0x00, 0x00];
/// The section contains uninitialized data.
pub const IMAGE_SCN_CNT_UNINITIALIZED_DATA: [u8; 4] = [0x80, 0x00, 0x00, 0x00];
/// Reserved for future use.
pub const IMAGE_SCN_LNK_OTHER: [u8; 4] = [0x00, 0x01, 0x00, 0x00];
/// The section contains comments or other information. The `.drectve` section has this type. Valid only for object files
pub const IMAGE_SCN_LNK_INFO: [u8; 4] = [0x00, 0x02, 0x00, 0x00];
/// The section will not become part of the image. Valid only for object files.
pub const IMAGE_SCN_LNK_REMOVE: [u8; 4] = [0x00, 0x08, 0x00, 0x00];
/// The section contains COMDAT data. For more information, see [COMDAT Sections](https://learn.microsoft.com/en-us/windows/win32/debug/pe-format#comdat-sections-object-only). Valid only for object files
pub const IMAGE_SCN_LNK_COMDAT: [u8; 4] = [0x00, 0x10, 0x00, 0x00];
/// The section contains data referenced through the global pointer (GP).
pub const IMAGE_SCN_GPREL: [u8; 4] = [0x00, 0x80, 0x00, 0x00];
/// Align data on a 1-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_1BYTES: [u8; 4] = [0x00, 0x00, 0x10, 0x00];
/// Align data on a 2-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_2BYTES: [u8; 4] = [0x00, 0x00, 0x20, 0x00];
/// Align data on a 4-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_4BYTES: [u8; 4] = [0x00, 0x00, 0x30, 0x00];
/// Align data on an 8-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_8BYTES: [u8; 4] = [0x00, 0x00, 0x40, 0x00];
/// Align data on a 16-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_16BYTES: [u8; 4] = [0x00, 0x00, 0x50, 0x00];
/// Align data on a 32-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_32BYTES: [u8; 4] = [0x00, 0x00, 0x60, 0x00];
/// Align data on a 64-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_64BYTES: [u8; 4] = [0x00, 0x00, 0x70, 0x00];
/// Align data on a 128-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_128BYTES: [u8; 4] = [0x00, 0x00, 0x80, 0x00];
/// Align data on a 256-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_256BYTES: [u8; 4] = [0x00, 0x00, 0x90, 0x00];
/// Align data on a 512-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_512BYTES: [u8; 4] = [0x00, 0x00, 0xA0, 0x00];
/// Align data on a 1024-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_1024BYTES: [u8; 4] = [0x00, 0x00, 0xB0, 0x00];
/// Align data on a 2048-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_2048BYTES: [u8; 4] = [0x00, 0x00, 0xC0, 0x00];
/// Align data on a 4096-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_4096BYTES: [u8; 4] = [0x00, 0x00, 0xD0, 0x00];
/// Align data on an 8192-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_8192BYTES: [u8; 4] = [0x00, 0x00, 0xE0, 0x00];
/// The section contains extended relocations.
pub const IMAGE_SCN_LNK_NRELOC_OVFL: [u8; 4] = [0x00, 0x00, 0x00, 0x01];
/// The section can be discarded as needed.
pub const IMAGE_SCN_MEM_DISCARDABLE: [u8; 4] = [0x00, 0x00, 0x00, 0x02];
/// The section cannot be cached.
pub const IMAGE_SCN_MEM_NOT_CACHED: [u8; 4] = [0x00, 0x00, 0x00, 0x04];
/// The section is not pageable.
pub const IMAGE_SCN_MEM_NOT_PAGED: [u8; 4] = [0x00, 0x00, 0x00, 0x08];
/// The section can be shared in memory.
pub const IMAGE_SCN_MEM_SHARED: [u8; 4] = [0x00, 0x00, 0x00, 0x10];
/// The section can be executed as code.
pub const IMAGE_SCN_MEM_EXECUTE: [u8; 4] = [0x00, 0x00, 0x00, 0x20];
/// The section can be read.
pub const IMAGE_SCN_MEM_READ: [u8; 4] = [0x00, 0x00, 0x00, 0x40];
/// The section can be written to.
pub const IMAGE_SCN_MEM_WRITE: [u8; 4] = [0x00, 0x00, 0x00, 0x80];

pub struct SectionFlags([u8; 4]);
