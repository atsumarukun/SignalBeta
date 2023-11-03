#![no_main]
#![no_std]

extern crate alloc;

mod fs;

use core::arch::asm;
use core::str::from_utf8;
use log::info;
use uefi::prelude::{entry, Boot, Handle, Status, SystemTable};
use uefi::proto::media::file::File;
use uefi::table::boot::{AllocateType, MemoryType, MemoryDescriptor};
use uefi::Error;
use alloc::vec;
use goblin::elf;

type EntryPoint = extern "C" fn();

fn load_elf(system_table: &mut SystemTable<Boot>, buf: &vec::Vec<u8>) -> usize {
    let elf = elf::Elf::parse(&buf).unwrap();

    let mut start_ptr = usize::MAX;
    let mut end_ptr = 0;
    for ph in elf.program_headers.iter() {
        if ph.p_type != elf::program_header::PT_LOAD {
            continue;
        }
        start_ptr = start_ptr.min(ph.p_vaddr as usize);
        end_ptr = end_ptr.max((ph.p_vaddr + ph.p_memsz) as usize);
    }

    let _ = system_table
        .boot_services()
        .allocate_pages(
            AllocateType::Address(start_ptr.try_into().unwrap()),
            MemoryType::LOADER_DATA,
            (end_ptr - start_ptr + 0xfff) / 0x1000,
        )
        .unwrap();

    for ph in elf.program_headers.iter() {
        if ph.p_type != elf::program_header::PT_LOAD {
            continue;
        }
        let offset = ph.p_offset as usize;
        let filesz = ph.p_filesz as usize;
        let memsz = ph.p_memsz as usize;
        let dest = unsafe { core::slice::from_raw_parts_mut(ph.p_vaddr as *mut u8, memsz) };
        dest[..filesz].copy_from_slice(&buf[offset..offset + filesz]);
        dest[filesz..].fill(0);
    }

    return elf.entry as usize;
}

fn boot(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Result<(), Error> {
    uefi_services::init(&mut system_table)?;
    info!("Hello world!");

    let mut dir = fs::open_root_dir(&image_handle, &system_table)?;

    let mut file = fs::open_file(&mut dir, "kernel.elf")?;
    let data = file.read()?;
    file.close();
    let entry_point_ptr = load_elf(&mut system_table, &data);
    let entry_point: EntryPoint = unsafe { core::mem::transmute(entry_point_ptr) };

    dir.close();

    system_table.exit_boot_services(MemoryType::LOADER_DATA);

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
