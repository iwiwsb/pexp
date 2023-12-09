use pexp::ReadArray;
use std::fmt::{self, Debug, Display};
use std::io::Read;

/// The file is an executable image of 32-bit application
pub const IMAGE_NT_OPTIONAL_HDR32_MAGIC: u16 = 0x010B;
/// The file is an executable image of 64-bit application
pub const IMAGE_NT_OPTIONAL_HDR64_MAGIC: u16 = 0x020B;
/// The file is a ROM image.
pub const IMAGE_ROM_OPTIONAL_HDR_MAGIC: u16 = 0x0107;
/// Size of COFF File Header
pub const FILE_HEADER_SIZE: u64 = 28;

#[derive(Debug)]
pub enum ImageType {
    /// Represents 32-bit PE image
    Image32,
    /// Represents 64-bit PE image
    Image64,
    /// Represents ROM PE Image
    ImageRom,
    /// Represents that image type is unknown
    ImageUnknown,
}

impl From<u16> for ImageType {
    fn from(value: u16) -> Self {
        match value {
            IMAGE_NT_OPTIONAL_HDR32_MAGIC => ImageType::Image32,
            IMAGE_NT_OPTIONAL_HDR64_MAGIC => ImageType::Image64,
            IMAGE_ROM_OPTIONAL_HDR_MAGIC => ImageType::ImageRom,
            _ => ImageType::ImageUnknown,
        }
    }
}

impl From<[u8; 2]> for ImageType {
    fn from(value: [u8; 2]) -> Self {
        let x = u16::from_le_bytes(value);
        ImageType::from(x)
    }
}

impl Display for ImageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageType::Image32 => write!(f, "32-bit PE image"),
            ImageType::Image64 => write!(f, "64-bit PE image"),
            ImageType::ImageRom => write!(f, "ROM PE image"),
            ImageType::ImageUnknown => write!(f, "Unknown image"),
        }
    }
}

/// Relative virtual address (RVA)
///
/// In an image file, this is the address of an item after it is loaded into memory, with the base address of the image file subtracted from it.
/// The RVA of an item almost always differs from its position within the file on disk (file pointer).
/// In an object file, an RVA is less meaningful because memory locations are not assigned.
/// In this case, an RVA would be an address within a section (described later in this table), to which a relocation is later applied during linking.
/// For simplicity, a compiler should just set the first RVA in each section to zero.
#[derive(Debug)]
pub struct RelativeVirtualAddress {}

/// Virtual address (VA)
///
/// Same as [RVA](RelativeVirtualAddress), except that the base address of the image file is not subtracted.
/// The address is called a VA because Windows creates a distinct VA space for each process, independent of physical memory.
/// For almost all purposes, a VA should be considered just an address.
/// A VA is not as predictable as an [RVA](RelativeVirtualAddress) because the loader might not load the image at its preferred location.
#[derive(Debug)]
pub struct VirtualAddress {}

/// Standard fields that are defined for every implementation of COFF, including UNIX.
///
/// These fields contain general information that is useful for loading and running an executable file.
/// They are unchanged for the PE32+ format.
#[derive(Debug)]
pub struct OptHdrStdFields {
    /// Identifies the state of the image file.
    /// The most common number is `0x10B`, which identifies it as a 32-bit (PE32) executable file.
    /// `0x107` identifies it as a ROM image, and `0x20B` identifies it as a 64-bit (PE32+) executable file.
    pub magic: u16,

    /// The linker major version number.
    pub major_linker_version: u8,

    /// The linker minor version number.
    pub minor_linker_version: u8,

    /// The size of the code (`.text`) section, or the sum of all code sections if there are multiple sections.
    pub size_of_code: u32,

    /// The size of the initialized data section, or the sum of all such sections if there are multiple data sections.
    pub size_of_initialized_data: u32,

    /// The size of the uninitialized data section (`BSS`), or the sum of all such sections if there are multiple `BSS` sections.
    pub size_of_uninitialized_data: u32,

    /// The address of the entry point relative to the image base when the executable file is loaded into memory.
    /// For program images, this is the starting address.
    /// For device drivers, this is the address of the initialization function.
    /// An entry point is optional for DLLs.
    pub address_of_entry_point: u32,

    /// The address that is relative to the image base of the beginning-of-code section when it is loaded into memory.
    pub base_of_code: u32,
}

impl Display for OptHdrStdFields {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Magic: {}\nMajor linker version: {}\nMinor linker version: {}\nSize of code: {}\nSize of initialized data: {}\nSize of uninitialized data: {}\nAddress of entry point: {:#X}\nBase of code: {:#X}",
            self.magic,
            self.major_linker_version,
            self.minor_linker_version,
            self.size_of_code,
            self.size_of_initialized_data,
            self.size_of_uninitialized_data,
            self.address_of_entry_point,
            self.base_of_code
        )
    }
}

impl ReadArray for OptHdrStdFields {}

impl OptHdrStdFields {
    pub fn read_from<R: Read>(reader: &mut R) -> Self {
        let magic = u16::from_le_bytes(Self::read_array(reader));
        let major_linker_version = u8::from_le_bytes(Self::read_array(reader));
        let minor_linker_version = u8::from_le_bytes(Self::read_array(reader));
        let size_of_code = u32::from_le_bytes(Self::read_array(reader));
        let size_of_initialized_data = u32::from_le_bytes(Self::read_array(reader));
        let size_of_uninitialized_data = u32::from_le_bytes(Self::read_array(reader));
        let address_of_entry_point = u32::from_le_bytes(Self::read_array(reader));
        let base_of_code = u32::from_le_bytes(Self::read_array(reader));
        Self {
            magic,
            major_linker_version,
            minor_linker_version,
            size_of_code,
            size_of_initialized_data,
            size_of_uninitialized_data,
            address_of_entry_point,
            base_of_code,
        }
    }
}

/// Additional information that is required by the linker and loader in Windows.
#[derive(Debug)]
pub struct OptHdrWinSpecificFields {
    /// The preferred address of the first byte of image when loaded into memory; must be a multiple of 64 K.
    /// The default for DLLs is `0x10000000`.
    /// The default for Windows CE EXEs is `0x00010000`.
    /// The default for Windows NT, Windows 2000, Windows XP, Windows 95, Windows 98, and Windows Me is `0x00400000`.
    pub image_base: u64,

    /// The alignment (in bytes) of sections when they are loaded into memory.
    ///
    /// It must be greater than or equal to FileAlignment.
    /// The default is the page size for the architecture.
    pub section_alignment: u32,

    /// The alignment factor (in bytes) that is used to align the raw data of sections in the image file.
    ///
    /// The value should be a power of 2 between 512 and 64 K, inclusive.
    /// The default is 512.
    /// If the SectionAlignment is less than the architecture's page size, then FileAlignment must match SectionAlignment.
    pub file_alignment: u32,

    /// The major version number of the required operating system
    pub major_operating_system_version: u16,

    /// The minor version number of the required operating system.
    pub minor_operating_system_version: u16,

    /// The major version number of the image.
    pub major_image_version: u16,

    /// The minor version number of the image.
    pub minor_image_version: u16,

    /// The major version number of the subsystem.
    pub major_subsystem_version: u16,

    /// The minor version number of the subsystem.
    pub minor_subsystem_version: u16,

    /// Reserved, must be zero.
    pub win32_version_value: u32,

    /// The size (in bytes) of the image, including all headers, as the image is loaded in memory.
    /// It must be a multiple of `section_alignment`.
    pub size_of_image: u32,

    /// The combined size of an MS-DOS stub, PE header, and section headers rounded up to a multiple of `file_alignment`.
    pub size_of_headers: u32,

    /// The image file checksum.
    /// The algorithm for computing the checksum is incorporated into IMAGHELP.DLL.
    /// The following are checked for validation at load time: all drivers, any DLL loaded at boot time, and any DLL that is loaded into a critical Windows process.
    pub check_sum: u32,

    /// The subsystem that is required to run this image. For more information, see [`win_subsystem`](crate::header::win_subsystem) module.
    pub subsystem: WinSubSystem,

    /// See [`dll_characteristics`](crate::header::dll_characteristics) module.
    pub dll_characteristics: DllCharacteristics,

    /// The size of the stack to reserve. Only `size_of_stack_commit` is committed; the rest is made available one page at a time until the reserve size is reached.
    pub size_of_stack_reserve: u64,

    /// The size of the stack to commit.
    pub size_of_stack_commit: u64,

    /// The size of the local heap space to reserve. Only `size_of_heap_commit` is committed; the rest is made available one page at a time until the reserve size is reached.
    pub size_of_heap_reserve: u64,

    /// The size of the local heap space to commit.
    pub size_of_heap_commit: u64,

    /// Reserved, must be zero.
    pub loader_flags: u32,

    /// The number of data-directory entries in the remainder of the optional header. Each describes a location and size.
    pub number_of_rva_and_sizes: u32,
}

/// Optional Header structure
///
/// Every image file has an optional header that provides information to the loader.
/// This header is optional in the sense that some files (specifically, object files) do not have it.
/// For image files, this header is required.
/// An object file can have an optional header, but generally this header has no function in an object file except to increase its size.
/// Note that the size of the optional header is not fixed.
/// The [`size_of_optional_header`](crate::header::file_header::FileHeader#structfield.size_of_optional_header) field in the COFF header must be used
/// to validate that a probe into the file for a particular data directory does not go beyond [`size_of_optional_header`](crate::header::file_header::FileHeader#structfield.size_of_optional_header).
///
/// The first 8 fields of the optional header are standard fields that are defined for every implementation of COFF.
/// PE32 contains additional field `base_of_data`, which is absent in PE32+, following `base_of_code`.
/// These fields contain general information that is useful for loading and running an executable file. They are unchanged for the PE32+ format.
/// The next 21 fields are an extension to the COFF optional header format. They contain additional information that is required by the linker and loader in Windows.
#[derive(Debug)]
pub struct OptionalHeader {
    /// The first eight fields of the optional header are standard fields that are defined for every implementation of COFF.
    /// These fields contain general information that is useful for loading and running an executable file.
    pub std_fields: OptHdrStdFields,
    /// The address that is relative to the image base of the beginning-of-data section when it is loaded into memory.
    /// PE32 contains this additional field, which is absent in PE32+
    pub base_of_data: Option<u32>,
    /// Additional information that is required by the linker and loader in Windows.
    pub windows_specific_fields: OptHdrWinSpecificFields,
    /// Address/size pairs for special tables that are found in the image file and are used by the operating system (for example, the import table and the export table).
    /// Note that the number of directories is not fixed. Before looking for a specific directory,
    /// check the `number_of_rva_and_sizes` field.
    pub data_directories: Vec<DataDirectory>,
}

impl OptionalHeader {
    pub fn read_from<R: Read>(&self, reader: &mut R) -> Self {
        todo!()
    }
}

/// Optional Header ROM structure
///
#[derive(Debug)]
pub struct OptionalHeaderRom {
    pub std_fields: OptHdrStdFields,
    pub base_of_data: u32,
    pub base_of_bss: u32,
    pub gpr_mask: u32,
    pub cpr_mask: [u32; 4],
    pub gp_value: u32,
}

#[derive(Debug, Clone)]
pub enum DataDirectoryType {
    ExportTable,
    ImportTable,
    ResourceTable,
    ExceptionTable,
    CertificateTable,
    BaseRelocationTable,
    Debug,
    Architecture,
    GlobalPtr,
    TLSTable,
    LoadConfig,
    BoundImport,
    ImportAdressTable,
    DelayImportDescriptor,
    CLRHeader,
    Reserved,
}

/// Data Directory structure
///
/// Each data directory gives the address and size of a table or string that Windows uses.
/// These data directory entries are all loaded into memory so that the system can use them at run time.
#[derive(Debug)]
pub struct DataDirectory {
    /// The [`RVA`](crate::header::RelativeVirtualAddress) of the table
    pub virtual_address: u32,
    /// Size in bytes
    pub size: u32,
}

impl DataDirectory {
    fn read_from<R: Read>(reader: &mut R) -> Self {
        let virtual_address = u32::from_le_bytes(Self::read_array(reader));
        let size = u32::from_le_bytes(Self::read_array(reader));

        Self {
            virtual_address,
            size,
        }
    }
}

impl ReadArray for DataDirectory {}

use std::fmt::{Binary, LowerHex, UpperHex};

#[derive(Debug)]
pub struct DllCharacteristics {
    flags: [bool; 16],
}

impl DllCharacteristics {
    /// Image can handle a high entropy 64-bit virtual address space
    pub const IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA: u16 = 0x0020;
    /// DLL can be relocated at load time
    pub const IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE: u16 = 0x0040;
    /// Code Integrity checks are enforced
    pub const IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY: u16 = 0x0080;
    /// Image is NX compatible
    pub const IMAGE_DLLCHARACTERISTICS_NX_COMPAT: u16 = 0x0100;
    /// Isolation aware, but do not isolate the image
    pub const IMAGE_DLLCHARACTERISTICS_NO_ISOLATION: u16 = 0x0200;
    /// Does not use structured exception (SE) handling. No SE handler may be called in this image
    pub const IMAGE_DLLCHARACTERISTICS_NO_SEH: u16 = 0x0400;
    /// Do not bind the image
    pub const IMAGE_DLLCHARACTERISTICS_NO_BIND: u16 = 0x0800;
    /// Image must execute in an [AppContainer](https://learn.microsoft.com/en-us/windows/win32/secauthz/appcontainer-isolation)
    pub const IMAGE_DLLCHARACTERISTICS_APPCONTAINER: u16 = 0x1000;
    /// A WDM driver
    pub const IMAGE_DLLCHARACTERISTICS_WDM_DRIVER: u16 = 0x2000;
    /// Image supports [Control Flow Guard](https://learn.microsoft.com/en-us/windows/win32/secbp/control-flow-guard)
    pub const IMAGE_DLLCHARACTERISTICS_GUARD_CF: u16 = 0x4000;
    /// Terminal Server aware
    pub const IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE: u16 = 0x8000;

    pub fn to_bits(&self) -> u16 {
        (self.flags[0] as u16) << 15
            | (self.flags[1] as u16) << 14
            | (self.flags[2] as u16) << 13
            | (self.flags[3] as u16) << 12
            | (self.flags[4] as u16) << 11
            | (self.flags[5] as u16) << 10
            | (self.flags[6] as u16) << 9
            | (self.flags[7] as u16) << 8
            | (self.flags[8] as u16) << 7
            | (self.flags[9] as u16) << 6
            | (self.flags[10] as u16) << 5
            | (self.flags[11] as u16) << 4
            | (self.flags[12] as u16) << 3
            | (self.flags[13] as u16) << 2
            | (self.flags[14] as u16) << 1
            | (self.flags[15] as u16)
    }
}

impl From<u16> for DllCharacteristics {
    fn from(value: u16) -> Self {
        let mut flags = [false; 16];
        flags[0] = (value & 0x0001) == 0x0001;
        flags[1] = (value & 0x0002) == 0x0002;
        flags[2] = (value & 0x0004) == 0x0004;
        flags[3] = (value & 0x0008) == 0x0008;
        flags[4] = (value & 0x0010) == 0x0010;
        flags[5] = (value & Self::IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA)
            == Self::IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA;
        flags[6] = (value & Self::IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE)
            == Self::IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE;
        flags[7] = (value & Self::IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY)
            == Self::IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY;
        flags[8] = (value & Self::IMAGE_DLLCHARACTERISTICS_NX_COMPAT)
            == Self::IMAGE_DLLCHARACTERISTICS_NX_COMPAT;
        flags[9] = (value & Self::IMAGE_DLLCHARACTERISTICS_NO_ISOLATION)
            == Self::IMAGE_DLLCHARACTERISTICS_NO_ISOLATION;
        flags[10] = (value & Self::IMAGE_DLLCHARACTERISTICS_NO_SEH)
            == Self::IMAGE_DLLCHARACTERISTICS_NO_SEH;
        flags[11] = (value & Self::IMAGE_DLLCHARACTERISTICS_NO_BIND)
            == Self::IMAGE_DLLCHARACTERISTICS_NO_BIND;
        flags[12] = (value & Self::IMAGE_DLLCHARACTERISTICS_APPCONTAINER)
            == Self::IMAGE_DLLCHARACTERISTICS_APPCONTAINER;
        flags[13] = (value & Self::IMAGE_DLLCHARACTERISTICS_WDM_DRIVER)
            == Self::IMAGE_DLLCHARACTERISTICS_WDM_DRIVER;
        flags[14] = (value & Self::IMAGE_DLLCHARACTERISTICS_GUARD_CF)
            == Self::IMAGE_DLLCHARACTERISTICS_GUARD_CF;
        flags[15] = (value & Self::IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE)
            == Self::IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE;
        Self { flags }
    }
}

impl Binary for DllCharacteristics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:016b}", self.to_bits())
    }
}

impl UpperHex for DllCharacteristics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:02X}", self.to_bits())
    }
}

impl LowerHex for DllCharacteristics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:02x}", self.to_bits())
    }
}

#[derive(Debug)]
pub enum WinSubSystem {
    Unknown,
    Native,
    WindowsCUI,
    WindowsGUI,
    OS2CUI,
    PosixCUI,
    NativeWindows,
    WindowsCEGUI,
    EFIApplication,
    EFIBootServiceDriver,
    EFIRuntimeDriver,
    EFIROM,
    XBOX,
    WindowsBootApplication,
}

impl WinSubSystem {
    /// An unknown subsystem
    pub const IMAGE_SUBSYSTEM_UNKNOWN: u16 = 0;
    /// Device drivers and native Windows processes
    pub const IMAGE_SUBSYSTEM_NATIVE: u16 = 1;
    /// The Windows graphical user interface (GUI) subsystem
    pub const IMAGE_SUBSYSTEM_WINDOWS_GUI: u16 = 2;
    /// The Windows character subsystem
    pub const IMAGE_SUBSYSTEM_WINDOWS_CUI: u16 = 3;
    /// The OS/2 character subsystem
    pub const IMAGE_SUBSYSTEM_OS2_CUI: u16 = 5;
    /// The Posix character subsystem
    pub const IMAGE_SUBSYSTEM_POSIX_CUI: u16 = 7;
    /// Native Win9x driver
    pub const IMAGE_SUBSYSTEM_NATIVE_WINDOWS: u16 = 8;
    /// Windows CE
    pub const IMAGE_SUBSYSTEM_WINDOWS_CE_GUI: u16 = 9;
    /// An Extensible Firmware Interface (EFI) application
    pub const IMAGE_SUBSYSTEM_EFI_APPLICATION: u16 = 10;
    /// An EFI driver with boot services
    pub const IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER: u16 = 11;
    /// An EFI driver with run-time services
    pub const IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER: u16 = 12;
    /// An EFI ROM image
    pub const IMAGE_SUBSYSTEM_EFI_ROM: u16 = 13;
    /// XBOX
    pub const IMAGE_SUBSYSTEM_XBOX: u16 = 14;
    /// Windows boot application.
    pub const IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION: u16 = 16;
}
