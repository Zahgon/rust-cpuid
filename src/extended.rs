
use bitflags::bitflags;
use core::fmt::{self, Debug, Display, Formatter};
use core::mem::size_of;
use core::slice;
use core::str;

use crate::{
    get_bits, CpuIdReader, CpuIdResult, Vendor, EAX_EXTENDED_CPU_TOPOLOGY,
    EAX_PQOS_EXTENDED_FEATURES,
};

pub struct ExtendedProcessorFeatureIdentifiers {
    vendor: Vendor,
    eax: u32,
    ebx: u32,
    ecx: ExtendedFunctionInfoEcx,
    edx: ExtendedFunctionInfoEdx,
}

impl ExtendedProcessorFeatureIdentifiers {
    pub(crate) fn new(vendor: Vendor, data: CpuIdResult) -> Self { panic!("STUB: not implemented") }

    pub fn extended_signature(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn pkg_type(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn brand_id(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn has_lahf_sahf(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_cmp_legacy(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_svm(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_ext_apic_space(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_alt_mov_cr8(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_lzcnt(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_sse4a(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_misaligned_sse_mode(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_prefetchw(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_osvw(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_ibs(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_xop(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_skinit(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_wdt(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_lwp(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_fma4(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_tbm(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_topology_extensions(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_perf_cntr_extensions(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_nb_perf_cntr_extensions(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_data_access_bkpt_extension(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_perf_tsc(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_perf_cntr_llc_extensions(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_monitorx_mwaitx(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_addr_mask_extension(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_syscall_sysret(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_execute_disable(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_mmx_extensions(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_fast_fxsave_fxstor(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_1gib_pages(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_rdtscp(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_64bit_mode(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_amd_3dnow_extensions(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_3dnow(&self) -> bool { panic!("STUB: not implemented") }
}

impl Debug for ExtendedProcessorFeatureIdentifiers {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { panic!("STUB: not implemented") }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ExtendedFunctionInfoEcx: u32 {
        const LAHF_SAHF = 1 << 0;
        const CMP_LEGACY =  1 << 1;
        const SVM = 1 << 2;
        const EXT_APIC_SPACE = 1 << 3;
        const ALTMOVCR8 = 1 << 4;
        const LZCNT = 1 << 5;
        const SSE4A = 1 << 6;
        const MISALIGNSSE = 1 << 7;
        const PREFETCHW = 1 << 8;
        const OSVW = 1 << 9;
        const IBS = 1 << 10;
        const XOP = 1 << 11;
        const SKINIT = 1 << 12;
        const WDT = 1 << 13;
        const LWP = 1 << 15;
        const FMA4 = 1 << 16;
        const TBM = 1 << 21;
        const TOPEXT = 1 << 22;
        const PERFCTREXT = 1 << 23;
        const PERFCTREXTNB = 1 << 24;
        const DATABRKPEXT = 1 << 26;
        const PERFTSC = 1 << 27;
        const PERFCTREXTLLC = 1 << 28;
        const MONITORX = 1 << 29;
        const ADDRMASKEXT = 1 << 30;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ExtendedFunctionInfoEdx: u32 {
        const SYSCALL_SYSRET = 1 << 11;
        const EXECUTE_DISABLE = 1 << 20;
        const MMXEXT = 1 << 22;
        const FFXSR = 1 << 25;
        const GIB_PAGES = 1 << 26;
        const RDTSCP = 1 << 27;
        const I64BIT_MODE = 1 << 29;
        const THREEDNOWEXT = 1 << 30;
        const THREEDNOW = 1 << 31;
    }
}

pub struct ProcessorBrandString {
    data: [CpuIdResult; 3],
}

impl ProcessorBrandString {
    pub(crate) fn new(data: [CpuIdResult; 3]) -> Self { panic!("STUB: not implemented") }

    pub fn as_str(&self) -> &str { panic!("STUB: not implemented") }
}

impl Debug for ProcessorBrandString {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { panic!("STUB: not implemented") }
}

#[derive(PartialEq, Eq, Debug)]
pub struct L1CacheTlbInfo {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
}

impl L1CacheTlbInfo {
    pub(crate) fn new(data: CpuIdResult) -> Self { panic!("STUB: not implemented") }

    pub fn dtlb_2m_4m_associativity(&self) -> Associativity { panic!("STUB: not implemented") }

    pub fn dtlb_2m_4m_size(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn itlb_2m_4m_associativity(&self) -> Associativity { panic!("STUB: not implemented") }

    pub fn itlb_2m_4m_size(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn dtlb_4k_associativity(&self) -> Associativity { panic!("STUB: not implemented") }

    pub fn dtlb_4k_size(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn itlb_4k_associativity(&self) -> Associativity { panic!("STUB: not implemented") }

    pub fn itlb_4k_size(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn dcache_size(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn dcache_associativity(&self) -> Associativity { panic!("STUB: not implemented") }

    pub fn dcache_lines_per_tag(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn dcache_line_size(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn icache_size(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn icache_associativity(&self) -> Associativity { panic!("STUB: not implemented") }

    pub fn icache_lines_per_tag(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn icache_line_size(&self) -> u8 { panic!("STUB: not implemented") }
}

#[derive(PartialEq, Eq, Debug)]
pub struct L2And3CacheTlbInfo {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
}

impl L2And3CacheTlbInfo {
    pub(crate) fn new(data: CpuIdResult) -> Self { panic!("STUB: not implemented") }

    pub fn dtlb_2m_4m_associativity(&self) -> Associativity { panic!("STUB: not implemented") }

    pub fn dtlb_2m_4m_size(&self) -> u16 { panic!("STUB: not implemented") }

    pub fn itlb_2m_4m_associativity(&self) -> Associativity { panic!("STUB: not implemented") }

    pub fn itlb_2m_4m_size(&self) -> u16 { panic!("STUB: not implemented") }

    pub fn dtlb_4k_associativity(&self) -> Associativity { panic!("STUB: not implemented") }

    pub fn dtlb_4k_size(&self) -> u16 { panic!("STUB: not implemented") }

    pub fn itlb_4k_associativity(&self) -> Associativity { panic!("STUB: not implemented") }

    pub fn itlb_4k_size(&self) -> u16 { panic!("STUB: not implemented") }

    pub fn l2cache_line_size(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn l2cache_lines_per_tag(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn l2cache_associativity(&self) -> Associativity { panic!("STUB: not implemented") }

    pub fn l2cache_size(&self) -> u16 { panic!("STUB: not implemented") }

    pub fn l3cache_line_size(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn l3cache_lines_per_tag(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn l3cache_associativity(&self) -> Associativity { panic!("STUB: not implemented") }

    pub fn l3cache_size(&self) -> u16 { panic!("STUB: not implemented") }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Associativity {
    Disabled,
    DirectMapped,
    NWay(u8),
    FullyAssociative,
    Unknown,
}

impl Display for Associativity {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result { panic!("STUB: not implemented") }
}

impl Associativity {
    
    fn for_l1(n: u8) -> Associativity { panic!("STUB: not implemented") }

    fn for_l2(n: u8) -> Associativity { panic!("STUB: not implemented") }

    fn for_l3(n: u8) -> Associativity { panic!("STUB: not implemented") }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ApmInfo {
    
    _eax: u32,
    ebx: RasCapabilities,
    ecx: u32,
    edx: ApmInfoEdx,
}

impl ApmInfo {
    pub(crate) fn new(data: CpuIdResult) -> Self { panic!("STUB: not implemented") }

    pub fn has_mca_overflow_recovery(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_succor(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_hwa(&self) -> bool { panic!("STUB: not implemented") }

    pub fn cpu_pwr_sample_time_ratio(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn has_ts(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_freq_id_ctrl(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_volt_id_ctrl(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_thermtrip(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_tm(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_100mhz_steps(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_hw_pstate(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_invariant_tsc(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_cpb(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_ro_effective_freq_iface(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_feedback_iface(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_power_reporting_iface(&self) -> bool { panic!("STUB: not implemented") }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ApmInfoEdx: u32 {
        const TS = 1 << 0;
        const FID = 1 << 1;
        const VID = 1 << 2;
        const TTP = 1 << 3;
        const TM = 1 << 4;
        const MHZSTEPS100 = 1 << 6;
        const HWPSTATE = 1 << 7;
        const INVTSC = 1 << 8;
        const CPB = 1 << 9;
        const EFFFREQRO = 1 << 10;
        const PROCFEEDBACKIF = 1 << 11;
        const PROCPWRREPORT = 1 << 12;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct RasCapabilities: u32 {
        const MCAOVFLRECOV = 1 << 0;
        const SUCCOR = 1 << 1;
        const HWA = 1 << 2;
    }
}

#[derive(PartialEq, Eq)]
pub struct ProcessorCapacityAndFeatureInfo {
    eax: u32,
    ebx: ProcessorCapacityAndFeatureEbx,
    ecx: u32,
    edx: u32,
}

impl ProcessorCapacityAndFeatureInfo {
    pub(crate) fn new(data: CpuIdResult) -> Self { panic!("STUB: not implemented") }

    pub fn physical_address_bits(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn linear_address_bits(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn guest_physical_address_bits(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn has_cl_zero(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_inst_ret_cntr_msr(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_restore_fp_error_ptrs(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_invlpgb(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_rdpru(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_mcommit(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_wbnoinvd(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_int_wbinvd(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_unsupported_efer_lmsle(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_invlpgb_nested(&self) -> bool { panic!("STUB: not implemented") }

    pub fn perf_tsc_size(&self) -> usize { panic!("STUB: not implemented") }

    pub fn apic_id_size(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn maximum_logical_processors(&self) -> usize { panic!("STUB: not implemented") }

    pub fn num_phys_threads(&self) -> usize { panic!("STUB: not implemented") }

    pub fn invlpgb_max_pages(&self) -> u16 { panic!("STUB: not implemented") }

    pub fn max_rdpru_id(&self) -> u16 { panic!("STUB: not implemented") }
}

impl Debug for ProcessorCapacityAndFeatureInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ProcessorCapacityAndFeatureEbx: u32 {
        const CLZERO = 1 << 0;
        const INST_RETCNT_MSR = 1 << 1;
        const RSTR_FP_ERR_PTRS = 1 << 2;
        const INVLPGB = 1 << 3;
        const RDPRU = 1 << 4;
        const MCOMMIT = 1 << 8;
        const WBNOINVD = 1 << 9;
        const INT_WBINVD = 1 << 13;
        const EFER_LMSLE_UNSUPP = 1 << 20;
        const INVLPGB_NESTED = 1 << 21;
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct SvmFeatures {
    eax: u32,
    ebx: u32,
    
    _ecx: u32,
    edx: SvmFeaturesEdx,
}

impl SvmFeatures {
    pub(crate) fn new(data: CpuIdResult) -> Self { panic!("STUB: not implemented") }

    pub fn revision(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn supported_asids(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn has_nested_paging(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_lbr_virtualization(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_svm_lock(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_nrip(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_tsc_rate_msr(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_vmcb_clean_bits(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_flush_by_asid(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_decode_assists(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_pause_filter(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_pause_filter_threshold(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_avic(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_vmsave_virtualization(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_gif(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_gmet(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_sss_check(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_spec_ctrl(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_host_mce_override(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_tlb_ctrl(&self) -> bool { panic!("STUB: not implemented") }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct SvmFeaturesEdx: u32 {
        const NP = 1 << 0;
        const LBR_VIRT = 1 << 1;
        const SVML = 1 << 2;
        const NRIPS = 1 << 3;
        const TSC_RATE_MSR = 1 << 4;
        const VMCB_CLEAN = 1 << 5;
        const FLUSH_BY_ASID = 1 << 6;
        const DECODE_ASSISTS = 1 << 7;
        const PAUSE_FILTER = 1 << 10;
        const PAUSE_FILTER_THRESHOLD = 1 << 12;
        const AVIC = 1 << 13;
        const VMSAVE_VIRT = 1 << 15;
        const VGIF = 1 << 16;
        const GMET = 1 << 17;
        const SSS_CHECK = 1 << 19;
        const SPEC_CTRL = 1 << 20;
        const HOST_MCE_OVERRIDE = 1 << 23;
        const TLB_CTL = 1 << 24;
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Tlb1gbPageInfo {
    eax: u32,
    ebx: u32,
    
    _ecx: u32,
    
    _edx: u32,
}

impl Tlb1gbPageInfo {
    pub(crate) fn new(data: CpuIdResult) -> Self { panic!("STUB: not implemented") }

    pub fn dtlb_l1_1gb_associativity(&self) -> Associativity { panic!("STUB: not implemented") }

    pub fn dtlb_l1_1gb_size(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn itlb_l1_1gb_associativity(&self) -> Associativity { panic!("STUB: not implemented") }

    pub fn itlb_l1_1gb_size(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn dtlb_l2_1gb_associativity(&self) -> Associativity { panic!("STUB: not implemented") }

    pub fn dtlb_l2_1gb_size(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn itlb_l2_1gb_associativity(&self) -> Associativity { panic!("STUB: not implemented") }

    pub fn itlb_l2_1gb_size(&self) -> u8 { panic!("STUB: not implemented") }
}

#[derive(PartialEq, Eq, Debug)]
pub struct PerformanceOptimizationInfo {
    eax: PerformanceOptimizationInfoEax,
    
    _ebx: u32,
    
    _ecx: u32,
    
    _edx: u32,
}

impl PerformanceOptimizationInfo {
    pub(crate) fn new(data: CpuIdResult) -> Self { panic!("STUB: not implemented") }

    pub fn has_fp128(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_movu(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_fp256(&self) -> bool { panic!("STUB: not implemented") }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct PerformanceOptimizationInfoEax: u32 {
        const FP128 = 1 << 0;
        const MOVU = 1 << 1;
        const FP256 = 1 << 2;
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct InstructionBasedSamplingCapabilities {
    eax: InstructionBasedSamplingCapabilitiesEax,
    
    _ebx: u32,
    
    _ecx: u32,
    
    _edx: u32,
}

impl InstructionBasedSamplingCapabilities {
    pub(crate) fn new(data: CpuIdResult) -> Self { panic!("STUB: not implemented") }

    pub fn has_feature_flags(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_fetch_sampling(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_execution_sampling(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_read_write_operation_counter(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_operation_counter(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_branch_target_address_reporting(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_operation_counter_extended(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_invalid_rip_indication(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_fused_branch_micro_op_indication(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_l3_miss_filtering(&self) -> bool { panic!("STUB: not implemented") }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct InstructionBasedSamplingCapabilitiesEax: u32 {
        const IBSFFV = 1 << 0;
        const FETCH_SAM = 1 << 1;
        const OP_SAM = 1 << 2;
        const RD_WR_OP_CNT = 1 << 3;
        const OP_CNT = 1 << 4;
        const BRN_TRGT = 1 << 5;
        const OP_CNT_EXT = 1 << 6;
        const RIP_INVALID_CHK = 1 << 7;
        const OP_BRN_FUSE = 1 << 8;
        const IBS_L3_MISS_FILTERING = 1 << 11;
    }
}

#[derive(PartialEq, Eq)]
pub struct ProcessorTopologyInfo {
    eax: u32,
    ebx: u32,
    ecx: u32,
    
    _edx: u32,
}

impl ProcessorTopologyInfo {
    pub(crate) fn new(data: CpuIdResult) -> Self { panic!("STUB: not implemented") }

    pub fn x2apic_id(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn core_id(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn threads_per_core(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn node_id(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn nodes_per_processor(&self) -> u8 { panic!("STUB: not implemented") }
}

impl Debug for ProcessorTopologyInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MemoryEncryptionInfo {
    eax: MemoryEncryptionInfoEax,
    ebx: u32,
    ecx: u32,
    edx: u32,
}

impl MemoryEncryptionInfo {
    pub(crate) fn new(data: CpuIdResult) -> Self { panic!("STUB: not implemented") }

    pub fn has_sme(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_sev(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_page_flush_msr(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_sev_es(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_sev_snp(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_vmpl(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_hw_enforced_cache_coh(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_64bit_mode(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_restricted_injection(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_alternate_injection(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_debug_swap(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_prevent_host_ibs(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_vte(&self) -> bool { panic!("STUB: not implemented") }

    pub fn c_bit_position(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn physical_address_reduction(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn max_encrypted_guests(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn min_sev_no_es_asid(&self) -> u32 { panic!("STUB: not implemented") }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct MemoryEncryptionInfoEax: u32 {
        const SME = 1 << 0;
        const SEV = 1 << 1;
        const PAGE_FLUSH_MSR = 1 << 2;
        const SEV_ES = 1 << 3;
        const SEV_SNP = 1 << 4;
        const VMPL = 1 << 5;
        const HWENFCACHECOH = 1 << 10;
        const HOST64 = 1 << 11;
        const RESTINJECT = 1 << 12;
        const ALTINJECT = 1 << 13;
        const DBGSWP = 1 << 14;
        const PREVHOSTIBS = 1 << 15;
        const VTE = 1 << 16;
    }
}

#[derive(PartialEq, Eq)]
pub struct PqosExtendedFeatureInfo<R: CpuIdReader> {
    read: R,
    _eax: u32,
    ebx: PqosExtendedFeatureInfoEbx,
    _ecx: u32,
    _edx: u32,
}

impl<R: CpuIdReader> PqosExtendedFeatureInfo<R> {
    pub(crate) fn new(read: R) -> Self { panic!("STUB: not implemented") }

    pub fn has_l3mbe(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_l3smbe(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_bmec(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_l3rr(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_abmc(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_sdciae(&self) -> bool { panic!("STUB: not implemented") }

    pub fn get_l3_memory_bandwidth_enforcement_info(
        &self,
    ) -> Option<L3MemoryBandwidthEnforcementInformation> { panic!("STUB: not implemented") }

    pub fn get_l3_slow_memory_bandwidth_enforcement_info(
        &self,
    ) -> Option<L3MemoryBandwidthEnforcementInformation> { panic!("STUB: not implemented") }

    pub fn get_bandwidth_monitoring_event_counters_info(
        &self,
    ) -> Option<BandwidthMonitoringEventCounters> { panic!("STUB: not implemented") }

    pub fn get_assignable_bandwidth_monitoring_counters_info(
        &self,
    ) -> Option<AssignableBandwidthMonitoringCounterInfo> { panic!("STUB: not implemented") }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct PqosExtendedFeatureInfoEbx: u32 {
        const L3MBE = 1 << 1;
        const L3SMBE = 1 << 2;
        const BMEC = 1 << 3;
        const L3RR = 1 << 4;
        const ABMC = 1 << 5;
        const SDCIAE = 1 << 6;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct PqosExtendedFeatureInfoEbx5: u32 {
        const SELECT_COS = 1 << 0;
    }
}

impl<R: CpuIdReader> Debug for PqosExtendedFeatureInfo<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

#[derive(PartialEq, Eq, Debug)]
pub struct L3MemoryBandwidthEnforcementInformation {
    eax: u32,
    _ebx: u32,
    _ecx: u32,
    edx: u32,
}

impl L3MemoryBandwidthEnforcementInformation {
    pub(crate) fn new(data: CpuIdResult) -> Self { panic!("STUB: not implemented") }

    pub fn bandwidth_length(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn cos_max(&self) -> u32 { panic!("STUB: not implemented") }
}

#[derive(PartialEq, Eq, Debug)]
pub struct BandwidthMonitoringEventCounters {
    _eax: u32,
    ebx: u32,
    ecx: BandwidthMonitoringEventCountersEcx,
    _edx: u32,
}

impl BandwidthMonitoringEventCounters {
    pub(crate) fn new(data: CpuIdResult) -> Self { panic!("STUB: not implemented") }

    pub fn number_events(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn has_l3_cache_lcl_bw_fill_mon(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_l3_cache_rmt_bw_fill_mon(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_l3_cache_lcl_bw_nt_wr_mon(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_l3_cache_rmt_bw_nt_wr_mon(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_l3_cache_lcl_slow_bw_fill_mon(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_l3_cache_rmt_slow_bw_fill_mon(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_l3_cache_vic_mon(&self) -> bool { panic!("STUB: not implemented") }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct BandwidthMonitoringEventCountersEcx: u32 {
        const L3_CACHE_LCL_BW_FILL_MON = 1 << 0;
        const L3_CACHE_RMT_BW_FILL_MON = 1 << 1;
        const L3_CACHE_LCL_BW_NT_WR_MON = 1 << 2;
        const L3_CACHE_RMT_BW_NT_WR_MON = 1 << 3;
        const L3_CACHE_LCL_SLOW_BW_FILL_MON = 1 << 4;
        const L3_CACHE_RMT_SLOW_BW_FILL_MON = 1 << 5;
        const L3_CACHE_VIC_MON = 1 << 6;
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct AssignableBandwidthMonitoringCounterInfo {
    eax: u32,
    ebx: u32,
    ecx: u32,
    _edx: u32,
}

impl AssignableBandwidthMonitoringCounterInfo {
    pub(crate) fn new(data: CpuIdResult) -> Self { panic!("STUB: not implemented") }

    pub fn counter_size(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn has_overflow_bit(&self) -> bool { panic!("STUB: not implemented") }

    pub fn max_abmc(&self) -> u16 { panic!("STUB: not implemented") }

    pub fn has_select_cos(&self) -> bool { panic!("STUB: not implemented") }
}

#[derive(PartialEq, Eq, Debug)]
pub struct ExtendedFeatureIdentification2 {
    eax: ExtendedFeatureIdentification2Eax,
    ebx: u32,
    _ecx: u32,
    _edx: u32,
}

impl ExtendedFeatureIdentification2 {
    pub(crate) fn new(data: CpuIdResult) -> Self { panic!("STUB: not implemented") }

    pub fn has_no_nested_data_bp(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_lfence_always_serializing(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_smm_pg_cfg_lock(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_null_select_clears_base(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_upper_address_ignore(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_automatic_ibrs(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_no_smm_ctl_msr(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_prefetch_ctl_msr(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_cpuid_user_dis(&self) -> bool { panic!("STUB: not implemented") }

    pub fn microcode_patch_size(&self) -> u16 { panic!("STUB: not implemented") }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ExtendedFeatureIdentification2Eax: u32 {
        const NO_NESTED_DATA_BP = 1 << 0;
        const LFENCE_ALWAYS_SERIALIZING = 1 << 2;
        const SMM_PG_CFG_LOCK = 1 << 3;
        const NULL_SELECT_CLEARS_BASE = 1 << 6;
        const UPPER_ADDRESS_IGNORE = 1 << 7;
        const AUTOMATIC_IBRS = 1 << 8;
        const NO_SMM_CTL_MSR = 1 << 9;
        const PREFETCH_CTL_MSR = 1 << 13;
        const CPUID_USER_DIS = 1 << 17;
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct ExtendedPerformanceMonitoringDebug {
    eax: ExtendedPerformanceMonitoringDebugEax,
    ebx: u32,
    _ecx: u32,
    _edx: u32,
}

impl ExtendedPerformanceMonitoringDebug {
    pub(crate) fn new(data: CpuIdResult) -> Self { panic!("STUB: not implemented") }

    pub fn has_perf_mon_v2(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_lbr_stack(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_lbr_and_pmc_freeze(&self) -> bool { panic!("STUB: not implemented") }

    pub fn num_perf_ctr_core(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn num_lbr_stack_size(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn num_perf_ctr_nb(&self) -> u8 { panic!("STUB: not implemented") }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ExtendedPerformanceMonitoringDebugEax: u32 {
        const PERF_MON_V2 = 1 << 0;
        const LBR_STACK = 1 << 1;
        const LBR_AND_PMC_FREEZE = 1 << 2;
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct MultiKeyEncryptedMemoryCapabilities {
    eax: MultiKeyEncryptedMemoryCapabilitiesEax,
    ebx: u32,
    _ecx: u32,
    _edx: u32,
}

impl MultiKeyEncryptedMemoryCapabilities {
    pub(crate) fn new(data: CpuIdResult) -> Self { panic!("STUB: not implemented") }

    pub fn has_mem_hmk(&self) -> bool { panic!("STUB: not implemented") }

    pub fn max_mem_hmk_encr_key_id(&self) -> u16 { panic!("STUB: not implemented") }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct MultiKeyEncryptedMemoryCapabilitiesEax: u32 {
        const MEM_HMK = 1 << 0;
    }
}

#[derive(Clone)]
pub struct ExtendedCpuTopologyIter<R: CpuIdReader> {
    read: R,
    level: u32,
}

impl<R: CpuIdReader> ExtendedCpuTopologyIter<R> {
    pub fn new(read: R) -> Self { panic!("STUB: not implemented") }
}

#[derive(PartialEq, Eq, Debug)]
pub struct ExtendedCpuTopologyLevel {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
}

impl ExtendedCpuTopologyLevel {
    pub(crate) fn new(data: CpuIdResult) -> Self { panic!("STUB: not implemented") }

    pub fn mask_width(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn has_efficiency_ranking_available(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_heterogeneous_cores(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_asymmetric_topology(&self) -> bool { panic!("STUB: not implemented") }

    pub fn num_logical_processors(&self) -> u16 { panic!("STUB: not implemented") }

    pub fn pwr_efficiency_ranking(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn native_mode_id(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn core_type(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn input_ecx(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn level_type(&self) -> HierarchyLevelType { panic!("STUB: not implemented") }

    pub fn extended_apic_id(&self) -> u32 { panic!("STUB: not implemented") }
}

impl<R: CpuIdReader> Iterator for ExtendedCpuTopologyIter<R> {
    type Item = ExtendedCpuTopologyLevel;

    fn next(&mut self) -> Option<ExtendedCpuTopologyLevel> { panic!("STUB: not implemented") }
}

#[repr(u8)]
#[derive(PartialEq, Eq)]
pub enum HierarchyLevelType {
    Reserved = 0,
    Core = 1,
    Complex = 2,
    Die = 3,
    Socket = 4,
    Unknown(u8),
}

impl From<u8> for HierarchyLevelType {
    fn from(value: u8) -> Self { panic!("STUB: not implemented") }
}

impl Display for HierarchyLevelType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl Debug for HierarchyLevelType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}
