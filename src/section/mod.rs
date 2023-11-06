/// The section should not be padded to the next boundary. This flag is obsolete and is replaced by [`IMAGE_SCN_ALIGN_1BYTES`]. Valid only for object files.
pub const IMAGE_SCN_TYPE_NO_PAD: u32 = 0x00000008;
/// The section contains executable code.
pub const IMAGE_SCN_CNT_CODE: u32 = 0x00000020;
/// The section contains initialized data.
pub const IMAGE_SCN_CNT_INITIALIZED_DATA: u32 = 0x00000040;
/// The section contains uninitialized data.
pub const IMAGE_SCN_CNT_UNINITIALIZED_DATA: u32 = 0x00000080;
/// Reserved for future use.
pub const IMAGE_SCN_LNK_OTHER: u32 = 0x00000100;
/// The section contains comments or other information. The `.drectve` section has this type. Valid only for object files
pub const IMAGE_SCN_LNK_INFO: u32 = 0x00000200;
/// The section will not become part of the image. Valid only for object files.
pub const IMAGE_SCN_LNK_REMOVE: u32 = 0x00000800;
/// The section contains COMDAT data. For more information, see [COMDAT Sections](https://learn.microsoft.com/en-us/windows/win32/debug/pe-format#comdat-sections-object-only). Valid only for object files
pub const IMAGE_SCN_LNK_COMDAT: u32 = 0x00001000;
/// The section contains data referenced through the global pointer (GP).
pub const IMAGE_SCN_GPREL: u32 = 0x00008000;
/// Align data on a 1-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_1BYTES: u32 = 0x00100000;
/// Align data on a 2-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_2BYTES: u32 = 0x00200000;
/// Align data on a 4-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_4BYTES: u32 = 0x00300000;
/// Align data on an 8-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_8BYTES: u32 = 0x00400000;
/// Align data on a 16-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_16BYTES: u32 = 0x00500000;
/// Align data on a 32-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_32BYTES: u32 = 0x00600000;
/// Align data on a 64-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_64BYTES: u32 = 0x00700000;
/// Align data on a 128-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_128BYTES: u32 = 0x00800000;
/// Align data on a 256-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_256BYTES: u32 = 0x00900000;
/// Align data on a 512-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_512BYTES: u32 = 0x00A00000;
/// Align data on a 1024-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_1024BYTES: u32 = 0x00B00000;
/// Align data on a 2048-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_2048BYTES: u32 = 0x00C00000;
/// Align data on a 4096-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_4096BYTES: u32 = 0x00D00000;
/// Align data on an 8192-byte boundary. Valid only for object files.
pub const IMAGE_SCN_ALIGN_8192BYTES: u32 = 0x00E00000;
/// The section contains extended relocations.
pub const IMAGE_SCN_LNK_NRELOC_OVFL: u32 = 0x01000000;
/// The section can be discarded as needed.
pub const IMAGE_SCN_MEM_DISCARDABLE: u32 = 0x02000000;
/// The section cannot be cached.
pub const IMAGE_SCN_MEM_NOT_CACHED: u32 = 0x04000000;
/// The section is not pageable.
pub const IMAGE_SCN_MEM_NOT_PAGED: u32 = 0x08000000;
/// The section can be shared in memory.
pub const IMAGE_SCN_MEM_SHARED: u32 = 0x10000000;
/// The section can be executed as code.
pub const IMAGE_SCN_MEM_EXECUTE: u32 = 0x20000000;
/// The section can be read.
pub const IMAGE_SCN_MEM_READ: u32 = 0x40000000;
/// The section can be written to.
pub const IMAGE_SCN_MEM_WRITE: u32 = 0x80000000;

pub struct SectionFlags(u32);

pub struct Section {
    pub name: String,
    pub virtual_size: u32,
    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub pointer_to_relocations: u32,
    pub pointer_to_linenumbers: u32,
    pub number_of_relocations: u16,
    pub number_of_linenumbers: u16,
    pub section_flags: SectionFlags,
}
