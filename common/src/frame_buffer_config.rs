#[derive(Clone, Copy, Eq, PartialEq)]
pub enum PixelFormat {
    PixelRGBResv8BitPerColor,
    PixelBGRResv8BitPerColor,
}

pub struct FrameBufferConfig {
    frame_buffer: *mut u8,
    // プラットフォームによって変わりそうなので、u32 ではなく usize で定義
    pixels_per_scan_line: usize,
    horizontal_resolution: usize,
    vertical_resolution: usize,
    pixel_format: PixelFormat,
}

impl FrameBufferConfig {
    pub fn new(frame_buffer: *mut u8, frame_buffer_size: usize, resolution: (usize, usize), pixel_format: PixelFormat) -> Self {
        Self {
            frame_buffer,
            pixels_per_scan_line: frame_buffer_size / resolution.1,
            horizontal_resolution: resolution.0,
            vertical_resolution: resolution.1,
            pixel_format,
        }
    }

    pub fn frame_buffer(&self) -> *mut u8 {
        self.frame_buffer
    }

    pub fn pixels_per_scan_line(&self) -> usize {
        self.pixels_per_scan_line
    }

    pub fn resolution(&self) -> (usize, usize) {
        (self.horizontal_resolution, self.vertical_resolution)
    }

    pub fn pixel_format(&self) -> PixelFormat {
        self.pixel_format
    }
}
