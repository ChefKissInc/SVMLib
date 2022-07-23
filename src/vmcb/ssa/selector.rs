//! Copyright (c) ChefKiss Inc 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

#![allow(clippy::if_same_then_else, clippy::eq_op)]

use modular_bitfield::prelude::*;

/// VMCB SSA selector structure
#[bitfield(bits = 128)]
#[derive(BitfieldSpecifier, Debug, Default)]
#[repr(u128)]
pub struct VMCBSSASelector {
    pub selector: u16,
    pub attrib: u16,
    pub limit: u32,
    pub base: u64,
}

/// VMCB SSA GDTR and IDTR selector structure
#[bitfield(bits = 128)]
#[derive(BitfieldSpecifier, Debug, Default)]
#[repr(u128)]
pub struct VMCBGDTRegIDTRegSelector {
    #[skip]
    __: u16,
    #[skip]
    __: u16,
    pub limit: u16,
    #[skip]
    __: u16,
    pub base: u64,
}
