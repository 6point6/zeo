#[allow(unused_attributes)]
#[macro_use]
#[allow(unused_imports)]
use assert_hex::assert_eq_hex;
use crate::util::{IterWriteBack, ROCursor, RWCursor};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use alloc::prelude::v1::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RelocationType {
    // For now skip/ignore the other types
    ImageRelBasedAbsolute,
    ImageRelBasedHigh,
    ImageRelBasedLow,
    ImageRelBasedHighLow,
    ImageRelBasedHighAdj,
    ImageRelBasedMipsJmpAddr,
}

impl RelocationType {
    pub fn new(reloc_type: u8) -> Self {
        match reloc_type {
            0 => Self::ImageRelBasedAbsolute,
            1 => Self::ImageRelBasedHigh,
            2 => Self::ImageRelBasedLow,
            3 => Self::ImageRelBasedHighLow,
            4 => Self::ImageRelBasedHighAdj,
            5 => Self::ImageRelBasedMipsJmpAddr,
            _ => unimplemented!("reloc_type: {}", reloc_type),
        }
    }
}

#[derive(Debug)]
pub struct RelocTypeOffset {
    pub reloc_type: RelocationType,
    pub reloc_offset: u16,
}

impl RelocTypeOffset {
    pub fn new(type_offset_pair: u16) -> Self {
        Self {
            reloc_type: RelocationType::new(((type_offset_pair & 0xF000) >> 12) as u8),
            reloc_offset: (type_offset_pair & 0xFFF) as u16,
        }
    }

    pub fn to_u16le(&self) -> u16 {
        (self.reloc_type as u16) << 12 | self.reloc_offset as u16
    }
}

#[derive(Debug)]
pub struct Relocation {
    pub virt_addr: u32,
    pub size_of_block: u32,
    pub block: Vec<RelocTypeOffset>,
}

pub struct RelocationsIter<'a> {
    buf: ROCursor<'a>,
}

impl<'a> RelocationsIter<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self {
            buf: ROCursor::new(buf),
        }
    }
}

impl<'a> Iterator for RelocationsIter<'a> {
    type Item = Relocation;

    // Maybe this shouldn't panic
    fn next(&mut self) -> Option<Self::Item> {
        let virt_addr = self.buf.read_u32::<LittleEndian>();

        if virt_addr == 0 {
            return None;
        }

        let size_of_block = self.buf.read_u32::<LittleEndian>();
        let type_offset_count = ((size_of_block - 8) / 2) as usize;

        let mut block: Vec<RelocTypeOffset> = Vec::with_capacity(type_offset_count);

        for _ in 1..=type_offset_count {
            let type_offset_pair = self.buf.read_u16::<LittleEndian>();
            block.push(RelocTypeOffset::new(type_offset_pair));
        }

        Some(Relocation {
            virt_addr,
            size_of_block,
            block,
        })
    }
}

pub struct Relocations;

impl<'a> IterWriteBack<'a> for Relocations {
    type Iter = RelocationsIter<'a>;
    type Output = Relocation;

    fn iter(buf: &'a [u8]) -> Self::Iter {
        RelocationsIter::into_iter(RelocationsIter::new(buf))
    }

    fn write_single(buf: &mut RWCursor, reloc: &Self::Output) {
        buf.write_u32::<LittleEndian>(reloc.virt_addr);
        buf.write_u32::<LittleEndian>(reloc.size_of_block);
        for e in &reloc.block {
            buf.write_u16::<LittleEndian>(e.to_u16le())
        }
    }
}

#[allow(dead_code)]
const RELOC_TESTDATA: [u8; 28] = [
    0, 0x10, 0, 0, 0x0C, 0, 0, 0, 0x17, 0x30, 0x1F, 0x30, 0, 0x10, 0, 0, 0x0C, 0, 0, 0, 0x17, 0x30,
    0x1F, 0x30, 0, 0, 0, 0,
];

#[cfg(feature = "std_unit_tests")]
#[cfg(test)]
use alloc::prelude::v1::*;

#[test]
fn relocations_iter() {
    let relocs_iter = RelocationsIter::new(&RELOC_TESTDATA);
    let mut relocs: Vec<Relocation> = Vec::with_capacity(2);

    for r in relocs_iter.into_iter() {
        relocs.push(r);
    }

    assert_eq_hex!(relocs[0].virt_addr, 0x1000);
    assert_eq_hex!(relocs[0].size_of_block, 0x0C);
    assert_eq_hex!(relocs[0].block[0].to_u16le(), 0x3017);
    assert_eq_hex!(relocs[0].block[1].to_u16le(), 0x301F);

    assert_eq_hex!(relocs[1].virt_addr, 0x1000);
    assert_eq_hex!(relocs[1].size_of_block, 0x0C);
    assert_eq_hex!(relocs[1].block[0].to_u16le(), 0x3017);
    assert_eq_hex!(relocs[1].block[1].to_u16le(), 0x301F);

    assert_eq!(
        relocs[0].block[0].reloc_type,
        RelocationType::ImageRelBasedHighLow
    );
    assert_eq!(
        relocs[0].block[1].reloc_type,
        RelocationType::ImageRelBasedHighLow
    );

    assert_eq!(
        relocs[1].block[0].reloc_type,
        RelocationType::ImageRelBasedHighLow
    );
    assert_eq!(
        relocs[1].block[1].reloc_type,
        RelocationType::ImageRelBasedHighLow
    );

    assert_eq!(relocs[0].block[0].reloc_offset, 0x17);
    assert_eq!(relocs[0].block[1].reloc_offset, 0x1F);

    assert_eq!(relocs[1].block[0].reloc_offset, 0x17);
    assert_eq!(relocs[1].block[1].reloc_offset, 0x1F);
}

#[test]
fn relocations_writeback() {
    let relocs_iter = Relocations::iter(&RELOC_TESTDATA);
    let write_buf = &mut [0 as u8; RELOC_TESTDATA.len()] as &mut [u8];
    let mut relocs_write_buf = RWCursor::new(write_buf);
    let mut relocs: Vec<Relocation> = Vec::with_capacity(2);

    for r in relocs_iter.into_iter() {
        relocs.push(r);
    }

    Relocations::write_all(&mut relocs_write_buf, &relocs);

    assert_eq!(RELOC_TESTDATA, relocs_write_buf.buf);
}
