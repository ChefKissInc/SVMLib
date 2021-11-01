/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

#![allow(clippy::if_same_then_else, clippy::eq_op)]

use modular_bitfield::prelude::*;

/// VMCB SSA selector structure
#[bitfield(bits = 128)]
#[repr(C)]
#[derive(BitfieldSpecifier, Debug, Default)]
pub struct VmcbSsaSelector {
    pub selector: u16,
    pub attrib: u16,
    pub limit: u32,
    pub base: u64,
}

/// VMCB SSA selector structure with 32-bit base
#[bitfield(bits = 128)]
#[repr(C)]
#[derive(BitfieldSpecifier, Debug, Default)]
pub struct VmcbSsaBase32Selector {
    pub selector: u16,
    pub attrib: u16,
    pub limit: u32,
    pub base: u32,
    #[skip]
    __: u32,
}

/// VMCB SSA GDTR and IDTR selector structure
#[bitfield(bits = 128)]
#[repr(C)]
#[derive(BitfieldSpecifier, Debug, Default)]
pub struct VmcbGdtrIdtrSelector {
    #[skip]
    __: u16,
    #[skip]
    __: u16,
    pub limit: u16,
    #[skip]
    __: u16,
    pub base: u64,
}
