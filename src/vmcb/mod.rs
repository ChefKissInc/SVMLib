#![allow(clippy::must_use_candidate, clippy::map_unwrap_or)]

mod vmcb_control;

use packed_struct::prelude::*;
pub use vmcb_control::*;

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering = "msb0", endian = "lsb")]
#[allow(clippy::struct_excessive_bools, clippy::used_underscore_binding)]
pub struct Vmcb {
    #[packed_field(bits = "0..", size_bytes="0x400")]
    pub control: VmcbControl,
}
