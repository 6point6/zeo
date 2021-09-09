use byteorder::ByteOrder;
use std::io::Cursor;

#[macro_export]

macro_rules! fmt_err {
    ($($arg:tt)*) => {{
        let res = std::fmt::format(format_args!($($arg)*));

        format!(
            "[-] Error:\n\t- Cause: {}\n\t- Line: {}\n\t- File: {}\n\n{}",
            res,
            line!(),
            file!(),
            std::backtrace::Backtrace::force_capture()
        )
    }}
}

pub trait IterWriteBack<'a> {
    type Iter;
    type Output;

    fn iter(buf: &'a [u8]) -> Self::Iter
    where
        Self::Iter: Iterator;

    fn write_single(buf: &mut RWCursor, item: &Self::Output);

    fn write_all(buf: &mut RWCursor, item_vec: &Vec<Self::Output>) {
        for i in item_vec {
            Self::write_single(buf, i)
        }
    }
}

pub struct ROCursor<'a> {
    pub buf: &'a [u8],
    pos: usize,
}

impl<'a> ROCursor<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }
}

pub struct RWCursor<'a> {
    pub buf: &'a mut [u8],
    pos: usize,
}

impl<'a> RWCursor<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, pos: 0 }
    }
}

#[macro_use]
macro_rules! impl_ro_cursor {
    ($read_name:ident, $typ:ty) => {
        impl<'a> ROCursor<'a> {
            pub fn $read_name<E: ByteOrder>(&mut self) -> $typ {
                let r = E::$read_name(&self.buf[self.pos..self.buf.len()]);
                self.pos += core::mem::size_of::<$typ>();
                r
            }
        }
    };
}

#[macro_use]
macro_rules! impl_rw_cursor {
    ($read_name:ident, $write_name:ident, $typ:ty) => {
        impl<'a> RWCursor<'a> {
            pub fn $read_name<E: ByteOrder>(&mut self) -> $typ {
                let r = E::$read_name(&self.buf[self.pos..self.buf.len()]);
                self.pos += core::mem::size_of::<$typ>();
                r
            }

            pub fn $write_name<E: ByteOrder>(&mut self, val: $typ) {
                let buf_len = self.buf.len();
                E::$write_name(&mut self.buf[self.pos..buf_len], val);
                self.pos += core::mem::size_of::<$typ>();
            }
        }
    };
}

impl_ro_cursor!(read_u16, u16);
impl_ro_cursor!(read_u32, u32);
impl_ro_cursor!(read_u64, u64);
impl_ro_cursor!(read_u128, u128);

impl_rw_cursor!(read_u16, write_u16, u16);
impl_rw_cursor!(read_u32, write_u32, u32);
impl_rw_cursor!(read_u64, write_u64, u64);
impl_rw_cursor!(read_u128, write_u128, u128);
