//! Copyright (c) ChefKiss Inc 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives license.

use modular_bitfield::prelude::*;

#[derive(BitfieldSpecifier, Debug, Default, Clone, Copy)]
#[bits = 8]
pub enum TLBControl {
    #[default]
    DoNothing = 0x0,
    FlushTlbOnVmrun = 0x1,
    FlushGuestTlbEnts = 0x3,
    FlushGuestNonGlobalTlbEnts = 0x7,
}
