use zordon::types::*;
use zordon::MutView;

#[derive(MutView)]
pub struct NtHeader<'a> {
    pub sig: MulByteView<'a, u32, LitEnd>,
    pub file_hdr: FileHeader<'a>,
    pub opt_hdr: OptHeader<'a>,
}

#[derive(MutView)]
pub struct FileHeader<'a> {
    pub machine: MulByteView<'a, u16, LitEnd>,
    pub num_of_secs: MulByteView<'a, u16, LitEnd>,
    pub time_data_stamp: MulByteView<'a, u32, LitEnd>,
    pub ptr_to_symbol_table: MulByteView<'a, u32, LitEnd>,
    pub num_of_symbols: MulByteView<'a, u32, LitEnd>,
    pub opt_hdr_size: MulByteView<'a, u16, LitEnd>,
    pub file_characteristics: MulByteView<'a, u16, LitEnd>, // TODO: Think about making this into bitfields struct
}

#[derive(MutView)]
pub struct OptHeader<'a> {
    pub magic: MulByteView<'a, u16, LitEnd>,
    pub major_linker_ver: ByteView<'a, u8>,
    pub minor_linker_ver: ByteView<'a, u8>,
    pub size_of_code: MulByteView<'a, u32, LitEnd>,
    pub size_of_init_data: MulByteView<'a, u32, LitEnd>,
    pub size_of_uninit_data: MulByteView<'a, u32, LitEnd>,
    pub addr_of_entrypoint: MulByteView<'a, u32, LitEnd>,
    pub base_of_code: MulByteView<'a, u32, LitEnd>,
    pub image_base: MulByteView<'a, u64, LitEnd>,
    pub sec_alignment: MulByteView<'a, u32, LitEnd>,
    pub file_alignment: MulByteView<'a, u32, LitEnd>,
    pub major_os_ver: MulByteView<'a, u16, LitEnd>,
    pub minor_os_ver: MulByteView<'a, u16, LitEnd>,
    pub major_image_ver: MulByteView<'a, u16, LitEnd>,
    pub minor_image_ver: MulByteView<'a, u16, LitEnd>,
    pub major_subsystem_ver: MulByteView<'a, u16, LitEnd>,
    pub minor_subsystem_ver: MulByteView<'a, u16, LitEnd>,
    pub win32_ver_val: MulByteView<'a, u32, LitEnd>,
    pub size_of_image: MulByteView<'a, u32, LitEnd>,
    pub size_of_hdrs: MulByteView<'a, u32, LitEnd>,
    pub checksum: MulByteView<'a, u32, LitEnd>,
    pub subsystem: MulByteView<'a, u16, LitEnd>,
    pub dll_characteristics: MulByteView<'a, u16, LitEnd>, // TODO: another one that can be made into bitfields struct
    pub size_of_stack_reservee: MulByteView<'a, u64, LitEnd>,
    pub size_of_stack_commit: MulByteView<'a, u64, LitEnd>,
    pub size_of_heap_reserve: MulByteView<'a, u64, LitEnd>,
    pub size_of_heap_commit: MulByteView<'a, u64, LitEnd>,
    pub loader_flags: MulByteView<'a, u32, LitEnd>,
    pub num_of_rva_and_sizes: MulByteView<'a, u32, LitEnd>,
    pub data_dirs: DataDirectories<'a>,
}

#[derive(MutView)]
pub struct DataDirectories<'a> {
    pub export: Option<DataDirectory<'a>>,
    pub import: Option<DataDirectory<'a>>,
    pub resource: Option<DataDirectory<'a>>,
    pub exception: Option<DataDirectory<'a>>,
    pub security: Option<DataDirectory<'a>>,
    pub base_reloc: Option<DataDirectory<'a>>,
    pub debug: Option<DataDirectory<'a>>,
    pub architecture: Option<DataDirectory<'a>>,
    pub global_ptr: Option<DataDirectory<'a>>,
    pub tls: Option<DataDirectory<'a>>,
    pub load_config: Option<DataDirectory<'a>>,
    pub bound_import: Option<DataDirectory<'a>>,
    pub iat: Option<DataDirectory<'a>>,
    pub delay_import: Option<DataDirectory<'a>>,
    pub com_descriptor: Option<DataDirectory<'a>>,
    pub reserved: Option<DataDirectory<'a>>,
}
#[derive(MutView)]
pub struct DataDirectory<'a> {
    pub virt_addr: MulByteView<'a, u32, LitEnd>,
    pub size: MulByteView<'a, u32, LitEnd>,
}

pub enum DataDirType {
    Import,
    Reloc
}
