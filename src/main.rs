pub mod header;
pub mod section;

use std::fs::File;

use header::FileHeader;

fn main() {
    let mut pe_file = File::open("target\\debug\\pexp.exe").expect("File should be openable");
    let file_header = FileHeader::read_from(&mut pe_file);
    print!("{}", file_header);
}
