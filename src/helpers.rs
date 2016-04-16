
pub fn read_unsigned_int(buf: &[u8]) -> u32 {
    buf[3] as u32 |
    (buf[2] as u32) << 8 |
    (buf[1] as u32) << 16 |
    (buf[0] as u32) << 24
}

pub fn extract_4bit(b: u8, index: u32) -> u8 {
    match index & 0x01 == 0 {
        true => ((b & 0xf0) >> 4),
        false => (b & 0x0f)
    }
}
