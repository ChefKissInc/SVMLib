use modular_bitfield::prelude::*;

pub use clean_bits::*;
pub use tlb_control::*;

mod clean_bits;
mod tlb_control;

#[bitfield(bits = 8192)]
#[derive(Debug, Clone, Copy)]
#[allow(clippy::module_name_repetitions)]
pub struct VmcbControl {
    pub intercept_crx_read: u16,
    pub intercept_crx_write: u16,
    pub intercept_drx_read: u16,
    pub intercept_drx_write: u16,
    pub intercept_exc_vec: u32,
    pub intercept_intr: bool,
    pub intercept_nmi: bool,
    pub intercept_smi: bool,
    pub intercept_init: bool,
    pub intercept_vintr: bool,
    pub intercept_cr0_except_ts_or_mp: bool,
    pub intercept_read_idtr: bool,
    pub intercept_read_gdtr: bool,
    pub intercept_read_ldtr: bool,
    pub intercept_read_tr: bool,
    pub intercept_write_idtr: bool,
    pub intercept_write_gdtr: bool,
    pub intercept_write_ldtr: bool,
    pub intercept_write_tr: bool,
    pub intercept_rdtsc: bool,
    pub intercept_rdpmc: bool,
    pub intercept_pushf: bool,
    pub intercept_popf: bool,
    pub intercept_cpuid: bool,
    pub intercept_rsm: bool,
    pub intercept_iret: bool,
    pub intercept_intn: bool,
    pub intercept_invd: bool,
    pub intercept_pause: bool,
    pub intercept_hlt: bool,
    pub intercept_invlpg: bool,
    pub intercept_invlpga: bool,
    pub intercept_io_prot: bool,
    pub intercept_msr_prot: bool,
    pub intercept_task_switches: bool,
    pub intercept_ferr_freeze: bool,
    pub intercept_shutdown: bool,
    pub intercept_vmrun: bool,
    pub intercept_vmmcall: bool,
    pub intercept_vmload: bool,
    pub intercept_vmsave: bool,
    pub intercept_stgi: bool,
    pub intercept_clgi: bool,
    pub intercept_skinit: bool,
    pub intercept_rdtscp: bool,
    pub intercept_icebp: bool,
    pub intercept_wbinvd: bool,
    pub intercept_monitor: bool,
    pub intercept_mwait: bool,
    pub intercept_mwait_hw_mon: bool,
    pub intercept_xsetbv: bool,
    pub intercept_rdpru: bool,
    pub intercept_efer_writes_post_finish: bool,
    pub intercept_crx_writes_post_finish: u16,
    pub intercept_invlpgb: bool,
    pub intercept_illegal_invlpgb: bool,
    pub intercept_pcid: bool,
    pub intercept_mcommit: bool,
    /// Only when Fn8000_000A, EDX\[24\] = 1
    pub intercept_tlbsync: bool,
    #[skip]
    __: B128,
    #[skip]
    __: B128,
    #[skip]
    __: B59,
    pub pause_filter_threshold: u16,
    pub pause_filter_count: u16,
    #[skip]
    __: B12,
    pub iopm_base: B52,
    #[skip]
    __: B12,
    pub msrpm_base: B52,
    pub tsc_offset: u64,
    pub guest_asid: u32,
    pub tlb_control: TlbControl,
    #[skip]
    __: B24,
    pub virt_tpr: B4,
    #[skip]
    __: B4,
    /// Ignored when AVIC enabled
    pub virt_intr_pending: bool,
    pub virt_intr_unmask: bool,
    #[skip]
    __: B6,
    /// Ignored when AVIC enabled
    pub virt_intr_priority: B4,
    /// Ignored when AVIC enabled
    pub curr_virt_intr_ignore_tpr: bool,
    #[skip]
    __: B3,
    pub virtualize_intr_masking: bool,
    pub amd_virtual_gif: bool,
    #[skip]
    __: B5,
    pub avic: bool,
    pub intr_vector: u8,
    #[skip]
    __: B24,
    pub interrupt_shadow: bool,
    pub guest_interrupt_mask: bool,
    #[skip]
    __: B62,
    pub exit_code: u64,
    pub exit_info1: u64,
    pub exit_info2: u64,
    pub exit_int_info: u64,
    pub nested_paging: bool,
    pub secure_encrypted_virt: bool,
    pub encrypted_state_for_sev: bool,
    pub guest_mode_exec_trap: bool,
    /// Only when Fn8000_000A, EDX\[19\] = 1
    pub shadow_stack_restrictions_in_np: bool,
    pub virt_transparent_encryption: bool,
    /// Only when Fn8000_000A, EDX\[24\] = 1
    pub invlpgb_tlbsync: bool,
    #[skip]
    __: B57,
    pub avic_apic_bar: B52,
    #[skip]
    __: B12,
    pub guest_ghcb_base: u64,
    pub event_injection: u64,
    pub nested_paging_cr3: u64,
    pub lbr_virt_accel: bool,
    pub virt_vmsave_vmload: bool,
    #[skip]
    __: B62,
    pub vmcb_clean_bits: VmcbCleanField,
    #[skip]
    __: B32,
    pub next_rip: u64,
    pub n_bytes_fetched: u8,
    pub guest_inst_bytes_0: B128,
    pub guest_inst_bytes_1: B128,
    pub guest_inst_bytes_2: B128,
    pub guest_inst_bytes_3: B128,
    pub guest_inst_bytes_4: B128,
    pub guest_inst_bytes_5: B128,
    pub guest_inst_bytes_6: B128,
    pub guest_inst_bytes_7: B64,
    #[skip]
    __: B12,
    pub avic_apic_backing_page_ptr: B40,
    #[skip]
    __: B12,
    #[skip]
    __: B64,
    pub avic_logical_table_ptr: B52,
    #[skip]
    __: B12,
    pub avic_physical_max_idx: u8,
    #[skip]
    __: B4,
    pub avic_phys_table_ptr: B40,
    #[skip]
    __: B12,
    #[skip]
    __: B64,
    #[skip]
    __: B12,
    pub vmsa_ptr: B40,
    #[skip]
    __: B12,
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
}

impl Default for VmcbControl {
    fn default() -> Self {
        Self::new()
    }
}
