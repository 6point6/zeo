use crate::util::IterWriteBack;
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

pub struct ImportDescriptor {
    pub original_first_thunk: u32,
    pub time_data_stamp: u32,
    pub forwarder_chain: u32,
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

    fn write_item(buf: &mut Cursor<&'a mut [u8]>, import_desc: &Self::Output) {
        buf.write_u32::<LittleEndian>(import_desc.original_first_thunk).unwrap();
        buf.write_u32::<LittleEndian>(import_desc.time_data_stamp).unwrap();
        buf.write_u32::<LittleEndian>(import_desc.forwarder_chain).unwrap();
        buf.write_u32::<LittleEndian>(import_desc.first_thunk).unwrap();
    }
}
