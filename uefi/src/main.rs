#![no_main]
#![no_std]

extern crate alloc;

mod fs;

use core::str::from_utf8;
use log::info;
use uefi::prelude::{entry, Boot, Handle, Status, SystemTable};
use uefi::proto::media::file::File;

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    info!("Hello world!");

    let mut dir = fs::open_root_dir(&image_handle, &system_table).unwrap();

    let mut file = fs::open_file(&mut dir, "hello").unwrap();
    file.write("Hello World!".as_bytes()).unwrap();
    file.close();

    let mut file = fs::open_file(&mut dir, "hello").unwrap();
    let data = file.read().unwrap();
    file.close();
    info!("{:?}", from_utf8(&data).unwrap());

    dir.close();

    loop {}
    Status::SUCCESS
}
