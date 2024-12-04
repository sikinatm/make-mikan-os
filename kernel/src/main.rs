#![no_std]
#![no_main]

use core::arch::asm;
use make_mikan_os_common::frame_buffer_config::{FrameBufferConfig, PixelFormat};
use make_mikan_os_common::pixel_color::PixelColor;

struct Display {
    pub pixel_format: PixelFormat,
    pub frame_buffer: *mut u8,
    pub pixels_per_scan_line: u64,
}

impl Display {
    fn write_pixel(&self, x: isize, y: isize, color: PixelColor) {
        let pixel_position = (self.pixels_per_scan_line as isize) * x * y;

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
    // ここのコメントアウトを外すと描画されなくなる。どうも、メンバメソッドを使うとダメなようだ
    // let display = Display {
    //     pixel_format: frame_buffer_config.pixel_format,
    //     frame_buffer: frame_buffer_config.frame_buffer as *mut u8,
    //     pixels_per_scan_line: frame_buffer_config.pixels_per_scan_line(),
    // };

    // for i in 0..frame_buffer_config.vertical_resolution {
    //     for j in 0..frame_buffer_config.horizontal_resolution {
    //         display.write_pixel(i as isize, j as isize, PixelColor::new(255, 255, 255));
    //     }
    // }
    let frame_buffer = frame_buffer_config.frame_buffer as *mut u8;
    for i in 0..frame_buffer_config.frame_buffer_size {
        unsafe {
            *frame_buffer.offset(i as isize) = (i % 256) as u8;
        }
    }
    // for i in 0..200 {
    //     for j in 0..100 {
    //         display.write_pixel(i, j, PixelColor::new(0, 255, 0));
    //     }
    // }

    loop {
        unsafe { asm!("hlt") };
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}