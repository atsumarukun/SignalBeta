#![no_main]
#![no_std]

extern crate alloc;

use log::info;
use alloc::vec;
use core::str::from_utf8;
use uefi::prelude::*;
use uefi::CStr16;
use uefi::proto::media::file::{File, FileMode, FileAttribute, FileInfo};

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

    let mut buf = [0; 6];
    let mut file = dir
        .open(
            &CStr16::from_str_with_buf("hello", &mut buf).unwrap(),
            FileMode::Read,
            FileAttribute::empty(),
        )
        .unwrap()
        .into_regular_file()
        .unwrap();
    let file_size = file.get_boxed_info::<FileInfo>().unwrap().file_size() as usize;
    let mut buf = vec![0; file_size];
    file.read(&mut buf).unwrap();
    file.close();
    info!("{:?}", from_utf8(&buf).unwrap());

    loop {}
    Status::SUCCESS
}