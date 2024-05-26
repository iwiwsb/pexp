use std::fmt;

pub mod file_header;
pub mod optional_header;

#[derive(Debug)]
pub struct StructField<T, const N: usize> {
    offset: u64,
    name: String,
    raw_bytes: [u8; N],
    value: T,
}

impl fmt::Display for StructField<u16, 2> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}\t{}\t{:?}\t{}", self.offset, self.name, self.raw_bytes, self.value)
    }
}

pub enum PEType {
    Object,
    Image,
    Unknown,
}

impl PEType {
    /// Returns `true` if the PE type is [`Object`].
    ///
    /// [`Object`]: PEType::Object
    pub fn is_object(&self) -> bool {
        matches!(self, Self::Object)
    }

    /// Returns `true` if the PE type is [`Image`].
    ///
    /// [`Image`]: PEType::Image
    pub fn is_image(&self) -> bool {
        matches!(self, Self::Image)
    }
}

pub enum ImageType {
    X32,
    X64,
    ROM,
}

impl ImageType {
    /// Returns `true` if the image type is [`X32`].
    ///
    /// [`X32`]: ImageType::X32
    pub fn is_x32(&self) -> bool {
        matches!(self, Self::X32)
    }

    /// Returns `true` if the image type is [`X64`].
    ///
    /// [`X64`]: ImageType::X64
    pub fn is_x64(&self) -> bool {
        matches!(self, Self::X64)
    }

    /// Returns `true` if the image type is [`ROM`].
    ///
    /// [`ROM`]: ImageType::ROM
    pub fn is_rom(&self) -> bool {
        matches!(self, Self::ROM)
    }
}
