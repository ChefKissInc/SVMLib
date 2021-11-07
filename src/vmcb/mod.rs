/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

#![allow(
    clippy::must_use_candidate,
    clippy::map_unwrap_or,
    clippy::unnecessary_cast,
    clippy::cast_possible_truncation
)]

use amd64::registers::msr::Msr;
pub use control::*;
pub use ssa::*;

mod control;
mod ssa;

#[repr(C, packed)]
#[derive(Debug, Default)]
pub struct Vmcb {
    pub control: VmcbControl,
    pub save_state: VmcbStateSave,
}

impl Vmcb {
    pub fn set_pat(&mut self, pat_val: amd64::registers::msr::Pat) {
        if self.control.nested_paging() {
            self.save_state.set_guest_pat(pat_val);
        } else {
            unsafe { pat_val.write() };
        }
    }
}
