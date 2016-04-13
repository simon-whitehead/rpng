
use color_type::ColorType;
use helpers;

pub type IHDRParseResult = Result<IHDR, String>;

pub struct IHDR {
   pub width: usize,
   pub height: usize,

   pub bit_depth: u8,
   pub color_type: ColorType,
   pub compression_method: u8,
   pub filter_method: u8,
   pub interlace_method: u8,
   pub crc: u32
}

impl IHDR {
    pub fn new() -> Self {
        IHDR {
            width: 0,
            height: 0,
            bit_depth: 0,
            color_type: ColorType::Unknown,
            compression_method: 0,
            filter_method: 0,
            interlace_method: 0,
            crc: 0
        }
    }
}

pub fn parse(data: &[u8]) -> IHDRParseResult {
    let mut ihdr = IHDR::new();

    ihdr.width = helpers::read_unsigned_int(&data[0x04..]) as usize;
    ihdr.height = helpers::read_unsigned_int(&data[0x08..]) as usize;

    ihdr.bit_depth = data[0x0C];
    ihdr.color_type = ColorType::from(data[0x0D]);
    ihdr.compression_method = data[0x0E];
    ihdr.filter_method = data[0x0F];
    ihdr.interlace_method = data[0x10];
    ihdr.crc = helpers::read_unsigned_int(&data[0x11..]);

    if let Err(message) = ihdr.color_type.validate(ihdr.bit_depth) {
        return Err(message);
    }

    if ihdr.compression_method != 0 {
        return Err("Compression method invalid".to_string());
    }

    if ihdr.filter_method != 0 {
        return Err("Filter method invalid".to_string());
    }

    if ihdr.interlace_method > 1 {
        return Err("Interlace method invalid".to_string());
    }

    Ok(ihdr)
}
