use zordon::types::*;
use zordon::MutView;

#[derive(MutView)]
pub struct ImportDescriptor<'a> {
    pub original_first_thunk: MulByteView<'a, u32, LitEnd>,
    pub time_data_stamp: MulByteView<'a, u32, LitEnd>,
    pub forwarder_chain: MulByteView<'a, u32, LitEnd>,
    pub first_thunk: MulByteView<'a, u32, LitEnd>,
}
