struct FieldInfo<T, const N: usize> {
    offset: u64,
    name: String,
    raw_bytes: [u8; N],
    value: T,
}
