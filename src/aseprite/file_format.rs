use nom::{
    bytes::complete::{tag, take},
    number::complete::*,
    IResult,
};

// .aseprite file header
#[derive(Debug)]
#[allow(dead_code)]
pub struct FileHeader {
    pub file_size: u32,
    // pub magic_number: [u8; 2],
    pub frame: u16,
    pub width: u16,
    pub height: u16,
    pub color_depth: u16,
    pub flags: u32,
    pub speed: u16,
    // pub _reserved1: u32,
    // pub _reserved2: u32,
    pub palette_entry: u8,
    // pub ignore_bytes: [u8; 3],
    pub num_colors: u16,
    pub pixel_width: u8,
    pub pixel_height: u8,
    pub grid_x: i16,
    pub grid_y: i16,
    pub grid_width: u16,
    pub grid_height: u16,
    // pub _reserved3: [u8; 84],
}

pub fn read_aseprite_file_header(input: &[u8]) -> IResult<&[u8], FileHeader> {
    let (input, file_size) = le_u32(input)?;
    let (input, _magic_number) = tag(&[0xE0, 0xA5])(input)?;
    let (input, frame) = le_u16(input)?;
    let (input, width) = le_u16(input)?;
    let (input, height) = le_u16(input)?;
    let (input, color_depth) = le_u16(input)?;
    let (input, flags) = le_u32(input)?;
    let (input, speed) = le_u16(input)?;
    let (input, _reserved1) = le_u32(input)?;
    let (input, _reserved2) = le_u32(input)?;
    let (input, palette_entry) = le_u8(input)?;
    let (input, _ignore_bytes) = take(3usize)(input)?;
    let (input, num_colors) = le_u16(input)?;
    let (input, pixel_width) = le_u8(input)?;
    let (input, pixel_height) = le_u8(input)?;
    let (input, grid_x) = le_i16(input)?;
    let (input, grid_y) = le_i16(input)?;
    let (input, grid_width) = le_u16(input)?;
    let (input, grid_height) = le_u16(input)?;
    let (input, _) = take(84usize)(input)?;

    Ok((
        input,
        FileHeader {
            file_size,
            frame,
            width,
            height,
            color_depth,
            flags,
            speed,
            palette_entry,
            num_colors,
            pixel_width,
            pixel_height,
            grid_x,
            grid_y,
            grid_width,
            grid_height,
        },
    ))
}

// TODO: parseのテストはどう書けばいい？
