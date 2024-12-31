#![no_std]
#![no_main]

use core::arch::asm;
use make_mikan_os_common::frame_buffer_config::{FrameBufferConfig, PixelFormat};
use make_mikan_os_common::pixel_color::PixelColor;

struct Display {
    pub pixel_format: PixelFormat,
    pub frame_buffer: *mut u8,
    pub vertical_resolution: u64,
    pub horizontal_resolution: u64,
    pub pixels_per_scan_line: u64,
}

impl Display {
    fn write_pixel(&self, x: isize, y: isize, color: PixelColor) {
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

// MEMO: frame_buffer_configに生やしたメンバメソッドを使うと、その時点でエラーになっていそう
#[no_mangle]
pub extern "C" fn kernel_main(frame_buffer_config: FrameBufferConfig) {
    let display = Display {
        pixel_format: frame_buffer_config.pixel_format,
        frame_buffer: frame_buffer_config.frame_buffer as *mut u8,
        vertical_resolution: frame_buffer_config.vertical_resolution,
        horizontal_resolution: frame_buffer_config.horizontal_resolution,
        pixels_per_scan_line: frame_buffer_config.pixels_per_scan_line,
    };

    for i in 0..display.horizontal_resolution {
        for j in 0..display.vertical_resolution {
            // PixelColorにnewメソッドを生やす形だと動かない。なぜ？
            display.write_pixel(
                i as isize,
                j as isize,
                PixelColor {
                    r: 255,
                    g: 255,
                    b: 255,
                },
            );
        }
    }
    for i in 0..200 {
        for j in 0..100 {
            display.write_pixel(
                i,
                j,
                PixelColor {
                    r: 0,
                    g: 255,
                    b: 0,
                },
            );
        }
    }

    loop {
        unsafe { asm!("hlt") };
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
