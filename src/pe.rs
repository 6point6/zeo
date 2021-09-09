#[allow(unused_attributes)]
#[macro_use]
#[allow(unused_imports)]
use crate::fmt_err;
use crate::{
    dos_hdr::DosHeader, imports::ImportDescriptor, nt_hdr::*, relocs::Relocation,
    sec_hdr::SectionHeader,
};
use zordon::prelude::*;
use alloc::prelude::v1::*;
use alloc::format;

pub struct PeHeader<'a> {
    pub dos_hdr: DosHeader<'a>,
    pub nt_hdr: NtHeader<'a>,
    pub sec_hdrs: Vec<SectionHeader<'a>>, // These could/should be Option
    pub secs: Vec<VarArrayView<'a, u8>>,
}

impl<'a> PeHeader<'a> {
    pub fn new(rwbuf: &'a mut [u8]) -> Self {
        let rwbuf_len = rwbuf.len();

        let (dos_hdr, leftover) = DosHeader::mut_view(rwbuf);

        let (_, leftover) =
            leftover.split_at_mut(dos_hdr.addr_of_new_exe_hdr.val() as usize - 0x40);

        let (nt_hdr, mut leftover) = NtHeader::mut_view(leftover);

        let num_of_secs = nt_hdr.file_hdr.num_of_secs.val();
        let mut sec_hdrs: Vec<SectionHeader> = Vec::with_capacity(num_of_secs as usize);

        let (_, l) = leftover.split_at_mut(0x80);
        leftover = l;

        for _ in 0..num_of_secs {
            let (slice, l) = SectionHeader::mut_view(leftover);
            sec_hdrs.push(slice);
            leftover = l;
        }

        let mut secs: Vec<VarArrayView<u8>> = Vec::new();
        sec_hdrs.sort_by_key(|s| s.ptr_to_raw_data.val());

        for h in &sec_hdrs {
            let rel_offset = rwbuf_len - leftover.len();

            let (_, left) = leftover.split_at_mut(h.ptr_to_raw_data.val() as usize - rel_offset);
            let (sec, l) = VarArrayView::<u8>::mut_view(left, h.size_of_raw_data.val() as usize);
            secs.push(sec);

            leftover = l;
        }

        Self {
            dos_hdr,
            nt_hdr,
            sec_hdrs,
            secs,
        }
    }

    pub fn rva_to_file_offset(sec_hdrs: &Vec<SectionHeader>, rva: u32) -> Result<u32, String> {
        for s in sec_hdrs.iter() {
            if (s.virt_addr.val() <= rva) && ((s.virt_addr.val() + s.virt_size.val()) > rva) {
                return Ok((rva - s.virt_addr.val()) + s.ptr_to_raw_data.val());
            }
        }

        Err(fmt_err!("Could find section rva resides in"))
    }

    pub fn virt_addr_to_sec_index(&self, section_va: u32) -> Result<usize, String> {
        for (i, s) in self.sec_hdrs.iter().enumerate() {
            if (s.virt_addr.val() <= section_va)
                && ((s.virt_addr.val() + Self::sec_virt_size(s.size_of_raw_data.val()))
                    > section_va)
            {
                return Ok(i);
            }
        }

        Err(fmt_err!(
            "Could not find section with va: {:#X}",
            section_va
        ))
    }

    pub fn entry_sec_index(&self) -> Result<usize, String> {
        self.virt_addr_to_sec_index(self.nt_hdr.opt_hdr.addr_of_entrypoint.val())
    }

    pub fn entry_rel_sec_offset(&self) -> Result<usize, String> {
        Ok(self.nt_hdr.opt_hdr.addr_of_entrypoint.val() as usize
            - self.entry_sec_ref()?.virt_addr.val() as usize)
    }

    pub fn entry_sec_ref(&self) -> Result<&SectionHeader<'a>, String> {
        Ok(&self.sec_hdrs[self.entry_sec_index()?])
    }

    pub fn entry_sec_virt_ip(&self) -> Result<u64, String> {
        Ok(self.nt_hdr.opt_hdr.image_base.val() + self.entry_sec_ref()?.virt_addr.val() as u64)
    }

    pub fn entry_disk_offset(&self) -> Result<usize, String> {
        Ok(self.entry_sec_ref()?.ptr_to_raw_data.val() as usize + self.entry_rel_sec_offset()?)
    }

    pub fn entry_sec_virt_size(&self) -> Result<u32, String> {
        Ok(((self.entry_sec_ref()?.size_of_raw_data.val() / 0x1000) + 1) * 0x1000)
    }

    pub fn sec_virt_size(size_of_raw_data: u32) -> u32 {
        ((size_of_raw_data / 0x1000) + 1) * 0x1000
    }
}

//Tests
#[cfg(feature = "std_unit_tests")]
use assert_hex::assert_eq_hex;
#[test]
fn virt_addr_to_sec_index() {
    let mut buf = read_test_pe();
    let mut pe_hdr = PeHeader::new(&mut buf);

    pe_hdr.sec_hdrs[0].virt_addr.set(0x1000);
    pe_hdr.sec_hdrs[0].virt_size.set(0x1000);

    pe_hdr.sec_hdrs[1].virt_addr.set(0x2000);
    pe_hdr.sec_hdrs[1].virt_size.set(0x1000);

    assert_eq_hex!(pe_hdr.virt_addr_to_sec_index(0x0).ok(), None);
    assert_eq_hex!(pe_hdr.virt_addr_to_sec_index(0x1000).ok(), Some(0));
    assert_eq_hex!(pe_hdr.virt_addr_to_sec_index(0x1500).ok(), Some(0));
    assert_eq_hex!(pe_hdr.virt_addr_to_sec_index(0x2000).ok(), Some(1));
}

#[test]
fn entry_sec_index() {
    let mut buf = read_test_pe();
    let mut pe_hdr = PeHeader::new(&mut buf);

    let new_entry_va = pe_hdr.sec_hdrs[0].virt_addr.val();
    pe_hdr.nt_hdr.opt_hdr.addr_of_entrypoint.set(new_entry_va);

    assert_eq_hex!(pe_hdr.entry_sec_index().ok(), Some(0));
}

#[test]
fn entry_rel_sec_offset() {
    let mut buf = read_test_pe();
    let mut pe_hdr = PeHeader::new(&mut buf);

    pe_hdr
        .nt_hdr
        .opt_hdr
        .addr_of_entrypoint
        .set(pe_hdr.sec_hdrs[0].virt_addr.val() + 0x100);
    assert_eq_hex!(pe_hdr.entry_rel_sec_offset().ok(), Some(0x100));

    pe_hdr.nt_hdr.opt_hdr.addr_of_entrypoint.set(0);
    assert_eq_hex!(pe_hdr.entry_rel_sec_offset().ok(), None);
}

#[test]
fn entry_sec_ref() {
    let mut buf = read_test_pe();
    let mut pe_hdr = PeHeader::new(&mut buf);

    pe_hdr.nt_hdr.opt_hdr.addr_of_entrypoint.set(0x1500);

    pe_hdr.sec_hdrs[0].virt_addr.set(0x1000);
    pe_hdr.sec_hdrs[0].virt_size.set(0x1000);

    assert_eq_hex!(*pe_hdr.entry_sec_ref().unwrap(), pe_hdr.sec_hdrs[0]);
}

#[test]
pub fn entry_sec_virt_ip() {
    let mut buf = read_test_pe();
    let mut pe_hdr = PeHeader::new(&mut buf);

    pe_hdr.nt_hdr.opt_hdr.addr_of_entrypoint.set(0x1000);

    pe_hdr.sec_hdrs[0].virt_addr.set(0x1000);
    pe_hdr.nt_hdr.opt_hdr.image_base.set(0x500000);

    assert_eq_hex!(pe_hdr.entry_sec_virt_ip().unwrap(), 0x501000)
}

#[test]
pub fn entry_disk_offset() {
    let mut buf = read_test_pe();
    let mut pe_hdr = PeHeader::new(&mut buf);

    pe_hdr.nt_hdr.opt_hdr.addr_of_entrypoint.set(0x1500);

    pe_hdr.sec_hdrs[0].virt_addr.set(0x1000);
    pe_hdr.sec_hdrs[0].virt_size.set(0x1000);
    pe_hdr.sec_hdrs[0].ptr_to_raw_data.set(0x400);

    assert_eq_hex!(pe_hdr.entry_disk_offset().unwrap(), 0x900);
}

#[test]
pub fn entry_sec_virt_size() {
    let mut buf = read_test_pe();
    let mut pe_hdr = PeHeader::new(&mut buf);

    pe_hdr.nt_hdr.opt_hdr.addr_of_entrypoint.set(0x1500);

    pe_hdr.sec_hdrs[0].virt_addr.set(0x1000);
    pe_hdr.sec_hdrs[0].size_of_raw_data.set(0x442);
    assert_eq_hex!(pe_hdr.entry_sec_virt_size().unwrap(), 0x1000);

    pe_hdr.sec_hdrs[0].size_of_raw_data.set(0x0);
    assert_eq_hex!(pe_hdr.entry_sec_virt_size().unwrap(), 0x1000);

    pe_hdr.sec_hdrs[0].size_of_raw_data.set(0x1001);
    assert_eq_hex!(pe_hdr.entry_sec_virt_size().unwrap(), 0x2000);
}

pub fn read_test_pe() -> Vec<u8> {
    std::fs::read("test_data/test_pe.exe").unwrap()
}
