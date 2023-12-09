use std::io::Read;

pub trait ReadArray {
    fn read_array<R: Read, const N: usize>(reader: &mut R) -> [u8; N] {
        let mut buf = [0u8; N];
        reader
            .read_exact(&mut buf)
            .expect("Data stream should be readable");
        buf
    }
}
