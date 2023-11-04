use alloc::boxed::Box;
use alloc::vec;
use uefi::prelude::{Boot, Handle, SystemTable};
use uefi::proto::media::file::{Directory, File, FileAttribute, FileInfo, FileMode, RegularFile};
use uefi::CStr16;
use uefi::Error;

pub fn open_root_dir(
    image_handle: &Handle,
    system_table: &SystemTable<Boot>,
) -> Result<Directory, Error> {
    system_table
        .boot_services()
        .get_image_file_system(*image_handle)?
        .get_mut()
        .unwrap()
        .open_volume()
}

pub fn open_file(dir: &mut Directory, path: &str) -> Result<SimpleFile, Error> {
    let mut buf = vec![0; path.len() + 1];
    Ok(SimpleFile::new(
        dir.open(
            &CStr16::from_str_with_buf(path, &mut buf).unwrap(),
            FileMode::Read,
            FileAttribute::empty(),
        )?
        .into_regular_file()
        .unwrap(),
    ))
}

pub struct SimpleFile {
    pub file: RegularFile,
}

impl SimpleFile {
    fn new(file: RegularFile) -> Self {
        Self { file: file }
    }

    pub fn get_info(&mut self) -> Result<Box<FileInfo>, Error> {
        self.file.get_boxed_info::<FileInfo>()
    }

    pub fn read(&mut self) -> Result<vec::Vec<u8>, Error> {
        let file_size = self.get_info()?.file_size() as usize;
        let mut buf = vec![0; file_size];
        self.file.read(&mut buf)?;
        Ok(buf)
    }

    pub fn close(self) {
        self.file.close()
    }
}
