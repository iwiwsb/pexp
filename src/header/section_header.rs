use crate::header::RelativeVirtualAddress;
use crate::struct_parse::StructField;

/// Section header structure
///
/// The basic unit of code or data within a PE or COFF file.
/// For example, all code in an object file can be combined within a single section or (depending on compiler behavior) each function can occupy its own section.
/// With more sections, there is more file overhead, but the linker is able to link in code more selectively.
/// A section is similar to a segment in Intel 8086 architecture.
/// All the raw data in a section must be loaded contiguously.
/// In addition, an image file can contain a number of sections, such as `.tls` or `.reloc` , which have special purposes.
#[derive(Debug)]
pub struct SectionHeaderReader {
    offset: u64,
    buffer: Vec<u8>,
}

pub struct SectionHeader {
    /// An 8-byte, null-padded UTF-8 encoded string.
    /// If the string is exactly 8 characters long, there is no terminating null.
    /// For longer names, this field contains a slash (/) that is followed by an ASCII representation of a decimal number that is an offset into the string table.
    /// Executable images do not use a string table and do not support section names longer than 8 characters.
    /// Long names in object files are truncated if they are emitted to an executable file.
    pub name: String,
    /// The total size of the section when loaded into memory.
    /// If this value is greater than `size_of_raw_data`, the section is zero-padded.
    /// This field is valid only for executable images and should be set to zero for object files.
    pub virtual_size: u32,
    /// For executable images, the address of the first byte of the section relative to the image base when the section is loaded into memory.
    /// For object files, this field is the address of the first byte before relocation is applied; for simplicity, compilers should set this to zero.
    /// Otherwise, it is an arbitrary value that is subtracted from offsets during relocation.
    pub virtual_address: RelativeVirtualAddress,
    /// The size of the section (for object files) or the size of the initialized data on disk (for image files).
    /// For executable images, this must be a multiple of [`file_alignment`](crate::header::optional_header::OptionalHeader#structfield.file_alignment) from the [`OptionalHeader`](crate::header::optional_header::OptionalHeader).
    /// If this is less than `virtual_size`, the remainder of the section is zero-filled.
    /// Because the `size_of_raw_data` field is rounded but the `virtual_size` field is not, it is possible for `size_of_raw_data` to be greater than `virtual_size` as well.
    /// When a section contains only uninitialized data, this field should be zero.
    pub size_of_raw_data: u32,
    /// The file pointer to the first page of the section within the COFF file.
    /// For executable images, this must be a multiple of [`file_alignment`](crate::header::optional_header::OptionalHeader#structfield.file_alignment) from the [`OptionalHeader`](crate::header::optional_header::OptionalHeader).
    /// For object files, the value should be aligned on a 4-byte boundary for best performance.
    /// When a section contains only uninitialized data, this field should be zero.
    pub pointer_to_raw_data: u64,
    /// The file pointer to the beginning of relocation entries for the section.
    /// This is set to zero for executable images or if there are no relocations.
    pub pointer_to_relocations: u64,
    /// The file pointer to the beginning of line-number entries for the section.
    /// This is set to zero if there are no COFF line numbers.
    /// This value should be zero for an image because COFF debugging information is deprecated.
    pub pointer_to_linenumbers: u64,
    /// The number of relocation entries for the section.
    /// This is set to zero for executable images.
    pub number_of_relocations: u64,
    /// The number of line-number entries for the section.
    /// This value should be zero for an image because COFF debugging information is deprecated.
    pub number_of_linenumbers: u64,
    /// The [flags](crate::header::section_flags) that describe the characteristics of the section.
    pub characteristics: [u8; 4],
}

impl SectionHeaderReader {
    pub fn read_section(&self) -> SectionHeader {
        let _name = self.read_name();
        todo!()
    }

    fn read_name(&self) -> StructField<String> {
        todo!()
    }
}
