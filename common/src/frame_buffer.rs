#[repr(C)]
#[derive(Debug)]
pub struct FrameBufferInfo {
    pub ptr: *mut u8,
    pub size: usize,
}
