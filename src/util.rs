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

    fn write_single(buf: &mut Cursor<&'a mut [u8]>, item: &Self::Output);

    fn write_all(buf: &mut Cursor<&'a mut [u8]>, item_vec: &Vec<Self::Output>) {
        for i in item_vec {
            Self::write_single(buf, i)
        }
    }
}
