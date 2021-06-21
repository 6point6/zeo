use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Read, Write};

pub struct ImportDescriptor {
    pub original_first_thunk: u32,
    pub time_data_stamp: u32,
    pub forwarder_chain: u32,
    pub first_thunk: u32,
}

pub struct ImportDescriptorIter<'a> {
    buf: Cursor<&'a [u8]>,
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

//pub struct ImportDescriptors;

