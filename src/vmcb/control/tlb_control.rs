use modular_bitfield::prelude::*;

#[derive(BitfieldSpecifier, Debug, Default, Clone, Copy)]
#[bits = 8]
pub enum TlbControl {
    #[default]
    DoNothing = 0x0,
    FlushTlbOnVmrun = 0x1,
    FlushGuestTlbEnts = 0x3,
    FlushGuestNonGlobalTlbEnts = 0x7,
}
