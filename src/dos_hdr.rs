use zordon::prelude::*;

#[derive(MutView)]
pub struct DosHeader<'a> {
    pub mz_sig: MulByteView<'a, u16, LitEnd>,
    pub used_bytes_in_last_page: MulByteView<'a, u16, LitEnd>,
    pub file_size_in_pages: MulByteView<'a, u16, LitEnd>,
    pub num_of_reloc_items: MulByteView<'a, u16, LitEnd>,
    pub header_size_in_paragraphs: MulByteView<'a, u16, LitEnd>,
    pub min_extra_paragraphs: MulByteView<'a, u16, LitEnd>,
    pub max_extra_paragraphs: MulByteView<'a, u16, LitEnd>,
    pub initial_relative_ss: MulByteView<'a, u16, LitEnd>,
    pub initial_sp: MulByteView<'a, u16, LitEnd>,
    pub checksum: MulByteView<'a, u16, LitEnd>,
    pub initial_ip: MulByteView<'a, u16, LitEnd>,
    pub initial_relative_cs: MulByteView<'a, u16, LitEnd>,
    pub addr_of_reloc_table: MulByteView<'a, u16, LitEnd>,
    pub overlay_number: MulByteView<'a, u16, LitEnd>,
    pub reserved_0: ArrayView<'a, [u8; 0x08]>,
    pub oem_id: MulByteView<'a, u16, LitEnd>,
    pub oem_info: MulByteView<'a, u16, LitEnd>,
    pub reserved_1: ArrayView<'a, [u8; 0x14]>,
    pub addr_of_new_exe_hdr: MulByteView<'a, u32, LitEnd>,
}
