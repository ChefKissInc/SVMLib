#![allow(
    clippy::must_use_candidate,
    clippy::map_unwrap_or,
    clippy::used_underscore_binding
)]

mod clean_bits;
mod tlb_control;

use core::mem::size_of;

pub use clean_bits::*;
use packed_struct::prelude::*;
use static_assertions::const_assert_eq;
pub use tlb_control::*;

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering = "msb0", endian = "lsb")]
#[allow(clippy::struct_excessive_bools)]
pub struct VmcbControl {
    #[packed_field(bits = "0..")]
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

    #[packed_field(element_size_bytes = "8")]
    _reserved: [ReservedZeroes<packed_bits::Bits<64>>; 4],
    _reserved2: ReservedZeroes<packed_bits::Bits<59>>,

    pub pause_filter_threshold: u16,
    pub pause_filter_count: u16,
    _reserved3: ReservedZeroes<packed_bits::Bits<12>>,
    pub iopm_base: Integer<u64, packed_bits::Bits<52>>,
    _reserved4: ReservedZeroes<packed_bits::Bits<12>>,
    pub msrpm_base: Integer<u64, packed_bits::Bits<52>>,
    pub tsc_offset: u64,

    pub guest_asid: u32,
    #[packed_field(size_bytes = "1", ty = "enum")]
    pub tlb_control: TlbControl,
    _reserved5: ReservedZeroes<packed_bits::Bits<24>>,

    pub virt_tpr: Integer<u8, packed_bits::Bits<4>>,
    _reserved6: ReservedZeroes<packed_bits::Bits<4>>,
    /// Ignored when AVIC enabled
    pub virt_intr_pending: bool,
    pub virt_intr_unmask: bool,
    _reserved7: ReservedZeroes<packed_bits::Bits<6>>,
    /// Ignored when AVIC enabled
    pub virt_intr_priority: Integer<u8, packed_bits::Bits<4>>,
    /// Ignored when AVIC enabled
    pub curr_virt_intr_ignore_tpr: bool,
    _reserved8: ReservedZeroes<packed_bits::Bits<3>>,
    pub virtualize_intr_masking: bool,
    pub amd_virtual_gif: bool,
    _reserved9: ReservedZeroes<packed_bits::Bits<5>>,
    pub avic: bool,
    pub intr_vector: u8,
    _reserved10: ReservedZeroes<packed_bits::Bits<24>>,

    pub interrupt_shadow: bool,
    pub guest_interrupt_mask: bool,
    _reserved11: ReservedZeroes<packed_bits::Bits<62>>,

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
    _reserved12: ReservedZeroes<packed_bits::Bits<57>>,

    pub avic_apic_bar: Integer<u64, packed_bits::Bits<52>>,
    _reserved13: ReservedZeroes<packed_bits::Bits<12>>,
    pub guest_ghcb_base: u64,
    pub event_injection: u64,
    pub nested_paging_cr3: u64,

    pub lbr_virt_accel: bool,
    pub virt_vmsave_vmload: bool,
    _reserved14: ReservedZeroes<packed_bits::Bits<62>>,
    #[packed_field(size_bytes = "4")]
    pub vmcb_clean_bits: VmcbCleanField,
    _reserved15: ReservedZeroes<packed_bits::Bits<32>>,

    pub next_rip: u64,

    pub n_bytes_fetched: u8,
    #[packed_field(element_size_bytes = "1")]
    pub guest_inst_bytes: [u8; 15],

    _reserved16: ReservedZeroes<packed_bits::Bits<12>>,
    pub avic_apic_backing_page_ptr: Integer<u64, packed_bits::Bits<40>>,
    _reserved17: ReservedZeroes<packed_bits::Bits<12>>,

    _reserved18: ReservedZeroes<packed_bits::Bits<64>>,

    pub avic_logical_table_ptr: Integer<u64, packed_bits::Bits<52>>,
    _reserved19: ReservedZeroes<packed_bits::Bits<12>>,

    pub avic_physical_max_idx: u8,
    _reserved20: ReservedZeroes<packed_bits::Bits<4>>,
    pub avic_phys_table_ptr: Integer<u64, packed_bits::Bits<40>>,
    _reserved21: ReservedZeroes<packed_bits::Bits<12>>,

    _reserved22: ReservedZeroes<packed_bits::Bits<64>>,

    _reserved23: ReservedZeroes<packed_bits::Bits<12>>,
    pub vmsa_ptr: Integer<u64, packed_bits::Bits<40>>,
    _reserved24: ReservedZeroes<packed_bits::Bits<12>>,

    #[packed_field(element_size_bytes = "1")]
    _reserved25: [ReservedZeroes<packed_bits::Bits<8>>; 752],
}

const_assert_eq!(size_of::<<VmcbControl as PackedStruct>::ByteArray>(), 0x400);
