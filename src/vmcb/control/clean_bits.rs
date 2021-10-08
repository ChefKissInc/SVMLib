use modular_bitfield::prelude::*;

/// VMCB Clean Field, clear each bit in this to reload guest state
#[bitfield(bits = 32)]
#[repr(u32)]
#[derive(BitfieldSpecifier, Debug, Default, Clone, Copy)]
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
    #[skip]
    __: B19,
}
