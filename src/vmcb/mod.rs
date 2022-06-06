//! Copyright (c) VisualDevelopment 2021-2022.
//! This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.

use amd64::registers::msr::ModelSpecificReg;
pub use control::*;
pub use ssa::*;

mod control;
mod ssa;

#[repr(C, packed)]
#[derive(Debug, Default)]
pub struct VMCBData {
    pub control: VMCBControl,
    pub save_state: VMCBStateSave,
}

impl VMCBData {
    pub fn set_pat(&mut self, pat_val: amd64::registers::msr::pat::PageAttributeTable) {
        if self.control.nested_paging() {
            self.save_state.set_guest_pat(pat_val);
        } else {
            unsafe { pat_val.write() };
        }
    }
}
