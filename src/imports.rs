use crate::util::IterWriteBack;
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;
#[allow(unused_imports)]
use assert_hex::assert_eq_hex;

pub struct ImportDescriptor {
    pub original_first_thunk: u32,
    pub time_data_stamp: u32,
    pub forwarder_chain: u32,
    pub name: u32,
    pub first_thunk: u32,
}

pub struct ImportDescriptorIter<'a> {
    buf: Cursor<&'a [u8]>,
}

impl<'a> ImportDescriptorIter<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self {
            buf: Cursor::new(buf),
        }
    }
}

impl<'a> Iterator for ImportDescriptorIter<'a> {
    type Item = ImportDescriptor;

    fn next(&mut self) -> Option<Self::Item> {
        let original_first_thunk = self.buf.read_u32::<LittleEndian>().unwrap();

        if original_first_thunk == 0 {
            return None;
        }

        Some(ImportDescriptor {
            original_first_thunk,
            time_data_stamp: self.buf.read_u32::<LittleEndian>().unwrap(),
            forwarder_chain: self.buf.read_u32::<LittleEndian>().unwrap(),
            name: self.buf.read_u32::<LittleEndian>().unwrap(),
            first_thunk: self.buf.read_u32::<LittleEndian>().unwrap(),
        })
    }
}

pub struct ImportDescriptors;

impl<'a> IterWriteBack<'a> for ImportDescriptors {
    type Iter = ImportDescriptorIter<'a>;
    type Output = ImportDescriptor;

    fn iter(buf: &'a [u8]) -> Self::Iter {
        ImportDescriptorIter::into_iter(ImportDescriptorIter::new(buf))
    }

    fn write_single(buf: &mut Cursor<&'a mut [u8]>, import_desc: &Self::Output) {
        buf.write_u32::<LittleEndian>(import_desc.original_first_thunk)
            .unwrap();
        buf.write_u32::<LittleEndian>(import_desc.time_data_stamp)
            .unwrap();
        buf.write_u32::<LittleEndian>(import_desc.forwarder_chain)
            .unwrap();
        buf.write_u32::<LittleEndian>(import_desc.name).unwrap();
        buf.write_u32::<LittleEndian>(import_desc.first_thunk)
            .unwrap();
    }
}

#[allow(dead_code)]
const IMPORT_DESC_TESTDATA: [u8; 44] = [
    0x40, 0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6E, 0x31, 0x00, 0x00,
    0x00, 0x30, 0x00, 0x00, 0x50, 0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x8A, 0x31, 0x00, 0x00, 0x10, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 
];

#[test]
fn import_descriptor_iter() {
    let import_descs_iter = ImportDescriptorIter::new(&IMPORT_DESC_TESTDATA);
    let mut import_descs: Vec<ImportDescriptor> = Vec::with_capacity(2);

    for i in import_descs_iter.into_iter() {
        import_descs.push(i);
    }

    assert_eq_hex!(import_descs[0].original_first_thunk, 0x3140);
    assert_eq_hex!(import_descs[0].time_data_stamp, 0);
    assert_eq_hex!(import_descs[0].forwarder_chain, 0);
    assert_eq_hex!(import_descs[0].name, 0x316E);
    assert_eq_hex!(import_descs[0].first_thunk, 0x3000);

    assert_eq_hex!(import_descs[1].original_first_thunk, 0x3150);
    assert_eq_hex!(import_descs[1].time_data_stamp, 0);
    assert_eq_hex!(import_descs[1].forwarder_chain, 0);
    assert_eq_hex!(import_descs[1].name, 0x318A);
    assert_eq_hex!(import_descs[1].first_thunk, 0x3010);
}

#[test]
fn import_descriptor_writeback() {
    let import_descs_iter = ImportDescriptorIter::new(&IMPORT_DESC_TESTDATA);
    let write_buf = &mut [0 as u8; IMPORT_DESC_TESTDATA.len()] as &mut [u8];
    let mut import_descs_write_buf = Cursor::new(write_buf);
    let mut import_descriptors: Vec<ImportDescriptor> = Vec::with_capacity(2);

    for r in import_descs_iter.into_iter() {
        import_descriptors.push(r);
    }

    ImportDescriptors::write_all(&mut import_descs_write_buf, &import_descriptors);

    assert_eq!(IMPORT_DESC_TESTDATA, import_descs_write_buf.into_inner());
}