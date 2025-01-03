use make_mikan_os_common::pixel_color::PixelColor;
use crate::PixelWriter;

static FONT_BIN: &'static [u8] = include_bytes!("../font.bin");

fn get_font(char_code: u8) -> &'static [u8] {
    // 1文字あたり16バイトのフォントが並んでいる前提など
    let bytes_per_char = 16;
    // ASCIIコードは1から始まるので、1を引く
    let start = (char_code - 1) as usize * bytes_per_char;
    let end = start + bytes_per_char;
    if end <= FONT_BIN.len() {
        &FONT_BIN[start..end]
    } else {
        panic!("char_code {} is out of range", char_code);
    }
}

pub struct AsciiWriter<'a> {
    pub pixel_writer: &'a PixelWriter,
}

impl AsciiWriter<'_> {
    pub fn write(&self, x: isize, y: isize, c: char, color: &PixelColor) {
        let font = get_font(c as u8);

        for dy in 0..16 {
            for dx in 0..8 {
                if (font[dy] & (0x80 >> dx)) == 0 {
                    continue;
                }
                self.pixel_writer.write(x + dx, y + (dy as isize), color);
            }
        }
    }
}
