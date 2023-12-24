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
        todo!()
    }
}

enum PEType {
    Object,
    Image,
    Unknown,
}

impl PEType {
    /// Returns `true` if the PE type is [`Object`].
    ///
    /// [`Object`]: PEType::Object
    fn is_object(&self) -> bool {
        match self {
            Self::Object => true,
            _ => false,
        }
    }

    /// Returns `true` if the PE type is [`Image`].
    ///
    /// [`Image`]: PEType::Image
    fn is_image(&self) -> bool {
        match self {
            Self::Image => true,
            _ => false,
        }
    }
}

enum ImageType {
    X32,
    X64,
    ROM,
}

impl ImageType {
    /// Returns `true` if the image type is [`X32`].
    ///
    /// [`X32`]: ImageType::X32
    fn is_x32(&self) -> bool {
        match self {
            Self::X32 => true,
            _ => false,
        }
    }

    /// Returns `true` if the image type is [`X64`].
    ///
    /// [`X64`]: ImageType::X64
    fn is_x64(&self) -> bool {
        match self {
            Self::X64 => true,
            _ => false,
        }
    }

    /// Returns `true` if the image type is [`ROM`].
    ///
    /// [`ROM`]: ImageType::ROM
    fn is_rom(&self) -> bool {
        match self {
            Self::ROM => true,
            _ => false,
        }
    }
}
