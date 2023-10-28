#![no_main]
#![no_std]

extern crate alloc;

mod fs;

use core::arch::asm;
use core::str::from_utf8;
use log::info;
use uefi::prelude::{entry, Boot, Handle, Status, SystemTable};
use uefi::proto::media::file::File;
use uefi::Error;

fn boot(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Result<(), Error> {
    uefi_services::init(&mut system_table)?;
    info!("Hello world!");

    let mut dir = fs::open_root_dir(&image_handle, &system_table)?;

    let mut file = fs::open_file(&mut dir, "hello")?;
    file.write("Hello World!".as_bytes())?;
    file.close();

    let mut file = fs::open_file(&mut dir, "hello")?;
    let data = file.read()?;
    file.close();
    info!("{:?}", from_utf8(&data).unwrap());

    dir.close();

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
