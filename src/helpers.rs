
pub fn read_unsigned_int(buf: &[u8]) -> u32 {
    buf[3] as u32 |
    (buf[2] as u32) << 8 |
    (buf[1] as u32) << 16 |
    (buf[0] as u32) << 24
}
