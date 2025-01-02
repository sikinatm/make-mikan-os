use make_mikan_os_common::pixel_color::PixelColor;
use crate::PixelWriter;

const FONT_A: [u8; 16] = [
    0b00000000, //
    0b00011000, //    **
    0b00011000, //    **
    0b00011000, //    **
    0b00011000, //    **
    0b00111100, //   ****
    0b00100100, //   *  *
    0b00100100, //   *  *
    0b00100100, //   *  *
    0b01111110, //  ******
    0b01000010, //  *    *
    0b01000010, //  *    *
    0b01000010, //  *    *
    0b11100111, // ***  ***
    0b00000000, //
    0b00000000, //
];

pub struct AsciiWriter<'a> {
    pub pixel_writer: &'a PixelWriter,
}

impl AsciiWriter<'_> {
    pub fn write_ascii(&self, x: isize, y: isize, c: char, color: &PixelColor) {
        let font = match c {
            'A' => FONT_A,
            _ => [0; 16],
        };

        for dy in 0..16 {
            for dx in 0..8 {
                if ((font[dy as usize] << dx) & 0x80) != 0 {
                    self.pixel_writer.write(x + dx, y + (dy as isize), color);
                }
            }
        }
    }
}