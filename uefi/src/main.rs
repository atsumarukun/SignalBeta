#![no_main]
#![no_std]
#![feature(error_in_core)]

extern crate alloc;

mod elf;
mod error;
mod fs;

use alloc::vec;
use core::arch::asm;
use log::info;
use uefi::prelude::{entry, Boot, Handle, Status, SystemTable};
use uefi::proto::media::file::File;
use uefi::table::boot::{AllocateType, MemoryType, OpenProtocolParams, OpenProtocolAttributes};
use uefi::proto::console::gop::GraphicsOutput;

type EntryPoint = extern "C" fn();

fn read_kernel(
    image_handle: &Handle,
    system_table: &SystemTable<Boot>,
) -> Result<vec::Vec<u8>, error::Error> {
    let mut dir = fs::open_root_dir(&image_handle, &system_table)?;
    let mut file = fs::open_file(&mut dir, "kernel.elf")?;
    let buf = file.read()?;
    file.close();
    dir.close();
    Ok(buf)
}

fn load_kernel(
    system_table: &mut SystemTable<Boot>,
    buf: &vec::Vec<u8>,
) -> Result<u64, error::Error> {
    let elf = elf::Elf::new(&buf)?;
    let (start_ptr, end_ptr) = elf.get_address_range();
    let _ = system_table.boot_services().allocate_pages(
        AllocateType::Address(start_ptr),
        MemoryType::LOADER_DATA,
        ((end_ptr - start_ptr + 0xfff) / 0x1000) as usize,
    )?;
    elf.load();
    Ok(elf.get_entry())
}

fn draw_display(system_table: &SystemTable<Boot>) -> Result<(), error::Error> {
    let boot_services = system_table.boot_services();
    let handle = boot_services.get_handle_for_protocol::<GraphicsOutput>()?;
    let mut gop = unsafe { boot_services.open_protocol::<GraphicsOutput>(
        OpenProtocolParams {
            handle: handle,
            agent: boot_services.image_handle(),
            controller: None,
        },
        OpenProtocolAttributes::GetProtocol,
    )? };
    let frame_buffer = unsafe { core::slice::from_raw_parts_mut(gop.frame_buffer().as_mut_ptr(), gop.frame_buffer().size()) };
    frame_buffer.iter_mut().for_each(|buf| {
        *buf = 255;
    });
    Ok(())
}

fn boot(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Result<(), error::Error> {
    uefi_services::init(&mut system_table)?;
    info!("Hello world!");

    let buf = read_kernel(&image_handle, &system_table)?;
    let entry_point_ptr = load_kernel(&mut system_table, &buf)?;
    let entry_point: EntryPoint = unsafe { core::mem::transmute(entry_point_ptr) };

    let _ = draw_display(&system_table)?;

    let _ = system_table.exit_boot_services(MemoryType::LOADER_DATA);

    entry_point();

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[entry]
fn main(image_handle: Handle, system_table: SystemTable<Boot>) -> Status {
    match boot(image_handle, system_table) {
        Ok(_) => Status::SUCCESS,
        Err(e) => panic!("{}", e),
    }
}
