#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum PixelFormat {
    PixelRGBResv8BitPerColor,
    PixelBGRResv8BitPerColor,
}

#[repr(C)]
pub struct FrameBufferConfig {
    pub frame_buffer: u64,
    // プラットフォームによって変わりそうなので、u32 ではなく usize で定義
    pub frame_buffer_size: u64,
    pub horizontal_resolution: u64,
    pub vertical_resolution: u64,
    pub pixel_format: PixelFormat,
}

impl FrameBufferConfig {
    pub fn pixels_per_scan_line(&self) -> u64 {
        self.frame_buffer_size / self.vertical_resolution
    }
}
