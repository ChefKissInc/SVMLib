#![allow(
    clippy::must_use_candidate,
    clippy::map_unwrap_or,
    clippy::used_underscore_binding
)]

use core::mem::size_of;

use packed_struct::prelude::*;
use static_assertions::const_assert_eq;

/// VMCB Clean Field, clear each bit in this to reload guest state
#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering = "msb0", endian = "lsb", size_bytes = "4")]
#[allow(clippy::struct_excessive_bools)]
pub struct VmcbCleanField {
    pub intercepts: bool,
    pub iopm: bool,
    pub guest_asid: bool,
    pub virt_tpr: bool,
    pub nested_paging: bool,
    pub crx: bool,
    pub drx: bool,
    pub dt: bool,
    pub seg: bool,
    pub cr2: bool,
    pub lbr: bool,
    pub avic: bool,
    pub cet: bool,
    _reserved: ReservedZeroes<packed_bits::Bits<19>>,
}

const_assert_eq!(size_of::<<VmcbCleanField as PackedStruct>::ByteArray>(), 4);
