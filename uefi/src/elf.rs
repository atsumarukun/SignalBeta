use goblin::elf;
use alloc::vec;

pub struct Elf<'a> {
    pub data: elf::Elf<'a>,
    pub buf: &'a vec::Vec<u8>,
}

impl<'a> Elf<'a> {
    pub fn new(buf: &'a vec::Vec<u8>) -> Self {
        Self { data: elf::Elf::parse(&buf).unwrap(), buf }
    }

    pub fn get_address_range(&self) -> (u64, u64) {
        let mut start_ptr = u64::MAX;
        let mut end_ptr = 0;
        for ph in self.data.program_headers.iter() {
            if ph.p_type != elf::program_header::PT_LOAD {
                continue;
            }
            start_ptr = start_ptr.min(ph.p_vaddr);
            end_ptr = end_ptr.max(ph.p_vaddr + ph.p_memsz);
        }
        (start_ptr, end_ptr)
    }

    pub fn load(&self) {
        for ph in self.data.program_headers.iter() {
            if ph.p_type != elf::program_header::PT_LOAD {
                continue;
            }
            let offset = ph.p_offset as usize;
            let filesz = ph.p_filesz as usize;
            let memsz = ph.p_memsz as usize;
            let dest = unsafe { core::slice::from_raw_parts_mut(ph.p_vaddr as *mut u8, memsz) };
            dest[..filesz].copy_from_slice(&self.buf[offset..offset + filesz]);
            dest[filesz..].fill(0);
        }
    }

    pub fn get_entry(&self) -> u64 {
        self.data.entry
    }
}
