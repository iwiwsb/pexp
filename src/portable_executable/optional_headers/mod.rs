mod data_directories;
mod standard_fields;
mod windows_specific_fields;

use data_directories::DataDirectories;
use standard_fields::StandardFields;
use windows_specific_fields::WindowsSpecificFields;

pub struct OptionalHeader {
    standard_fields: StandardFields,
    windows_specific_fields: WindowsSpecificFields,
    data_directories: DataDirectories,
}
