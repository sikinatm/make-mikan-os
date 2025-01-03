#![no_std]
#![no_main]

mod font_writer;
mod pixel_writer;

use core::arch::asm;
use make_mikan_os_common::frame_buffer_config::{FrameBufferConfig};
use make_mikan_os_common::pixel_color::PixelColor;
use crate::pixel_writer::PixelWriter;

// MEMO: frame_buffer_configに生やしたメンバメソッドを使うと、その時点でエラーになっていそう
#[no_mangle]
pub extern "C" fn kernel_main(frame_buffer_config: FrameBufferConfig) {
    let pixel_writer = PixelWriter {
        pixel_format: frame_buffer_config.pixel_format,
        frame_buffer: frame_buffer_config.frame_buffer as *mut u8,
        vertical_resolution: frame_buffer_config.vertical_resolution,
        horizontal_resolution: frame_buffer_config.horizontal_resolution,
        pixels_per_scan_line: frame_buffer_config.pixels_per_scan_line,
    };

    let ascii_writer = font_writer::AsciiWriter {
        pixel_writer: &pixel_writer,
    };

    for i in 0..pixel_writer.horizontal_resolution {
        for j in 0..pixel_writer.vertical_resolution {
            // PixelColorにnewメソッドを生やす形だと動かない。なぜ？
            pixel_writer.write(
                i as isize,
                j as isize,
                &PixelColor {
                    r: 255,
                    g: 255,
                    b: 255,
                },
            );
        }
    }

    for i in 0..200 {
        for j in 0..100 {
            pixel_writer.write(
                i,
                j,
                &PixelColor {
                    r: 0,
                    g: 255,
                    b: 0,
                },
            );
        }
    }

    ascii_writer.write_ascii(50, 50, 'H', &PixelColor { r: 0, g: 0, b: 0 });
    ascii_writer.write_ascii(58, 50, 'A', &PixelColor { r: 0, g: 0, b: 0 });
    ascii_writer.write_ascii(66, 50, 'P', &PixelColor { r: 0, g: 0, b: 0 });
    ascii_writer.write_ascii(74, 50, 'P', &PixelColor { r: 0, g: 0, b: 0 });
    ascii_writer.write_ascii(82, 50, 'Y', &PixelColor { r: 0, g: 0, b: 0 });
    ascii_writer.write_ascii(98, 50, 'N', &PixelColor { r: 0, g: 0, b: 0 });
    ascii_writer.write_ascii(106, 50, 'E', &PixelColor { r: 0, g: 0, b: 0 });
    ascii_writer.write_ascii(114, 50, 'W', &PixelColor { r: 0, g: 0, b: 0 });
    ascii_writer.write_ascii(130, 50, 'Y', &PixelColor { r: 0, g: 0, b: 0 });
    ascii_writer.write_ascii(138, 50, 'E', &PixelColor { r: 0, g: 0, b: 0 });
    ascii_writer.write_ascii(146, 50, 'A', &PixelColor { r: 0, g: 0, b: 0 });
    ascii_writer.write_ascii(154, 50, 'R', &PixelColor { r: 0, g: 0, b: 0 });
    ascii_writer.write_ascii(170, 50, '2', &PixelColor { r: 0, g: 0, b: 0 });
    ascii_writer.write_ascii(178, 50, '0', &PixelColor { r: 0, g: 0, b: 0 });
    ascii_writer.write_ascii(186, 50, '2', &PixelColor { r: 0, g: 0, b: 0 });
    ascii_writer.write_ascii(194, 50, '5', &PixelColor { r: 0, g: 0, b: 0 });


    loop {
        unsafe { asm!("hlt") };
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
