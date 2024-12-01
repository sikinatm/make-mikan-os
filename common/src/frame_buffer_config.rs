#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum PixelFormat {
    PixelRGBResv8BitPerColor,
    PixelBGRResv8BitPerColor,
}

#[repr(C)]
pub struct FrameBufferConfig {
    frame_buffer: u64,
    // プラットフォームによって変わりそうなので、u32 ではなく usize で定義
    frame_buffer_size: usize,
    horizontal_resolution: usize,
    vertical_resolution: usize,
    pixel_format: PixelFormat,
}

impl FrameBufferConfig {
    pub fn new(frame_buffer: u64, frame_buffer_size: usize, resolution: (usize, usize), pixel_format: PixelFormat) -> Self {
        Self {
            frame_buffer,
            frame_buffer_size,
            horizontal_resolution: resolution.0,
            vertical_resolution: resolution.1,
            pixel_format,
        }
    }

    pub fn frame_buffer(&self) -> *mut u8 {
        self.frame_buffer as *mut u8
    }

    pub fn pixels_per_scan_line(&self) -> usize {
        self.frame_buffer_size / self.vertical_resolution
    }

    pub fn resolution(&self) -> (usize, usize) {
        (self.horizontal_resolution, self.vertical_resolution)
    }

    pub fn pixel_format(&self) -> PixelFormat {
        self.pixel_format
    }
}
