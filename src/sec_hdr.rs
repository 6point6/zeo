use zordon::types::*;
use zordon::MutView;

#[derive(MutView, Debug, PartialEq)]
pub struct SectionHeader<'a> {
    pub name: ArrayView<'a, [u8; 0x08]>,
    pub virt_size: MulByteView<'a, u32, LitEnd>,
    pub virt_addr: MulByteView<'a, u32, LitEnd>,
    pub size_of_raw_data: MulByteView<'a, u32, LitEnd>,
    pub ptr_to_raw_data: MulByteView<'a, u32, LitEnd>,
    pub ptr_to_relocs: MulByteView<'a, u32, LitEnd>,
    pub ptr_to_line_nums: MulByteView<'a, u32, LitEnd>,
    pub num_of_relocs: MulByteView<'a, u16, LitEnd>,
    pub num_of_line_nums: MulByteView<'a, u16, LitEnd>,
    pub characteristics: MulByteView<'a, u32, LitEnd>,
}
