#![no_std]
#![no_main]

use core::arch::asm;
use make_mikan_os_common::frame_buffer_config::{FrameBufferConfig, PixelFormat};
use make_mikan_os_common::pixel_color::PixelColor;

struct Display {
    pixel_format: PixelFormat,
    frame_buffer: *mut u8,
    pixels_per_scan_line: usize,
}

impl Display {
    fn new(pixel_format: PixelFormat, frame_buffer: *mut u8, pixels_per_scan_line: usize) -> Self {
        Self {
            pixel_format,
            frame_buffer,
            pixels_per_scan_line,
        }
    }

    fn write_pixel(&self, x: usize, y: usize, color: PixelColor) {
        let pixel_position = self.pixels_per_scan_line * x * y;

        match self.pixel_format {
            PixelFormat::PixelRGBResv8BitPerColor => {
                let position = pixel_position * 4;
                unsafe {
                    *self.frame_buffer.offset(position as isize) = color.r;
                    *self.frame_buffer.offset((position + 1) as isize) = color.g;
                    *self.frame_buffer.offset((position + 2) as isize) = color.b;
                }
            }
            PixelFormat::PixelBGRResv8BitPerColor => {
                let position = pixel_position * 4;
                unsafe {
                    *self.frame_buffer.offset(position as isize) = color.b;
                    *self.frame_buffer.offset((position + 1) as isize) = color.g;
                    *self.frame_buffer.offset((position + 2) as isize) = color.r;
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn KernelMain(frame_buffer_config: FrameBufferConfig) {
    let resolution =  frame_buffer_config.resolution();
    let display = Display::new(frame_buffer_config.pixel_format(), frame_buffer_config.frame_buffer(), frame_buffer_config.pixels_per_scan_line());
    for i in 0..resolution.0 {
        for j in 0..resolution.1 {
            display.write_pixel(i, j, PixelColor::new(255, 255, 255));
        }
    }
    for i in 0..200 {
        for j in 0..100 {
            display.write_pixel(i, j, PixelColor::new(0, 255, 0));
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