use make_mikan_os_common::frame_buffer_config::PixelFormat;
use make_mikan_os_common::pixel_color::PixelColor;

pub struct PixelWriter {
    pub pixel_format: PixelFormat,
    pub frame_buffer: *mut u8,
    pub vertical_resolution: u64,
    pub horizontal_resolution: u64,
    pub pixels_per_scan_line: u64,
}

impl PixelWriter {
    pub fn write(&self, x: isize, y: isize, color: &PixelColor) {
        // 縦にy段目のところから、横にx個進んだ位置を計算
        let pixel_position = (self.pixels_per_scan_line as isize) * y + x;
        let position = pixel_position * 4;

        match self.pixel_format {
            PixelFormat::PixelRGBResv8BitPerColor => unsafe {
                *self.frame_buffer.offset(position) = color.r;
                *self.frame_buffer.offset(position + 1) = color.g;
                *self.frame_buffer.offset(position + 2) = color.b;
            },
            PixelFormat::PixelBGRResv8BitPerColor => unsafe {
                *self.frame_buffer.offset(position) = color.b;
                *self.frame_buffer.offset(position + 1) = color.g;
                *self.frame_buffer.offset(position + 2) = color.r;
            },
        }
    }
}
