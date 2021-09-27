#![allow(clippy::use_self)]

use packed_struct::prelude::*;
use static_assertions::assert_eq_size;

#[derive(PrimitiveEnum_u8, Clone, Copy, PartialEq, Debug)]
pub enum TlbControl {
    DoNothing = 0x0,
    FlushTlbOnVmrun = 0x1,
    FlushGuestTlbEnts = 0x3,
    FlushGuestNonGlobalTlbEnts = 0x7,
}

assert_eq_size!(TlbControl, u8);
