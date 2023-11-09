#![no_main]
#![no_std]

use common::frame_buffer;
use core::panic::PanicInfo;
use core::arch::asm;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    todo!()
}

#[no_mangle]
extern "sysv64" fn kernel_main(frame_buffer_info: frame_buffer::FrameBufferInfo) {
    let frame_buffer = unsafe { core::slice::from_raw_parts_mut(frame_buffer_info.ptr, frame_buffer_info.size) };
    frame_buffer.iter_mut().for_each(|buf| {
        *buf = 0;
    });

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
