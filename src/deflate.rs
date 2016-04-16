extern crate flate2;

use std::io::Read;

use self::flate2::read::ZlibDecoder;

pub fn decode<F>(data: &[u8], get_size: F) -> Result<Vec<u8>, String> 
    where F: Fn() -> usize {

    let predict = get_size();
    let mut decompressed_data = Vec::new();
    let mut buf = Vec::with_capacity(predict);
    let mut decompressor = ZlibDecoder::new(&data[..]);
    match decompressor.read_to_end(&mut buf) {
        Ok(n) => {
            if n != 0 {
                decompressed_data.extend(buf.iter().cloned());
            }
        },
        Err(err) => return Err(err.to_string())
    }

    Ok(decompressed_data)
}
