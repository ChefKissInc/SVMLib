/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

#![allow(clippy::module_name_repetitions, clippy::return_self_not_must_use)]

use modular_bitfield::prelude::*;
pub use selector::*;

mod selector;

/// VMCB State Save Area
#[bitfield(bits = 5312)]
#[derive(Debug, Clone, Copy)]
pub struct VmcbStateSave {
    pub es: VmcbSsaBase32Selector,
    pub cs: VmcbSsaBase32Selector,
    pub ss: VmcbSsaBase32Selector,
    pub ds: VmcbSsaBase32Selector,
    pub fs: VmcbSsaSelector,
    pub gs: VmcbSsaSelector,
    pub gdtr: VmcbGdtrIdtrSelector,
    pub ldtr: VmcbSsaSelector,
    pub idtr: VmcbGdtrIdtrSelector,
    pub tr: VmcbSsaSelector,
    #[skip]
    __: B128,
    #[skip]
    __: B128,
    #[skip]
    __: B88,
    pub cpl: u8,
    #[skip]
    __: u32,
    #[bits = 64]
    pub efer: amd64::registers::msr::Efer,
    #[skip]
    __: B128,
    #[skip]
    __: B128,
    #[skip]
    __: B128,
    #[skip]
    __: B128,
    #[skip]
    __: B128,
    #[skip]
    __: B128,
    #[skip]
    __: B128,
    pub cr4: u64,
    pub cr3: u64,
    pub cr0: u64,
    pub dr7: u64,
    pub dr6: u64,
    pub rflags: u64,
    pub rip: u64,
    #[skip]
    __: B128,
    #[skip]
    __: B128,
    #[skip]
    __: B128,
    #[skip]
    __: B128,
    #[skip]
    __: B128,
    #[skip]
    __: u64,
    pub rsp: u64,
    pub s_cet: u64,
    pub ssp: u64,
    pub isst_addr: u64,
    pub rax: u64,
    pub star: u64,
    pub lstar: u64,
    pub cstar: u64,
    pub sfmask: u64,
    pub kernel_gs_base: u64,
    pub sysenter_cs: u64,
    pub sysenter_esp: u64,
    pub sysenter_eip: u64,
    pub cr2: u64,
    #[skip]
    __: B128,
    #[skip]
    __: B128,
    /// Only if nested paging is enabled
    #[bits = 64]
    pub guest_pat: amd64::registers::msr::Pat,
    /// Only of LBR virtualization is enabled
    pub debug_ctl_msr: u64,
    /// Only of LBR virtualization is enabled
    pub last_branch_from_ip_msr: u64,
    /// Only of LBR virtualization is enabled
    pub last_branch_to_ip_msr: u64,
    pub last_excp_from: u64,
    pub last_excp_to: u64,
}

impl Default for VmcbStateSave {
    fn default() -> Self {
        Self::new()
    }
}
