#![no_main]
#![no_std]

use log::info;
use uefi::prelude::*;
use uefi::CStr16;
use uefi::proto::media::file::{File, FileMode, FileAttribute};

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    info!("Hello world!");

    let mut dir = system_table
        .boot_services()
        .get_image_file_system(_image_handle)
        .unwrap()
        .get_mut()
        .unwrap()
        .open_volume()
        .unwrap();

    let mut buf = [0; 6];
    let mut file = dir
        .open(
            &CStr16::from_str_with_buf("hello", &mut buf).unwrap(),
            FileMode::CreateReadWrite,
            FileAttribute::empty(),
        )
        .unwrap()
        .into_regular_file()
        .unwrap();

    file.write("Hello World!".as_bytes()).unwrap();
    file.close();

    loop {}
    Status::SUCCESS
}