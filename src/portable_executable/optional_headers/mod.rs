mod standard_fields;
mod windows_specific_fields;
mod data_directories;

use standard_fields::StandardFields;
use windows_specific_fields::WindowsSpecificFields;
use data_directories::DataDirectories;

pub struct OptionalHeader {
    standard_fields: StandardFields,
    windows_specific_fields: WindowsSpecificFields,
    data_directories: DataDirectories,
}
