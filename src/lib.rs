
#![cfg_attr(not(feature = "std"), no_std)]
#![crate_name = "raw_cpuid"]
#![crate_type = "lib"]

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(feature = "display")]
pub mod display;
mod extended;
#[cfg(test)]
mod tests;

use bitflags::bitflags;
use core::fmt::{self, Debug, Formatter};
use core::mem::size_of;
use core::slice;
use core::str;

#[cfg(feature = "serialize")]
use serde_derive::{Deserialize, Serialize};

pub use extended::*;

#[cfg(any(
    all(target_arch = "x86", not(target_env = "sgx"), target_feature = "sse"),
    all(target_arch = "x86_64", not(target_env = "sgx"))
))]
pub mod native_cpuid {
    use crate::CpuIdResult;

    #[cfg(all(target_arch = "x86", not(target_env = "sgx"), target_feature = "sse"))]
    use core::arch::x86 as arch;
    #[cfg(all(target_arch = "x86_64", not(target_env = "sgx")))]
    use core::arch::x86_64 as arch;

    pub fn cpuid_count(a: u32, c: u32) -> CpuIdResult { panic!("STUB: not implemented") }
    
    #[derive(Clone, Copy)]
    pub struct CpuIdReaderNative;

    impl super::CpuIdReader for CpuIdReaderNative {
        fn cpuid2(&self, eax: u32, ecx: u32) -> CpuIdResult { panic!("STUB: not implemented") }
    }
}

#[cfg(any(
    all(target_arch = "x86", not(target_env = "sgx"), target_feature = "sse"),
    all(target_arch = "x86_64", not(target_env = "sgx"))
))]
pub use native_cpuid::CpuIdReaderNative;

#[cfg(any(
    all(target_arch = "x86", not(target_env = "sgx"), target_feature = "sse"),
    all(target_arch = "x86_64", not(target_env = "sgx"))
))]
#[macro_export]
macro_rules! cpuid {
    ($eax:expr) => {
        $crate::native_cpuid::cpuid_count($eax as u32, 0)
    };

    ($eax:expr, $ecx:expr) => {
        $crate::native_cpuid::cpuid_count($eax as u32, $ecx as u32)
    };
}

fn get_bits(r: u32, from: u32, to: u32) -> u32 { panic!("STUB: not implemented") }

macro_rules! check_flag {
    ($doc:meta, $fun:ident, $flags:ident, $flag:expr) => {
        #[$doc]
        pub fn $fun(&self) -> bool {
            self.$flags.contains($flag)
        }
    };
}

macro_rules! is_bit_set {
    ($field:expr, $bit:expr) => {
        $field & (1 << $bit) > 0
    };
}

macro_rules! check_bit_fn {
    ($doc:meta, $fun:ident, $field:ident, $bit:expr) => {
        #[$doc]
        pub fn $fun(&self) -> bool {
            is_bit_set!(self.$field, $bit)
        }
    };
}

pub trait CpuIdReader: Clone {
    fn cpuid1(&self, eax: u32) -> CpuIdResult { panic!("STUB: not implemented") }
    fn cpuid2(&self, eax: u32, ecx: u32) -> CpuIdResult;
}

impl<F> CpuIdReader for F
where
    F: Fn(u32, u32) -> CpuIdResult + Clone,
{
    fn cpuid2(&self, eax: u32, ecx: u32) -> CpuIdResult { panic!("STUB: not implemented") }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Vendor {
    Intel,
    Amd,
    Unknown(u32, u32, u32),
}

impl Vendor {
    fn from_vendor_leaf(res: CpuIdResult) -> Self { panic!("STUB: not implemented") }
}

#[derive(Clone, Copy)]
pub struct CpuId<R: CpuIdReader> {
    
    read: R,
    
    vendor: Vendor,
    
    supported_leafs: u32,
    
    supported_extended_leafs: u32,
}

#[cfg(any(
    all(target_arch = "x86", not(target_env = "sgx"), target_feature = "sse"),
    all(target_arch = "x86_64", not(target_env = "sgx"))
))]
impl Default for CpuId<CpuIdReaderNative> {
    
    fn default() -> Self { panic!("STUB: not implemented") }
}

#[cfg(any(
    all(target_arch = "x86", not(target_env = "sgx"), target_feature = "sse"),
    all(target_arch = "x86_64", not(target_env = "sgx"))
))]
impl CpuId<CpuIdReaderNative> {
    
    pub fn new() -> Self { panic!("STUB: not implemented") }
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct CpuIdResult {
    
    pub eax: u32,
    
    pub ebx: u32,
    
    pub ecx: u32,
    
    pub edx: u32,
}

impl CpuIdResult {
    pub fn all_zero(&self) -> bool { panic!("STUB: not implemented") }
}

impl Debug for CpuIdResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { panic!("STUB: not implemented") }
}

const EAX_VENDOR_INFO: u32 = 0x0;
const EAX_FEATURE_INFO: u32 = 0x1;
const EAX_CACHE_INFO: u32 = 0x2;
const EAX_PROCESSOR_SERIAL: u32 = 0x3;
const EAX_CACHE_PARAMETERS: u32 = 0x4;
const EAX_MONITOR_MWAIT_INFO: u32 = 0x5;
const EAX_THERMAL_POWER_INFO: u32 = 0x6;
const EAX_STRUCTURED_EXTENDED_FEATURE_INFO: u32 = 0x7;
const EAX_DIRECT_CACHE_ACCESS_INFO: u32 = 0x9;
const EAX_PERFORMANCE_MONITOR_INFO: u32 = 0xA;
const EAX_EXTENDED_TOPOLOGY_INFO: u32 = 0xB;
const EAX_EXTENDED_STATE_INFO: u32 = 0xD;
const EAX_RDT_MONITORING: u32 = 0xF;
const EAX_RDT_ALLOCATION: u32 = 0x10;
const EAX_SGX: u32 = 0x12;
const EAX_TRACE_INFO: u32 = 0x14;
const EAX_TIME_STAMP_COUNTER_INFO: u32 = 0x15;
const EAX_FREQUENCY_INFO: u32 = 0x16;
const EAX_SOC_VENDOR_INFO: u32 = 0x17;
const EAX_DETERMINISTIC_ADDRESS_TRANSLATION_INFO: u32 = 0x18;
const EAX_EXTENDED_TOPOLOGY_INFO_V2: u32 = 0x1F;

const EAX_HYPERVISOR_INFO: u32 = 0x4000_0000;

const EAX_EXTENDED_FUNCTION_INFO: u32 = 0x8000_0000;
const EAX_EXTENDED_PROCESSOR_AND_FEATURE_IDENTIFIERS: u32 = 0x8000_0001;
const EAX_EXTENDED_BRAND_STRING: u32 = 0x8000_0002;
const EAX_L1_CACHE_INFO: u32 = 0x8000_0005;
const EAX_L2_L3_CACHE_INFO: u32 = 0x8000_0006;
const EAX_ADVANCED_POWER_MGMT_INFO: u32 = 0x8000_0007;
const EAX_PROCESSOR_CAPACITY_INFO: u32 = 0x8000_0008;
const EAX_TLB_1GB_PAGE_INFO: u32 = 0x8000_0019;
const EAX_PERFORMANCE_OPTIMIZATION_INFO: u32 = 0x8000_001A;
const EAX_INSTRUCTION_BASED_SAMPLING_CAPABILITIES: u32 = 0x8000_001B;
const EAX_CACHE_PARAMETERS_AMD: u32 = 0x8000_001D;
const EAX_PROCESSOR_TOPOLOGY_INFO: u32 = 0x8000_001E;
const EAX_MEMORY_ENCRYPTION_INFO: u32 = 0x8000_001F;
const EAX_SVM_FEATURES: u32 = 0x8000_000A;
const EAX_PQOS_EXTENDED_FEATURES: u32 = 0x8000_0020;
const EAX_EXTENDED_FEATURE_IDENTIFICATION_2: u32 = 0x8000_0021;
const EAX_EXTENDED_PERFORMANCE_MONITORING_AND_DEBUG: u32 = 0x8000_0022;
const EAX_MULTI_KEY_ENCRYPTED_MEMORY_CAPABILITIES: u32 = 0x8000_0023;
const EAX_EXTENDED_CPU_TOPOLOGY: u32 = 0x8000_0026;

impl<R: CpuIdReader> CpuId<R> {
    
    pub fn with_cpuid_reader(cpuid_fn: R) -> Self { panic!("STUB: not implemented") }

    pub fn with_cpuid_fn(cpuid_fn: R) -> Self { panic!("STUB: not implemented") }

    fn leaf_is_supported(&self, val: u32) -> bool { panic!("STUB: not implemented") }

    pub fn get_vendor_info(&self) -> Option<VendorInfo> { panic!("STUB: not implemented") }

    pub fn get_feature_info(&self) -> Option<FeatureInfo> { panic!("STUB: not implemented") }

    pub fn get_cache_info(&self) -> Option<CacheInfoIter> { panic!("STUB: not implemented") }

    pub fn get_processor_serial(&self) -> Option<ProcessorSerial> { panic!("STUB: not implemented") }

    pub fn get_cache_parameters(&self) -> Option<CacheParametersIter<R>> { panic!("STUB: not implemented") }

    pub fn get_monitor_mwait_info(&self) -> Option<MonitorMwaitInfo> { panic!("STUB: not implemented") }

    pub fn get_thermal_power_info(&self) -> Option<ThermalPowerInfo> { panic!("STUB: not implemented") }

    pub fn get_extended_feature_info(&self) -> Option<ExtendedFeatures> { panic!("STUB: not implemented") }

    pub fn get_direct_cache_access_info(&self) -> Option<DirectCacheAccessInfo> { panic!("STUB: not implemented") }

    pub fn get_performance_monitoring_info(&self) -> Option<PerformanceMonitoringInfo> { panic!("STUB: not implemented") }

    pub fn get_extended_topology_info(&self) -> Option<ExtendedTopologyIter<R>> { panic!("STUB: not implemented") }

    pub fn get_extended_topology_info_v2(&self) -> Option<ExtendedTopologyIter<R>> { panic!("STUB: not implemented") }

    pub fn get_extended_state_info(&self) -> Option<ExtendedStateInfo<R>> { panic!("STUB: not implemented") }

    pub fn get_rdt_monitoring_info(&self) -> Option<RdtMonitoringInfo<R>> { panic!("STUB: not implemented") }

    pub fn get_rdt_allocation_info(&self) -> Option<RdtAllocationInfo<R>> { panic!("STUB: not implemented") }

    pub fn get_sgx_info(&self) -> Option<SgxInfo<R>> { panic!("STUB: not implemented") }

    pub fn get_processor_trace_info(&self) -> Option<ProcessorTraceInfo> { panic!("STUB: not implemented") }

    pub fn get_tsc_info(&self) -> Option<TscInfo> { panic!("STUB: not implemented") }

    pub fn get_processor_frequency_info(&self) -> Option<ProcessorFrequencyInfo> { panic!("STUB: not implemented") }

    pub fn get_soc_vendor_info(&self) -> Option<SoCVendorInfo<R>> { panic!("STUB: not implemented") }

    pub fn get_deterministic_address_translation_info(&self) -> Option<DatIter<R>> { panic!("STUB: not implemented") }

    pub fn get_hypervisor_info(&self) -> Option<HypervisorInfo<R>> { panic!("STUB: not implemented") }

    pub fn get_extended_processor_and_feature_identifiers(
        &self,
    ) -> Option<ExtendedProcessorFeatureIdentifiers> { panic!("STUB: not implemented") }

    pub fn get_processor_brand_string(&self) -> Option<ProcessorBrandString> { panic!("STUB: not implemented") }

    pub fn get_l1_cache_and_tlb_info(&self) -> Option<L1CacheTlbInfo> { panic!("STUB: not implemented") }

    pub fn get_l2_l3_cache_and_tlb_info(&self) -> Option<L2And3CacheTlbInfo> { panic!("STUB: not implemented") }

    pub fn get_advanced_power_mgmt_info(&self) -> Option<ApmInfo> { panic!("STUB: not implemented") }

    pub fn get_processor_capacity_feature_info(&self) -> Option<ProcessorCapacityAndFeatureInfo> { panic!("STUB: not implemented") }

    pub fn get_svm_info(&self) -> Option<SvmFeatures> { panic!("STUB: not implemented") }

    pub fn get_tlb_1gb_page_info(&self) -> Option<Tlb1gbPageInfo> { panic!("STUB: not implemented") }

    pub fn get_performance_optimization_info(&self) -> Option<PerformanceOptimizationInfo> { panic!("STUB: not implemented") }

    pub fn get_instruction_based_sampling_capabilities(
        &self,
    ) -> Option<InstructionBasedSamplingCapabilities> { panic!("STUB: not implemented") }

    pub fn get_processor_topology_info(&self) -> Option<ProcessorTopologyInfo> { panic!("STUB: not implemented") }

    pub fn get_memory_encryption_info(&self) -> Option<MemoryEncryptionInfo> { panic!("STUB: not implemented") }

    pub fn get_pqos_extended_feature_info(&self) -> Option<PqosExtendedFeatureInfo<R>> { panic!("STUB: not implemented") }

    pub fn get_extended_feature_identification_2(&self) -> Option<ExtendedFeatureIdentification2> { panic!("STUB: not implemented") }

    pub fn get_extended_performance_monitoring_and_debug(
        &self,
    ) -> Option<ExtendedPerformanceMonitoringDebug> { panic!("STUB: not implemented") }

    pub fn get_multi_key_encrypted_memory_capabilities(
        &self,
    ) -> Option<MultiKeyEncryptedMemoryCapabilities> { panic!("STUB: not implemented") }

    pub fn get_extended_cpu_topology(&self) -> Option<ExtendedCpuTopologyIter<R>> { panic!("STUB: not implemented") }
}

impl<R: CpuIdReader> Debug for CpuId<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { panic!("STUB: not implemented") }
}

#[derive(PartialEq, Eq)]
#[repr(C)]
pub struct VendorInfo {
    ebx: u32,
    edx: u32,
    ecx: u32,
}

impl VendorInfo {
    
    pub fn as_str(&self) -> &str { panic!("STUB: not implemented") }

    #[deprecated(
        since = "10.0.0",
        note = "Use idiomatic function name `as_str` instead"
    )]
    pub fn as_string(&self) -> &str { panic!("STUB: not implemented") }
}

impl Debug for VendorInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { panic!("STUB: not implemented") }
}

impl fmt::Display for VendorInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { panic!("STUB: not implemented") }
}

#[derive(PartialEq, Eq, Clone)]
pub struct CacheInfoIter {
    current: u32,
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
}

impl Iterator for CacheInfoIter {
    type Item = CacheInfo;

    fn next(&mut self) -> Option<CacheInfo> { panic!("STUB: not implemented") }
}

impl Debug for CacheInfoIter {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { panic!("STUB: not implemented") }
}

#[derive(Copy, Clone, Debug)]
pub enum CacheInfoType {
    General,
    Cache,
    TLB,
    STLB,
    DTLB,
    Prefetch,
}

#[derive(Copy, Clone)]
pub struct CacheInfo {
    
    pub num: u8,
    
    pub typ: CacheInfoType,
}

impl CacheInfo {
    
    pub fn desc(&self) -> &'static str { panic!("STUB: not implemented") }
}

impl Debug for CacheInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl fmt::Display for CacheInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { panic!("STUB: not implemented") }
}

pub const CACHE_INFO_TABLE: [CacheInfo; 108] = [
    CacheInfo {
        num: 0x00,
        typ: CacheInfoType::General,
    },
    CacheInfo {
        num: 0x01,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0x02,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0x03,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0x04,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0x05,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0x06,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x08,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x09,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x0A,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x0B,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0x0C,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x0D,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x0E,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x21,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x22,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x23,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x24,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x25,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x29,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x2C,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x30,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x40,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x41,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x42,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x43,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x44,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x45,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x46,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x47,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x48,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x49,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x4A,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x4B,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x4C,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x4D,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x4E,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x4F,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0x50,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0x51,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0x52,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0x55,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0x56,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0x57,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0x59,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0x5A,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0x5B,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0x5C,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0x5D,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0x60,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x61,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0x63,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0x66,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x67,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x68,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x6A,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x6B,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x6C,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x6D,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x70,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x71,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x72,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x76,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0x78,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x79,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x7A,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x7B,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x7C,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x7D,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x7F,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x80,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x82,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x83,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x84,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x85,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x86,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0x87,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0xB0,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0xB1,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0xB2,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0xB3,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0xB4,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0xB5,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0xB6,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0xBA,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0xC0,
        typ: CacheInfoType::TLB,
    },
    CacheInfo {
        num: 0xC1,
        typ: CacheInfoType::STLB,
    },
    CacheInfo {
        num: 0xC2,
        typ: CacheInfoType::DTLB,
    },
    CacheInfo {
        num: 0xCA,
        typ: CacheInfoType::STLB,
    },
    CacheInfo {
        num: 0xD0,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0xD1,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0xD2,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0xD6,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0xD7,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0xD8,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0xDC,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0xDD,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0xDE,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0xE2,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0xE3,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0xE4,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0xEA,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0xEB,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0xEC,
        typ: CacheInfoType::Cache,
    },
    CacheInfo {
        num: 0xF0,
        typ: CacheInfoType::Prefetch,
    },
    CacheInfo {
        num: 0xF1,
        typ: CacheInfoType::Prefetch,
    },
    CacheInfo {
        num: 0xFE,
        typ: CacheInfoType::General,
    },
    CacheInfo {
        num: 0xFF,
        typ: CacheInfoType::General,
    },
];

#[derive(PartialEq, Eq)]
pub struct ProcessorSerial {
    
    ecx: u32,
    
    edx: u32,
    
    eax: u32,
}

impl ProcessorSerial {
    
    pub fn serial_lower(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn serial_middle(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn serial_upper(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn serial(&self) -> u64 { panic!("STUB: not implemented") }

    pub fn serial_all(&self) -> u128 { panic!("STUB: not implemented") }
}

impl Debug for ProcessorSerial {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { panic!("STUB: not implemented") }
}

pub struct FeatureInfo {
    vendor: Vendor,
    eax: u32,
    ebx: u32,
    edx_ecx: FeatureInfoFlags,
}

impl FeatureInfo {
    
    pub fn extended_family_id(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn extended_model_id(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn base_family_id(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn base_model_id(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn family_id(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn model_id(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn stepping_id(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn brand_index(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn cflush_cache_line_size(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn initial_local_apic_id(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn max_logical_processor_ids(&self) -> u8 { panic!("STUB: not implemented") }

    check_flag!(
        doc = "Streaming SIMD Extensions 3 (SSE3). A value of 1 indicates the processor \
               supports this technology.",
        has_sse3,
        edx_ecx,
        FeatureInfoFlags::SSE3
    );

    check_flag!(
        doc = "PCLMULQDQ. A value of 1 indicates the processor supports the PCLMULQDQ \
               instruction",
        has_pclmulqdq,
        edx_ecx,
        FeatureInfoFlags::PCLMULQDQ
    );

    check_flag!(
        doc = "64-bit DS Area. A value of 1 indicates the processor supports DS area \
               using 64-bit layout",
        has_ds_area,
        edx_ecx,
        FeatureInfoFlags::DTES64
    );

    check_flag!(
        doc = "MONITOR/MWAIT. A value of 1 indicates the processor supports this feature.",
        has_monitor_mwait,
        edx_ecx,
        FeatureInfoFlags::MONITOR
    );

    check_flag!(
        doc = "CPL Qualified Debug Store. A value of 1 indicates the processor supports \
               the extensions to the  Debug Store feature to allow for branch message \
               storage qualified by CPL.",
        has_cpl,
        edx_ecx,
        FeatureInfoFlags::DSCPL
    );

    check_flag!(
        doc = "Virtual Machine Extensions. A value of 1 indicates that the processor \
               supports this technology.",
        has_vmx,
        edx_ecx,
        FeatureInfoFlags::VMX
    );

    check_flag!(
        doc = "Safer Mode Extensions. A value of 1 indicates that the processor supports \
               this technology. See Chapter 5, Safer Mode Extensions Reference.",
        has_smx,
        edx_ecx,
        FeatureInfoFlags::SMX
    );

    check_flag!(
        doc = "Enhanced Intel SpeedStep® technology. A value of 1 indicates that the \
               processor supports this technology.",
        has_eist,
        edx_ecx,
        FeatureInfoFlags::EIST
    );

    check_flag!(
        doc = "Thermal Monitor 2. A value of 1 indicates whether the processor supports \
               this technology.",
        has_tm2,
        edx_ecx,
        FeatureInfoFlags::TM2
    );

    check_flag!(
        doc = "A value of 1 indicates the presence of the Supplemental Streaming SIMD \
               Extensions 3 (SSSE3). A value of 0 indicates the instruction extensions \
               are not present in the processor",
        has_ssse3,
        edx_ecx,
        FeatureInfoFlags::SSSE3
    );

    check_flag!(
        doc = "L1 Context ID. A value of 1 indicates the L1 data cache mode can be set \
               to either adaptive mode or shared mode. A value of 0 indicates this \
               feature is not supported. See definition of the IA32_MISC_ENABLE MSR Bit \
               24 (L1 Data Cache Context Mode) for details.",
        has_cnxtid,
        edx_ecx,
        FeatureInfoFlags::CNXTID
    );

    check_flag!(
        doc = "A value of 1 indicates the processor supports FMA extensions using YMM \
               state.",
        has_fma,
        edx_ecx,
        FeatureInfoFlags::FMA
    );

    check_flag!(
        doc = "CMPXCHG16B Available. A value of 1 indicates that the feature is \
               available. See the CMPXCHG8B/CMPXCHG16B Compare and Exchange Bytes \
               section. 14",
        has_cmpxchg16b,
        edx_ecx,
        FeatureInfoFlags::CMPXCHG16B
    );

    check_flag!(
        doc = "Perfmon and Debug Capability: A value of 1 indicates the processor \
               supports the performance   and debug feature indication MSR \
               IA32_PERF_CAPABILITIES.",
        has_pdcm,
        edx_ecx,
        FeatureInfoFlags::PDCM
    );

    check_flag!(
        doc = "Process-context identifiers. A value of 1 indicates that the processor \
               supports PCIDs and the software may set CR4.PCIDE to 1.",
        has_pcid,
        edx_ecx,
        FeatureInfoFlags::PCID
    );

    check_flag!(
        doc = "A value of 1 indicates the processor supports the ability to prefetch \
               data from a memory mapped device.",
        has_dca,
        edx_ecx,
        FeatureInfoFlags::DCA
    );

    check_flag!(
        doc = "A value of 1 indicates that the processor supports SSE4.1.",
        has_sse41,
        edx_ecx,
        FeatureInfoFlags::SSE41
    );

    check_flag!(
        doc = "A value of 1 indicates that the processor supports SSE4.2.",
        has_sse42,
        edx_ecx,
        FeatureInfoFlags::SSE42
    );

    check_flag!(
        doc = "A value of 1 indicates that the processor supports x2APIC feature.",
        has_x2apic,
        edx_ecx,
        FeatureInfoFlags::X2APIC
    );

    check_flag!(
        doc = "A value of 1 indicates that the processor supports MOVBE instruction.",
        has_movbe,
        edx_ecx,
        FeatureInfoFlags::MOVBE
    );

    check_flag!(
        doc = "A value of 1 indicates that the processor supports the POPCNT instruction.",
        has_popcnt,
        edx_ecx,
        FeatureInfoFlags::POPCNT
    );

    check_flag!(
        doc = "A value of 1 indicates that the processors local APIC timer supports \
               one-shot operation using a TSC deadline value.",
        has_tsc_deadline,
        edx_ecx,
        FeatureInfoFlags::TSC_DEADLINE
    );

    check_flag!(
        doc = "A value of 1 indicates that the processor supports the AESNI instruction \
               extensions.",
        has_aesni,
        edx_ecx,
        FeatureInfoFlags::AESNI
    );

    check_flag!(
        doc = "A value of 1 indicates that the processor supports the XSAVE/XRSTOR \
               processor extended states feature, the XSETBV/XGETBV instructions, and \
               XCR0.",
        has_xsave,
        edx_ecx,
        FeatureInfoFlags::XSAVE
    );

    check_flag!(
        doc = "A value of 1 indicates that the OS has enabled XSETBV/XGETBV instructions \
               to access XCR0, and support for processor extended state management using \
               XSAVE/XRSTOR.",
        has_oxsave,
        edx_ecx,
        FeatureInfoFlags::OSXSAVE
    );

    check_flag!(
        doc = "A value of 1 indicates the processor supports the AVX instruction \
               extensions.",
        has_avx,
        edx_ecx,
        FeatureInfoFlags::AVX
    );

    check_flag!(
        doc = "A value of 1 indicates that processor supports 16-bit floating-point \
               conversion instructions.",
        has_f16c,
        edx_ecx,
        FeatureInfoFlags::F16C
    );

    check_flag!(
        doc = "A value of 1 indicates that processor supports RDRAND instruction.",
        has_rdrand,
        edx_ecx,
        FeatureInfoFlags::RDRAND
    );

    check_flag!(
        doc = "A value of 1 indicates the indicates the presence of a hypervisor.",
        has_hypervisor,
        edx_ecx,
        FeatureInfoFlags::HYPERVISOR
    );

    check_flag!(
        doc = "Floating Point Unit On-Chip. The processor contains an x87 FPU.",
        has_fpu,
        edx_ecx,
        FeatureInfoFlags::FPU
    );

    check_flag!(
        doc = "Virtual 8086 Mode Enhancements. Virtual 8086 mode enhancements, including \
               CR4.VME for controlling the feature, CR4.PVI for protected mode virtual \
               interrupts, software interrupt indirection, expansion of the TSS with the \
               software indirection bitmap, and EFLAGS.VIF and EFLAGS.VIP flags.",
        has_vme,
        edx_ecx,
        FeatureInfoFlags::VME
    );

    check_flag!(
        doc = "Debugging Extensions. Support for I/O breakpoints, including CR4.DE for \
               controlling the feature, and optional trapping of accesses to DR4 and DR5.",
        has_de,
        edx_ecx,
        FeatureInfoFlags::DE
    );

    check_flag!(
        doc = "Page Size Extension. Large pages of size 4 MByte are supported, including \
               CR4.PSE for controlling the feature, the defined dirty bit in PDE (Page \
               Directory Entries), optional reserved bit trapping in CR3, PDEs, and PTEs.",
        has_pse,
        edx_ecx,
        FeatureInfoFlags::PSE
    );

    check_flag!(
        doc = "Time Stamp Counter. The RDTSC instruction is supported, including CR4.TSD \
               for controlling privilege.",
        has_tsc,
        edx_ecx,
        FeatureInfoFlags::TSC
    );

    check_flag!(
        doc = "Model Specific Registers RDMSR and WRMSR Instructions. The RDMSR and \
               WRMSR instructions are supported. Some of the MSRs are implementation \
               dependent.",
        has_msr,
        edx_ecx,
        FeatureInfoFlags::MSR
    );

    check_flag!(
        doc = "Physical Address Extension. Physical addresses greater than 32 bits are \
               supported: extended page table entry formats, an extra level in the page \
               translation tables is defined, 2-MByte pages are supported instead of 4 \
               Mbyte pages if PAE bit is 1.",
        has_pae,
        edx_ecx,
        FeatureInfoFlags::PAE
    );

    check_flag!(
        doc = "Machine Check Exception. Exception 18 is defined for Machine Checks, \
               including CR4.MCE for controlling the feature. This feature does not \
               define the model-specific implementations of machine-check error logging, \
               reporting, and processor shutdowns. Machine Check exception handlers may \
               have to depend on processor version to do model specific processing of \
               the exception, or test for the presence of the Machine Check feature.",
        has_mce,
        edx_ecx,
        FeatureInfoFlags::MCE
    );

    check_flag!(
        doc = "CMPXCHG8B Instruction. The compare-and-exchange 8 bytes (64 bits) \
               instruction is supported (implicitly locked and atomic).",
        has_cmpxchg8b,
        edx_ecx,
        FeatureInfoFlags::CX8
    );

    check_flag!(
        doc = "APIC On-Chip. The processor contains an Advanced Programmable Interrupt \
               Controller (APIC), responding to memory mapped commands in the physical \
               address range FFFE0000H to FFFE0FFFH (by default - some processors permit \
               the APIC to be relocated).",
        has_apic,
        edx_ecx,
        FeatureInfoFlags::APIC
    );

    check_flag!(
        doc = "SYSENTER and SYSEXIT Instructions. The SYSENTER and SYSEXIT and \
               associated MSRs are supported.",
        has_sysenter_sysexit,
        edx_ecx,
        FeatureInfoFlags::SEP
    );

    check_flag!(
        doc = "Memory Type Range Registers. MTRRs are supported. The MTRRcap MSR \
               contains feature bits that describe what memory types are supported, how \
               many variable MTRRs are supported, and whether fixed MTRRs are supported.",
        has_mtrr,
        edx_ecx,
        FeatureInfoFlags::MTRR
    );

    check_flag!(
        doc = "Page Global Bit. The global bit is supported in paging-structure entries \
               that map a page, indicating TLB entries that are common to different \
               processes and need not be flushed. The CR4.PGE bit controls this feature.",
        has_pge,
        edx_ecx,
        FeatureInfoFlags::PGE
    );

    check_flag!(
        doc = "Machine Check Architecture. A value of 1 indicates the Machine Check \
               Architecture of reporting machine errors is supported. The MCG_CAP MSR \
               contains feature bits describing how many banks of error reporting MSRs \
               are supported.",
        has_mca,
        edx_ecx,
        FeatureInfoFlags::MCA
    );

    check_flag!(
        doc = "Conditional Move Instructions. The conditional move instruction CMOV is \
               supported. In addition, if x87 FPU is present as indicated by the \
               CPUID.FPU feature bit, then the FCOMI and FCMOV instructions are supported",
        has_cmov,
        edx_ecx,
        FeatureInfoFlags::CMOV
    );

    check_flag!(
        doc = "Page Attribute Table. Page Attribute Table is supported. This feature \
               augments the Memory Type Range Registers (MTRRs), allowing an operating \
               system to specify attributes of memory accessed through a linear address \
               on a 4KB granularity.",
        has_pat,
        edx_ecx,
        FeatureInfoFlags::PAT
    );

    check_flag!(
        doc = "36-Bit Page Size Extension. 4-MByte pages addressing physical memory \
               beyond 4 GBytes are supported with 32-bit paging. This feature indicates \
               that upper bits of the physical address of a 4-MByte page are encoded in \
               bits 20:13 of the page-directory entry. Such physical addresses are \
               limited by MAXPHYADDR and may be up to 40 bits in size.",
        has_pse36,
        edx_ecx,
        FeatureInfoFlags::PSE36
    );

    check_flag!(
        doc = "Processor Serial Number. The processor supports the 96-bit processor \
               identification number feature and the feature is enabled.",
        has_psn,
        edx_ecx,
        FeatureInfoFlags::PSN
    );

    check_flag!(
        doc = "CLFLUSH Instruction. CLFLUSH Instruction is supported.",
        has_clflush,
        edx_ecx,
        FeatureInfoFlags::CLFSH
    );

    check_flag!(
        doc = "Debug Store. The processor supports the ability to write debug \
               information into a memory resident buffer. This feature is used by the \
               branch trace store (BTS) and processor event-based sampling (PEBS) \
               facilities (see Chapter 23, Introduction to Virtual-Machine Extensions, \
               in the Intel® 64 and IA-32 Architectures Software Developers Manual, \
               Volume 3C).",
        has_ds,
        edx_ecx,
        FeatureInfoFlags::DS
    );

    check_flag!(
        doc = "Thermal Monitor and Software Controlled Clock Facilities. The processor \
               implements internal MSRs that allow processor temperature to be monitored \
               and processor performance to be modulated in predefined duty cycles under \
               software control.",
        has_acpi,
        edx_ecx,
        FeatureInfoFlags::ACPI
    );

    check_flag!(
        doc = "Intel MMX Technology. The processor supports the Intel MMX technology.",
        has_mmx,
        edx_ecx,
        FeatureInfoFlags::MMX
    );

    check_flag!(
        doc = "FXSAVE and FXRSTOR Instructions. The FXSAVE and FXRSTOR instructions are \
               supported for fast save and restore of the floating point context. \
               Presence of this bit also indicates that CR4.OSFXSR is available for an \
               operating system to indicate that it supports the FXSAVE and FXRSTOR \
               instructions.",
        has_fxsave_fxstor,
        edx_ecx,
        FeatureInfoFlags::FXSR
    );

    check_flag!(
        doc = "SSE. The processor supports the SSE extensions.",
        has_sse,
        edx_ecx,
        FeatureInfoFlags::SSE
    );

    check_flag!(
        doc = "SSE2. The processor supports the SSE2 extensions.",
        has_sse2,
        edx_ecx,
        FeatureInfoFlags::SSE2
    );

    check_flag!(
        doc = "Self Snoop. The processor supports the management of conflicting memory \
               types by performing a snoop of its own cache structure for transactions \
               issued to the bus.",
        has_ss,
        edx_ecx,
        FeatureInfoFlags::SS
    );

    check_flag!(
        doc = "Max APIC IDs reserved field is Valid. A value of 0 for HTT indicates \
               there is only a single logical processor in the package and software \
               should assume only a single APIC ID is reserved.  A value of 1 for HTT \
               indicates the value in CPUID.1.EBX\\[23:16\\] (the Maximum number of \
               addressable IDs for logical processors in this package) is valid for the \
               package.",
        has_htt,
        edx_ecx,
        FeatureInfoFlags::HTT
    );

    check_flag!(
        doc = "Thermal Monitor. The processor implements the thermal monitor automatic \
               thermal control circuitry (TCC).",
        has_tm,
        edx_ecx,
        FeatureInfoFlags::TM
    );

    check_flag!(
        doc = "Pending Break Enable. The processor supports the use of the FERR#/PBE# \
               pin when the processor is in the stop-clock state (STPCLK# is asserted) \
               to signal the processor that an interrupt is pending and that the \
               processor should return to normal operation to handle the interrupt. Bit \
               10 (PBE enable) in the IA32_MISC_ENABLE MSR enables this capability.",
        has_pbe,
        edx_ecx,
        FeatureInfoFlags::PBE
    );
}

impl Debug for FeatureInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { panic!("STUB: not implemented") }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct FeatureInfoFlags: u64 {
        
        const SSE3 = 1 << 0;
        
        const PCLMULQDQ = 1 << 1;
        
        const DTES64 = 1 << 2;
        
        const MONITOR = 1 << 3;
        
        const DSCPL = 1 << 4;
        
        const VMX = 1 << 5;
        
        const SMX = 1 << 6;
        
        const EIST = 1 << 7;
        
        const TM2 = 1 << 8;
        
        const SSSE3 = 1 << 9;
        
        const CNXTID = 1 << 10;
        
        const FMA = 1 << 12;
        
        const CMPXCHG16B = 1 << 13;
        
        const PDCM = 1 << 15;
        
        const PCID = 1 << 17;
        
        const DCA = 1 << 18;
        
        const SSE41 = 1 << 19;
        
        const SSE42 = 1 << 20;
        
        const X2APIC = 1 << 21;
        
        const MOVBE = 1 << 22;
        
        const POPCNT = 1 << 23;
        
        const TSC_DEADLINE = 1 << 24;
        
        const AESNI = 1 << 25;
        
        const XSAVE = 1 << 26;
        
        const OSXSAVE = 1 << 27;
        
        const AVX = 1 << 28;
        
        const F16C = 1 << 29;
        
        const RDRAND = 1 << 30;
        
        const HYPERVISOR = 1 << 31;

        const FPU = 1 << 32;
        
        const VME = 1 << (32 + 1);
        
        const DE = 1 << (32 + 2);
        
        const PSE = 1 << (32 + 3);
        
        const TSC = 1 << (32 + 4);
        
        const MSR = 1 << (32 + 5);
        
        const PAE = 1 << (32 + 6);
        
        const MCE = 1 << (32 + 7);
        
        const CX8 = 1 << (32 + 8);
        
        const APIC = 1 << (32 + 9);
        
        const SEP = 1 << (32 + 11);
        
        const MTRR = 1 << (32 + 12);
        
        const PGE = 1 << (32 + 13);
        
        const MCA = 1 << (32 + 14);
        
        const CMOV = 1 << (32 + 15);
        
        const PAT = 1 << (32 + 16);
        
        const PSE36 = 1 << (32 + 17);
        
        const PSN = 1 << (32 + 18);
        
        const CLFSH = 1 << (32 + 19);
        
        const DS = 1 << (32 + 21);
        
        const ACPI = 1 << (32 + 22);
        
        const MMX = 1 << (32 + 23);
        
        const FXSR = 1 << (32 + 24);
        
        const SSE = 1 << (32 + 25);
        
        const SSE2 = 1 << (32 + 26);
        
        const SS = 1 << (32 + 27);
        
        const HTT = 1 << (32 + 28);
        
        const TM = 1 << (32 + 29);
        
        const PBE = 1 << (32 + 31);
    }
}

#[derive(Clone, Copy)]
pub struct CacheParametersIter<R: CpuIdReader> {
    read: R,
    leaf: u32,
    current: u32,
}

impl<R: CpuIdReader> Iterator for CacheParametersIter<R> {
    type Item = CacheParameter;

    fn next(&mut self) -> Option<CacheParameter> { panic!("STUB: not implemented") }
}

impl<R: CpuIdReader> Debug for CacheParametersIter<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { panic!("STUB: not implemented") }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CacheParameter {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
}

#[derive(PartialEq, Eq, Debug)]
pub enum CacheType {
    
    Null = 0,
    
    Data,
    
    Instruction,
    
    Unified,
    
    Reserved,
}

impl fmt::Display for CacheType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { panic!("STUB: not implemented") }
}

impl CacheParameter {
    
    pub fn cache_type(&self) -> CacheType { panic!("STUB: not implemented") }

    pub fn level(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn is_self_initializing(&self) -> bool { panic!("STUB: not implemented") }

    pub fn is_fully_associative(&self) -> bool { panic!("STUB: not implemented") }

    pub fn max_cores_for_cache(&self) -> usize { panic!("STUB: not implemented") }

    pub fn max_cores_for_package(&self) -> usize { panic!("STUB: not implemented") }

    pub fn coherency_line_size(&self) -> usize { panic!("STUB: not implemented") }

    pub fn physical_line_partitions(&self) -> usize { panic!("STUB: not implemented") }

    pub fn associativity(&self) -> usize { panic!("STUB: not implemented") }

    pub fn sets(&self) -> usize { panic!("STUB: not implemented") }

    pub fn is_write_back_invalidate(&self) -> bool { panic!("STUB: not implemented") }

    pub fn is_inclusive(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_complex_indexing(&self) -> bool { panic!("STUB: not implemented") }
}

impl Debug for CacheParameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { panic!("STUB: not implemented") }
}

#[derive(Eq, PartialEq)]
pub struct MonitorMwaitInfo {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
}

impl MonitorMwaitInfo {
    
    pub fn smallest_monitor_line(&self) -> u16 { panic!("STUB: not implemented") }

    pub fn largest_monitor_line(&self) -> u16 { panic!("STUB: not implemented") }

    pub fn extensions_supported(&self) -> bool { panic!("STUB: not implemented") }

    pub fn interrupts_as_break_event(&self) -> bool { panic!("STUB: not implemented") }

    pub fn supported_c0_states(&self) -> u16 { panic!("STUB: not implemented") }

    pub fn supported_c1_states(&self) -> u16 { panic!("STUB: not implemented") }

    pub fn supported_c2_states(&self) -> u16 { panic!("STUB: not implemented") }

    pub fn supported_c3_states(&self) -> u16 { panic!("STUB: not implemented") }

    pub fn supported_c4_states(&self) -> u16 { panic!("STUB: not implemented") }

    pub fn supported_c5_states(&self) -> u16 { panic!("STUB: not implemented") }

    pub fn supported_c6_states(&self) -> u16 { panic!("STUB: not implemented") }

    pub fn supported_c7_states(&self) -> u16 { panic!("STUB: not implemented") }
}

impl Debug for MonitorMwaitInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { panic!("STUB: not implemented") }
}

pub struct ThermalPowerInfo {
    eax: ThermalPowerFeaturesEax,
    ebx: u32,
    ecx: ThermalPowerFeaturesEcx,
    _edx: u32,
}

impl ThermalPowerInfo {
    
    pub fn dts_irq_threshold(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn has_dts(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_turbo_boost(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_arat(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_pln(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_ecmd(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_ptm(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_hwp(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_hwp_notification(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_hwp_activity_window(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_hwp_energy_performance_preference(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_hwp_package_level_request(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_hdc(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_turbo_boost3(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_hwp_capabilities(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_hwp_peci_override(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_flexible_hwp(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_hwp_fast_access_mode(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_ignore_idle_processor_hwp_request(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_hw_coord_feedback(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_energy_bias_pref(&self) -> bool { panic!("STUB: not implemented") }
}

impl Debug for ThermalPowerInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { panic!("STUB: not implemented") }
}

bitflags! {
    struct ThermalPowerFeaturesEax: u32 {
        
        const DTS = 1 << 0;
        
        const TURBO_BOOST = 1 << 1;
        
        const ARAT = 1 << 2;
        
        const RESERVED_3 = 1 << 3;
        
        const PLN = 1 << 4;
        
        const ECMD = 1 << 5;
        
        const PTM = 1 << 6;
        
        const HWP = 1 << 7;
        
        const HWP_NOTIFICATION = 1 << 8;
        
        const HWP_ACTIVITY_WINDOW = 1 << 9;
        
        const HWP_ENERGY_PERFORMANCE_PREFERENCE = 1 << 10;
        
        const HWP_PACKAGE_LEVEL_REQUEST = 1 << 11;
        
        const RESERVED_12 = 1 << 12;
        
        const HDC = 1 << 13;
        
        const TURBO_BOOST_3 = 1 << 14;
        
        const HWP_CAPABILITIES = 1 << 15;
        
        const HWP_PECI_OVERRIDE = 1 << 16;
        
        const FLEXIBLE_HWP = 1 << 17;
        
        const HWP_REQUEST_MSR_FAST_ACCESS = 1 << 18;
        
        const RESERVED_19 = 1 << 19;
        
        const IGNORE_IDLE_PROCESSOR_HWP_REQUEST = 1 << 20;
        
    }
}

bitflags! {
    struct ThermalPowerFeaturesEcx: u32 {
        const HW_COORD_FEEDBACK = 1 << 0;

        const ENERGY_BIAS_PREF = 1 << 3;
    }
}

pub struct ExtendedFeatures {
    _eax: u32,
    ebx: ExtendedFeaturesEbx,
    ecx: ExtendedFeaturesEcx,
    edx: ExtendedFeaturesEdx,
    eax1: ExtendedFeaturesEax1,
    _ebx1: u32,
    _ecx1: u32,
    edx1: ExtendedFeaturesEdx1,
}

impl ExtendedFeatures {
    
    #[inline]
    pub const fn has_fsgsbase(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::FSGSBASE)
    }

    #[inline]
    pub const fn has_tsc_adjust_msr(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::ADJUST_MSR)
    }

    #[inline]
    pub const fn has_bmi1(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::BMI1)
    }

    #[inline]
    pub const fn has_hle(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::HLE)
    }

    #[inline]
    pub const fn has_avx2(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::AVX2)
    }

    #[inline]
    pub const fn has_fdp(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::FDP)
    }

    #[inline]
    pub const fn has_smep(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::SMEP)
    }

    #[inline]
    pub const fn has_bmi2(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::BMI2)
    }

    #[inline]
    pub const fn has_rep_movsb_stosb(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::REP_MOVSB_STOSB)
    }

    #[inline]
    pub const fn has_invpcid(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::INVPCID)
    }

    #[inline]
    pub const fn has_rtm(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::RTM)
    }

    #[inline]
    pub const fn has_rdtm(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::RDTM)
    }

    #[inline]
    pub const fn has_fpu_cs_ds_deprecated(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::DEPRECATE_FPU_CS_DS)
    }

    #[inline]
    pub const fn has_mpx(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::MPX)
    }

    #[inline]
    pub const fn has_rdta(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::RDTA)
    }

    #[inline]
    pub const fn has_rdseed(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::RDSEED)
    }

    #[inline]
    pub const fn has_adx(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::ADX)
    }

    #[inline]
    pub const fn has_smap(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::SMAP)
    }

    #[inline]
    pub const fn has_clflushopt(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::CLFLUSHOPT)
    }

    #[inline]
    pub const fn has_processor_trace(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::PROCESSOR_TRACE)
    }

    #[inline]
    pub const fn has_sha(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::SHA)
    }

    #[inline]
    pub const fn has_sgx(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::SGX)
    }

    #[inline]
    pub const fn has_avx512f(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::AVX512F)
    }

    #[inline]
    pub const fn has_avx512dq(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::AVX512DQ)
    }

    #[inline]
    pub const fn has_avx512_ifma(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::AVX512_IFMA)
    }

    #[inline]
    pub const fn has_avx512pf(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::AVX512PF)
    }

    #[inline]
    pub const fn has_avx512er(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::AVX512ER)
    }

    #[inline]
    pub const fn has_avx512cd(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::AVX512CD)
    }

    #[inline]
    pub const fn has_avx512bw(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::AVX512BW)
    }

    #[inline]
    pub const fn has_avx512vl(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::AVX512VL)
    }

    #[inline]
    pub const fn has_clwb(&self) -> bool {
        self.ebx.contains(ExtendedFeaturesEbx::CLWB)
    }

    #[inline]
    pub const fn has_prefetchwt1(&self) -> bool {
        self.ecx.contains(ExtendedFeaturesEcx::PREFETCHWT1)
    }

    #[inline]
    pub const fn has_avx512vbmi(&self) -> bool {
        self.ecx.contains(ExtendedFeaturesEcx::AVX512VBMI)
    }

    #[inline]
    pub const fn has_umip(&self) -> bool {
        self.ecx.contains(ExtendedFeaturesEcx::UMIP)
    }

    #[inline]
    pub const fn has_pku(&self) -> bool {
        self.ecx.contains(ExtendedFeaturesEcx::PKU)
    }

    #[inline]
    pub const fn has_ospke(&self) -> bool {
        self.ecx.contains(ExtendedFeaturesEcx::OSPKE)
    }

    #[inline]
    pub const fn has_waitpkg(&self) -> bool {
        self.ecx.contains(ExtendedFeaturesEcx::WAITPKG)
    }

    #[deprecated(since = "11.4.0", note = "Please use `has_avx512vbmi2` instead")]
    #[inline]
    pub const fn has_av512vbmi2(&self) -> bool {
        self.ecx.contains(ExtendedFeaturesEcx::AVX512VBMI2)
    }

    #[inline]
    pub const fn has_avx512vbmi2(&self) -> bool {
        self.ecx.contains(ExtendedFeaturesEcx::AVX512VBMI2)
    }

    #[inline]
    pub const fn has_cet_ss(&self) -> bool {
        self.ecx.contains(ExtendedFeaturesEcx::CETSS)
    }

    #[inline]
    pub const fn has_gfni(&self) -> bool {
        self.ecx.contains(ExtendedFeaturesEcx::GFNI)
    }

    #[inline]
    pub const fn has_vaes(&self) -> bool {
        self.ecx.contains(ExtendedFeaturesEcx::VAES)
    }

    #[inline]
    pub const fn has_vpclmulqdq(&self) -> bool {
        self.ecx.contains(ExtendedFeaturesEcx::VPCLMULQDQ)
    }

    #[inline]
    pub const fn has_avx512vnni(&self) -> bool {
        self.ecx.contains(ExtendedFeaturesEcx::AVX512VNNI)
    }

    #[inline]
    pub const fn has_avx512bitalg(&self) -> bool {
        self.ecx.contains(ExtendedFeaturesEcx::AVX512BITALG)
    }

    #[inline]
    pub const fn has_tme_en(&self) -> bool {
        self.ecx.contains(ExtendedFeaturesEcx::TMEEN)
    }

    #[inline]
    pub const fn has_avx512vpopcntdq(&self) -> bool {
        self.ecx.contains(ExtendedFeaturesEcx::AVX512VPOPCNTDQ)
    }

    #[inline]
    pub const fn has_la57(&self) -> bool {
        self.ecx.contains(ExtendedFeaturesEcx::LA57)
    }

    #[inline]
    pub const fn has_rdpid(&self) -> bool {
        self.ecx.contains(ExtendedFeaturesEcx::RDPID)
    }

    #[inline]
    pub const fn has_sgx_lc(&self) -> bool {
        self.ecx.contains(ExtendedFeaturesEcx::SGX_LC)
    }

    #[inline]
    pub fn mawau_value(&self) -> u8 { panic!("STUB: not implemented") }

    #[inline]
    pub const fn has_avx512_4vnniw(&self) -> bool {
        self.edx.contains(ExtendedFeaturesEdx::AVX512_4VNNIW)
    }

    #[inline]
    pub const fn has_avx512_4fmaps(&self) -> bool {
        self.edx.contains(ExtendedFeaturesEdx::AVX512_4FMAPS)
    }

    #[inline]
    pub const fn has_avx512_vp2intersect(&self) -> bool {
        self.edx.contains(ExtendedFeaturesEdx::AVX512_VP2INTERSECT)
    }

    #[inline]
    pub const fn has_amx_bf16(&self) -> bool {
        self.edx.contains(ExtendedFeaturesEdx::AMX_BF16)
    }

    #[inline]
    pub const fn has_avx512_fp16(&self) -> bool {
        self.edx.contains(ExtendedFeaturesEdx::AVX512_FP16)
    }

    #[inline]
    pub const fn has_amx_tile(&self) -> bool {
        self.edx.contains(ExtendedFeaturesEdx::AMX_TILE)
    }

    #[inline]
    pub const fn has_amx_int8(&self) -> bool {
        self.edx.contains(ExtendedFeaturesEdx::AMX_INT8)
    }

    #[inline]
    pub const fn has_avx_vnni(&self) -> bool {
        self.eax1.contains(ExtendedFeaturesEax1::AVX_VNNI)
    }

    #[inline]
    pub const fn has_avx512_bf16(&self) -> bool {
        self.eax1.contains(ExtendedFeaturesEax1::AVX512_BF16)
    }

    #[inline]
    pub const fn has_fzrm(&self) -> bool {
        self.eax1.contains(ExtendedFeaturesEax1::FZRM)
    }

    #[inline]
    pub const fn has_fsrs(&self) -> bool {
        self.eax1.contains(ExtendedFeaturesEax1::FSRS)
    }

    #[inline]
    pub const fn has_fsrcrs(&self) -> bool {
        self.eax1.contains(ExtendedFeaturesEax1::FSRCRS)
    }

    #[inline]
    pub const fn has_hreset(&self) -> bool {
        self.eax1.contains(ExtendedFeaturesEax1::HRESET)
    }

    #[inline]
    pub const fn has_avx_ifma(&self) -> bool {
        self.eax1.contains(ExtendedFeaturesEax1::AVX_IFMA)
    }

    #[inline]
    pub const fn has_lam(&self) -> bool {
        self.eax1.contains(ExtendedFeaturesEax1::LAM)
    }

    #[inline]
    pub const fn has_msrlist(&self) -> bool {
        self.eax1.contains(ExtendedFeaturesEax1::MSRLIST)
    }

    #[inline]
    pub const fn has_invd_disable_post_bios_done(&self) -> bool {
        self.eax1
            .contains(ExtendedFeaturesEax1::INVD_DISABLE_POST_BIOS_DONE)
    }

    #[inline]
    pub const fn has_avx_vnni_int8(&self) -> bool {
        self.edx1.contains(ExtendedFeaturesEdx1::AVX_VNNI_INT8)
    }

    #[inline]
    pub const fn has_avx_ne_convert(&self) -> bool {
        self.edx1.contains(ExtendedFeaturesEdx1::AVX_NE_CONVERT)
    }

    #[inline]
    pub const fn has_avx_vnni_int16(&self) -> bool {
        self.edx1.contains(ExtendedFeaturesEdx1::AVX_VNNI_INT16)
    }

    #[inline]
    pub const fn has_prefetchi(&self) -> bool {
        self.edx1.contains(ExtendedFeaturesEdx1::PREFETCHI)
    }

    #[inline]
    pub const fn has_uiret_uif(&self) -> bool {
        self.edx1.contains(ExtendedFeaturesEdx1::UIRET_UIF)
    }

    #[inline]
    pub const fn has_cet_sss(&self) -> bool {
        self.edx1.contains(ExtendedFeaturesEdx1::CET_SSS)
    }

    #[inline]
    pub const fn has_avx10(&self) -> bool {
        self.edx1.contains(ExtendedFeaturesEdx1::AVX10)
    }
}

impl Debug for ExtendedFeatures {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { panic!("STUB: not implemented") }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ExtendedFeaturesEbx: u32 {
        
        const FSGSBASE = 1 << 0;
        
        const ADJUST_MSR = 1 << 1;
        
        const SGX = 1 << 2;
        
        const BMI1 = 1 << 3;
        
        const HLE = 1 << 4;
        
        const AVX2 = 1 << 5;
        
        const FDP = 1 << 6;
        
        const SMEP = 1 << 7;
        
        const BMI2 = 1 << 8;
        
        const REP_MOVSB_STOSB = 1 << 9;
        
        const INVPCID = 1 << 10;
        
        const RTM = 1 << 11;
        
        const RDTM = 1 << 12;
        
        const DEPRECATE_FPU_CS_DS = 1 << 13;
        
        const MPX = 1 << 14;
        
        const RDTA = 1 << 15;
        
        const AVX512F = 1 << 16;
        
        const AVX512DQ = 1 << 17;
        
        const RDSEED = 1 << 18;
        
        const ADX = 1 << 19;
        
        const SMAP = 1 << 20;
        
        const AVX512_IFMA = 1 << 21;
        
        const CLFLUSHOPT = 1 << 23;
        
        const CLWB = 1 << 24;
        
        const PROCESSOR_TRACE = 1 << 25;
        
        const AVX512PF = 1 << 26;
        
        const AVX512ER = 1 << 27;
        
        const AVX512CD = 1 << 28;
        
        const SHA = 1 << 29;
        
        const AVX512BW = 1 << 30;
        
        const AVX512VL = 1 << 31;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ExtendedFeaturesEcx: u32 {
        
        const PREFETCHWT1 = 1 << 0;
        
        const AVX512VBMI = 1 << 1;
        
        const UMIP = 1 << 2;
        
        const PKU = 1 << 3;
        
        const OSPKE = 1 << 4;
        
        const WAITPKG = 1 << 5;
        
        const AVX512VBMI2 = 1 << 6;
        
        const CETSS = 1 << 7;
        
        const GFNI = 1 << 8;
        
        const VAES = 1 << 9;
        
        const VPCLMULQDQ = 1 << 10;
        
        const AVX512VNNI = 1 << 11;
        
        const AVX512BITALG = 1 << 12;
        
        const TMEEN = 1 << 13;
        
        const AVX512VPOPCNTDQ = 1 << 14;

        const LA57 = 1 << 16;

        const RDPID = 1 << 22;

        const SGX_LC = 1 << 30;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ExtendedFeaturesEdx: u32 {
        
        const AVX512_4VNNIW = 1 << 2;
        
        const AVX512_4FMAPS = 1 << 3;
        
        const AVX512_VP2INTERSECT = 1 << 8;
        
        const AMX_BF16 = 1 << 22;
        
        const AVX512_FP16 = 1 << 23;
        
        const AMX_TILE = 1 << 24;
        
        const AMX_INT8 = 1 << 25;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ExtendedFeaturesEax1: u32 {
        
        const AVX_VNNI = 1 << 4;
        
        const AVX512_BF16 = 1 << 5;
        
        const FZRM = 1 << 10;
        
        const FSRS = 1 << 11;
        
        const FSRCRS = 1 << 12;
        
        const HRESET = 1 << 22;
        
        const AVX_IFMA = 1 << 23;
        
        const LAM = 1 << 26;
        
        const MSRLIST = 1 << 27;
        
        const INVD_DISABLE_POST_BIOS_DONE = 1 << 30;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ExtendedFeaturesEdx1: u32 {
        
        const AVX_VNNI_INT8 = 1 << 4;
        
        const AVX_NE_CONVERT = 1 << 5;
        
        const AVX_VNNI_INT16 = 1 << 10;
        
        const PREFETCHI = 1 << 14;
        
        const UIRET_UIF = 1 << 17;
        
        const CET_SSS = 1 << 18;
        
        const AVX10 = 1 << 19;
    }
}

pub struct DirectCacheAccessInfo {
    eax: u32,
}

impl DirectCacheAccessInfo {
    
    pub fn get_dca_cap_value(&self) -> u32 { panic!("STUB: not implemented") }
}

impl Debug for DirectCacheAccessInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { panic!("STUB: not implemented") }
}

pub struct PerformanceMonitoringInfo {
    eax: u32,
    ebx: PerformanceMonitoringFeaturesEbx,
    _ecx: u32,
    edx: u32,
}

impl PerformanceMonitoringInfo {
    
    pub fn version_id(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn number_of_counters(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn counter_bit_width(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn ebx_length(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn fixed_function_counters(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn fixed_function_counters_bit_width(&self) -> u8 { panic!("STUB: not implemented") }

    check_bit_fn!(
        doc = "AnyThread deprecation",
        has_any_thread_deprecation,
        edx,
        15
    );

    check_flag!(
        doc = "Core cycle event not available if 1.",
        is_core_cyc_ev_unavailable,
        ebx,
        PerformanceMonitoringFeaturesEbx::CORE_CYC_EV_UNAVAILABLE
    );

    check_flag!(
        doc = "Instruction retired event not available if 1.",
        is_inst_ret_ev_unavailable,
        ebx,
        PerformanceMonitoringFeaturesEbx::INST_RET_EV_UNAVAILABLE
    );

    check_flag!(
        doc = "Reference cycles event not available if 1.",
        is_ref_cycle_ev_unavailable,
        ebx,
        PerformanceMonitoringFeaturesEbx::REF_CYC_EV_UNAVAILABLE
    );

    check_flag!(
        doc = "Last-level cache reference event not available if 1.",
        is_cache_ref_ev_unavailable,
        ebx,
        PerformanceMonitoringFeaturesEbx::CACHE_REF_EV_UNAVAILABLE
    );

    check_flag!(
        doc = "Last-level cache misses event not available if 1.",
        is_ll_cache_miss_ev_unavailable,
        ebx,
        PerformanceMonitoringFeaturesEbx::LL_CACHE_MISS_EV_UNAVAILABLE
    );

    check_flag!(
        doc = "Branch instruction retired event not available if 1.",
        is_branch_inst_ret_ev_unavailable,
        ebx,
        PerformanceMonitoringFeaturesEbx::BRANCH_INST_RET_EV_UNAVAILABLE
    );

    check_flag!(
        doc = "Branch mispredict retired event not available if 1.",
        is_branch_midpred_ev_unavailable,
        ebx,
        PerformanceMonitoringFeaturesEbx::BRANCH_MISPRED_EV_UNAVAILABLE
    );
}

impl Debug for PerformanceMonitoringInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { panic!("STUB: not implemented") }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct PerformanceMonitoringFeaturesEbx: u32 {
        
        const CORE_CYC_EV_UNAVAILABLE = 1 << 0;
        
        const INST_RET_EV_UNAVAILABLE = 1 << 1;
        
        const REF_CYC_EV_UNAVAILABLE = 1 << 2;
        
        const CACHE_REF_EV_UNAVAILABLE = 1 << 3;
        
        const LL_CACHE_MISS_EV_UNAVAILABLE = 1 << 4;
        
        const BRANCH_INST_RET_EV_UNAVAILABLE = 1 << 5;
        
        const BRANCH_MISPRED_EV_UNAVAILABLE = 1 << 6;
    }
}

#[derive(Clone)]
pub struct ExtendedTopologyIter<R: CpuIdReader> {
    read: R,
    level: u32,
    is_v2: bool,
}

#[derive(PartialEq, Eq)]
pub struct ExtendedTopologyLevel {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
}

impl fmt::Debug for ExtendedTopologyLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { panic!("STUB: not implemented") }
}

impl ExtendedTopologyLevel {
    
    pub fn processors(&self) -> u16 { panic!("STUB: not implemented") }

    pub fn level_number(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn level_type(&self) -> TopologyType { panic!("STUB: not implemented") }

    pub fn x2apic_id(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn shift_right_for_next_apic_id(&self) -> u32 { panic!("STUB: not implemented") }
}

#[derive(PartialEq, Eq, Debug)]
pub enum TopologyType {
    Invalid = 0,
    
    SMT = 1,
    Core = 2,
    Module = 3,
    Tile = 4,
    Die = 5,
}

impl fmt::Display for TopologyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { panic!("STUB: not implemented") }
}

impl<R: CpuIdReader> Iterator for ExtendedTopologyIter<R> {
    type Item = ExtendedTopologyLevel;

    fn next(&mut self) -> Option<ExtendedTopologyLevel> { panic!("STUB: not implemented") }
}

impl<R: CpuIdReader> Debug for ExtendedTopologyIter<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { panic!("STUB: not implemented") }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ExtendedStateInfoXCR0Flags: u32 {
        
        const LEGACY_X87 = 1 << 0;

        const SSE128 = 1 << 1;

        const AVX256 = 1 << 2;

        const MPX_BNDREGS = 1 << 3;

        const MPX_BNDCSR = 1 << 4;

        const AVX512_OPMASK = 1 << 5;

        const AVX512_ZMM_HI256 = 1 << 6;

        const AVX512_ZMM_HI16 = 1 << 7;

        const PKRU = 1 << 9;

        const IA32_XSS_HDC = 1 << 13;

        const AMX_TILECFG = 1 << 17;

        const AMX_TILEDATA = 1 << 18;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ExtendedStateInfoXSSFlags: u32 {
        
        const PT = 1 << 8;

        const PASID = 1 << 10;

        const CET_USER = 1 << 11;

        const CET_SUPERVISOR = 1 << 12;

        const HDC = 1 << 13;

        const UINTR = 1 << 14;

        const LBR = 1 << 15;

        const HWP = 1 << 16;
    }
}

pub struct ExtendedStateInfo<R: CpuIdReader> {
    read: R,
    eax: ExtendedStateInfoXCR0Flags,
    ebx: u32,
    ecx: u32,
    _edx: u32,
    eax1: u32,
    ebx1: u32,
    ecx1: ExtendedStateInfoXSSFlags,
    _edx1: u32,
}

impl<F: CpuIdReader> ExtendedStateInfo<F> {
    check_flag!(
        doc = "Support for legacy x87 in XCR0.",
        xcr0_supports_legacy_x87,
        eax,
        ExtendedStateInfoXCR0Flags::LEGACY_X87
    );

    check_flag!(
        doc = "Support for SSE 128-bit in XCR0.",
        xcr0_supports_sse_128,
        eax,
        ExtendedStateInfoXCR0Flags::SSE128
    );

    check_flag!(
        doc = "Support for AVX 256-bit in XCR0.",
        xcr0_supports_avx_256,
        eax,
        ExtendedStateInfoXCR0Flags::AVX256
    );

    check_flag!(
        doc = "Support for MPX BNDREGS in XCR0.",
        xcr0_supports_mpx_bndregs,
        eax,
        ExtendedStateInfoXCR0Flags::MPX_BNDREGS
    );

    check_flag!(
        doc = "Support for MPX BNDCSR in XCR0.",
        xcr0_supports_mpx_bndcsr,
        eax,
        ExtendedStateInfoXCR0Flags::MPX_BNDCSR
    );

    check_flag!(
        doc = "Support for AVX512 OPMASK in XCR0.",
        xcr0_supports_avx512_opmask,
        eax,
        ExtendedStateInfoXCR0Flags::AVX512_OPMASK
    );

    check_flag!(
        doc = "Support for AVX512 ZMM Hi256 XCR0.",
        xcr0_supports_avx512_zmm_hi256,
        eax,
        ExtendedStateInfoXCR0Flags::AVX512_ZMM_HI256
    );

    check_flag!(
        doc = "Support for AVX512 ZMM Hi16 in XCR0.",
        xcr0_supports_avx512_zmm_hi16,
        eax,
        ExtendedStateInfoXCR0Flags::AVX512_ZMM_HI16
    );

    check_flag!(
        doc = "Support for PKRU in XCR0.",
        xcr0_supports_pkru,
        eax,
        ExtendedStateInfoXCR0Flags::PKRU
    );

    check_flag!(
        doc = "Support for PT in IA32_XSS.",
        ia32_xss_supports_pt,
        ecx1,
        ExtendedStateInfoXSSFlags::PT
    );

    check_flag!(
        doc = "Support for HDC in IA32_XSS.",
        ia32_xss_supports_hdc,
        ecx1,
        ExtendedStateInfoXSSFlags::HDC
    );

    pub fn xsave_area_size_enabled_features(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn xsave_area_size_supported_features(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn has_xsaveopt(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_xsavec(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_xgetbv(&self) -> bool { panic!("STUB: not implemented") }

    pub fn has_xsaves_xrstors(&self) -> bool { panic!("STUB: not implemented") }

    pub fn xsave_size(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn iter(&self) -> ExtendedStateIter<F> { panic!("STUB: not implemented") }
}

impl<R: CpuIdReader> Debug for ExtendedStateInfo<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { panic!("STUB: not implemented") }
}

#[derive(Clone)]
pub struct ExtendedStateIter<R: CpuIdReader> {
    read: R,
    level: u32,
    supported_xcr0: u32,
    supported_xss: u32,
}

impl<R: CpuIdReader> Iterator for ExtendedStateIter<R> {
    type Item = ExtendedState;

    fn next(&mut self) -> Option<ExtendedState> { panic!("STUB: not implemented") }
}

impl<R: CpuIdReader> Debug for ExtendedStateIter<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

#[derive(PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ExtendedRegisterType {
    Avx,
    MpxBndregs,
    MpxBndcsr,
    Avx512Opmask,
    Avx512ZmmHi256,
    Avx512ZmmHi16,
    Pt,
    Pkru,
    Hdc,
    Unknown(u32),
}

impl From<u32> for ExtendedRegisterType {
    fn from(value: u32) -> ExtendedRegisterType { panic!("STUB: not implemented") }
}

impl fmt::Display for ExtendedRegisterType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { panic!("STUB: not implemented") }
}

#[derive(PartialEq, Eq, Debug)]
pub enum ExtendedRegisterStateLocation {
    Xcr0,
    Ia32Xss,
}

impl fmt::Display for ExtendedRegisterStateLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { panic!("STUB: not implemented") }
}

pub struct ExtendedState {
    pub subleaf: u32,
    eax: u32,
    ebx: u32,
    ecx: u32,
}

impl ExtendedState {
    
    pub fn register(&self) -> ExtendedRegisterType { panic!("STUB: not implemented") }

    pub fn size(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn offset(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn location(&self) -> ExtendedRegisterStateLocation { panic!("STUB: not implemented") }

    pub fn is_in_ia32_xss(&self) -> bool { panic!("STUB: not implemented") }

    pub fn is_in_xcr0(&self) -> bool { panic!("STUB: not implemented") }

    pub fn is_compacted_format(&self) -> bool { panic!("STUB: not implemented") }
}

impl Debug for ExtendedState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

pub struct RdtMonitoringInfo<R: CpuIdReader> {
    read: R,
    ebx: u32,
    edx: u32,
}

impl<R: CpuIdReader> RdtMonitoringInfo<R> {
    
    pub fn rmid_range(&self) -> u32 { panic!("STUB: not implemented") }

    check_bit_fn!(
        doc = "Supports L3 Cache Intel RDT Monitoring.",
        has_l3_monitoring,
        edx,
        1
    );

    pub fn l3_monitoring(&self) -> Option<L3MonitoringInfo> { panic!("STUB: not implemented") }
}

impl<R: CpuIdReader> Debug for RdtMonitoringInfo<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { panic!("STUB: not implemented") }
}

pub struct L3MonitoringInfo {
    ebx: u32,
    ecx: u32,
    edx: u32,
}

impl L3MonitoringInfo {
    
    pub fn conversion_factor(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn maximum_rmid_range(&self) -> u32 { panic!("STUB: not implemented") }

    check_bit_fn!(
        doc = "Supports occupancy monitoring.",
        has_occupancy_monitoring,
        edx,
        0
    );

    check_bit_fn!(
        doc = "Supports total bandwidth monitoring.",
        has_total_bandwidth_monitoring,
        edx,
        1
    );

    check_bit_fn!(
        doc = "Supports local bandwidth monitoring.",
        has_local_bandwidth_monitoring,
        edx,
        2
    );
}

impl Debug for L3MonitoringInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { panic!("STUB: not implemented") }
}

pub struct RdtAllocationInfo<R: CpuIdReader> {
    read: R,
    ebx: u32,
}

impl<R: CpuIdReader> RdtAllocationInfo<R> {
    check_bit_fn!(doc = "Supports L3 Cache Allocation.", has_l3_cat, ebx, 1);

    check_bit_fn!(doc = "Supports L2 Cache Allocation.", has_l2_cat, ebx, 2);

    check_bit_fn!(
        doc = "Supports Memory Bandwidth Allocation.",
        has_memory_bandwidth_allocation,
        ebx,
        3
    );

    pub fn l3_cat(&self) -> Option<L3CatInfo> { panic!("STUB: not implemented") }

    pub fn l2_cat(&self) -> Option<L2CatInfo> { panic!("STUB: not implemented") }

    pub fn memory_bandwidth_allocation(&self) -> Option<MemBwAllocationInfo> { panic!("STUB: not implemented") }
}

impl<R: CpuIdReader> Debug for RdtAllocationInfo<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { panic!("STUB: not implemented") }
}

pub struct L3CatInfo {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
}

impl L3CatInfo {
    
    pub fn capacity_mask_length(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn isolation_bitmap(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn highest_cos(&self) -> u16 { panic!("STUB: not implemented") }

    check_bit_fn!(
        doc = "Is Code and Data Prioritization Technology supported?",
        has_code_data_prioritization,
        ecx,
        2
    );
}

impl Debug for L3CatInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { panic!("STUB: not implemented") }
}

#[derive(Eq, PartialEq)]
pub struct L2CatInfo {
    eax: u32,
    ebx: u32,
    edx: u32,
}

impl L2CatInfo {
    
    pub fn capacity_mask_length(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn isolation_bitmap(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn highest_cos(&self) -> u16 { panic!("STUB: not implemented") }
}

impl Debug for L2CatInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { panic!("STUB: not implemented") }
}

#[derive(Eq, PartialEq)]
pub struct MemBwAllocationInfo {
    eax: u32,
    ecx: u32,
    edx: u32,
}

impl MemBwAllocationInfo {
    
    pub fn max_hba_throttling(&self) -> u16 { panic!("STUB: not implemented") }

    pub fn highest_cos(&self) -> u16 { panic!("STUB: not implemented") }

    check_bit_fn!(
        doc = "Reports whether the response of the delay values is linear.",
        has_linear_response_delay,
        ecx,
        2
    );
}

impl Debug for MemBwAllocationInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { panic!("STUB: not implemented") }
}

pub struct SgxInfo<R: CpuIdReader> {
    read: R,
    eax: u32,
    ebx: u32,
    _ecx: u32,
    edx: u32,
    eax1: u32,
    ebx1: u32,
    ecx1: u32,
    edx1: u32,
}

impl<F: CpuIdReader> SgxInfo<F> {
    check_bit_fn!(doc = "Has SGX1 support.", has_sgx1, eax, 0);
    check_bit_fn!(doc = "Has SGX2 support.", has_sgx2, eax, 1);

    check_bit_fn!(
        doc = "Supports ENCLV instruction leaves EINCVIRTCHILD, EDECVIRTCHILD, and ESETCONTEXT.",
        has_enclv_leaves_einvirtchild_edecvirtchild_esetcontext,
        eax,
        5
    );

    check_bit_fn!(
        doc = "Supports ENCLS instruction leaves ETRACKC, ERDINFO, ELDBC, and ELDUC.",
        has_encls_leaves_etrackc_erdinfo_eldbc_elduc,
        eax,
        6
    );

    pub fn miscselect(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn max_enclave_size_non_64bit(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn max_enclave_size_64bit(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn secs_attributes(&self) -> (u64, u64) { panic!("STUB: not implemented") }
    
    pub fn iter(&self) -> SgxSectionIter<F> { panic!("STUB: not implemented") }
}

impl<R: CpuIdReader> Debug for SgxInfo<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { panic!("STUB: not implemented") }
}

#[derive(Clone)]
pub struct SgxSectionIter<R: CpuIdReader> {
    read: R,
    current: u32,
}

impl<R: CpuIdReader> Iterator for SgxSectionIter<R> {
    type Item = SgxSectionInfo;

    fn next(&mut self) -> Option<SgxSectionInfo> { panic!("STUB: not implemented") }
}

impl<R: CpuIdReader> Debug for SgxSectionIter<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { panic!("STUB: not implemented") }
}

#[derive(Debug)]
pub enum SgxSectionInfo {
    
    Epc(EpcSection),
}

#[derive(Debug)]
pub struct EpcSection {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
}

impl EpcSection {
    
    pub fn physical_base(&self) -> u64 { panic!("STUB: not implemented") }

    pub fn size(&self) -> u64 { panic!("STUB: not implemented") }
}

pub struct ProcessorTraceInfo {
    _eax: u32,
    ebx: u32,
    ecx: u32,
    _edx: u32,
    leaf1: Option<CpuIdResult>,
}

impl ProcessorTraceInfo {
    
    check_bit_fn!(
        doc = "If true, Indicates that IA32_RTIT_CTL.CR3Filter can be set to 1, and \
               that IA32_RTIT_CR3_MATCH MSR can be accessed.",
        has_rtit_cr3_match,
        ebx,
        0
    );
    check_bit_fn!(
        doc = "If true, Indicates support of Configurable PSB and Cycle-Accurate Mode.",
        has_configurable_psb_and_cycle_accurate_mode,
        ebx,
        1
    );
    check_bit_fn!(
        doc = "If true, Indicates support of IP Filtering, TraceStop filtering, and \
               preservation of Intel PT MSRs across warm reset.",
        has_ip_tracestop_filtering,
        ebx,
        2
    );
    check_bit_fn!(
        doc = "If true, Indicates support of MTC timing packet and suppression of \
               COFI-based packets.",
        has_mtc_timing_packet_coefi_suppression,
        ebx,
        3
    );

    check_bit_fn!(
        doc = "Indicates support of PTWRITE. Writes can set IA32_RTIT_CTL\\[12\\] (PTWEn \
               and IA32_RTIT_CTL\\[5\\] (FUPonPTW), and PTWRITE can generate packets",
        has_ptwrite,
        ebx,
        4
    );

    check_bit_fn!(
        doc = "Support of Power Event Trace. Writes can set IA32_RTIT_CTL\\[4\\] (PwrEvtEn) \
               enabling Power Event Trace packet generation.",
        has_power_event_trace,
        ebx,
        5
    );

    check_bit_fn!(
        doc = "If true, Tracing can be enabled with IA32_RTIT_CTL.ToPA = 1, hence \
               utilizing the ToPA output scheme; IA32_RTIT_OUTPUT_BASE and \
               IA32_RTIT_OUTPUT_MASK_PTRS MSRs can be accessed.",
        has_topa,
        ecx,
        0
    );
    check_bit_fn!(
        doc = "If true, ToPA tables can hold any number of output entries, up to the \
               maximum allowed by the MaskOrTableOffset field of \
               IA32_RTIT_OUTPUT_MASK_PTRS.",
        has_topa_maximum_entries,
        ecx,
        1
    );
    check_bit_fn!(
        doc = "If true, Indicates support of Single-Range Output scheme.",
        has_single_range_output_scheme,
        ecx,
        2
    );
    check_bit_fn!(
        doc = "If true, Indicates support of output to Trace Transport subsystem.",
        has_trace_transport_subsystem,
        ecx,
        3
    );
    check_bit_fn!(
        doc = "If true, Generated packets which contain IP payloads have LIP values, \
               which include the CS base component.",
        has_lip_with_cs_base,
        ecx,
        31
    );

    pub fn configurable_address_ranges(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn supported_mtc_period_encodings(&self) -> u16 { panic!("STUB: not implemented") }

    pub fn supported_cycle_threshold_value_encodings(&self) -> u16 { panic!("STUB: not implemented") }

    pub fn supported_psb_frequency_encodings(&self) -> u16 { panic!("STUB: not implemented") }
}

impl Debug for ProcessorTraceInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { panic!("STUB: not implemented") }
}

pub struct TscInfo {
    eax: u32,
    ebx: u32,
    ecx: u32,
}

impl fmt::Debug for TscInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { panic!("STUB: not implemented") }
}

impl TscInfo {
    
    pub fn denominator(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn numerator(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn nominal_frequency(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn tsc_frequency(&self) -> Option<u64> { panic!("STUB: not implemented") }
}

pub struct ProcessorFrequencyInfo {
    eax: u32,
    ebx: u32,
    ecx: u32,
}

impl ProcessorFrequencyInfo {
    
    pub fn processor_base_frequency(&self) -> u16 { panic!("STUB: not implemented") }

    pub fn processor_max_frequency(&self) -> u16 { panic!("STUB: not implemented") }

    pub fn bus_frequency(&self) -> u16 { panic!("STUB: not implemented") }
}

impl fmt::Debug for ProcessorFrequencyInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { panic!("STUB: not implemented") }
}

#[derive(Clone)]
pub struct DatIter<R: CpuIdReader> {
    read: R,
    current: u32,
    count: u32,
}

impl<R: CpuIdReader> Iterator for DatIter<R> {
    type Item = DatInfo;

    fn next(&mut self) -> Option<DatInfo> { panic!("STUB: not implemented") }
}

impl<R: CpuIdReader> Debug for DatIter<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

pub struct DatInfo {
    _eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
}

impl DatInfo {
    check_bit_fn!(
        doc = "4K page size entries supported by this structure",
        has_4k_entries,
        ebx,
        0
    );

    check_bit_fn!(
        doc = "2MB page size entries supported by this structure",
        has_2mb_entries,
        ebx,
        1
    );

    check_bit_fn!(
        doc = "4MB page size entries supported by this structure",
        has_4mb_entries,
        ebx,
        2
    );

    check_bit_fn!(
        doc = "1GB page size entries supported by this structure",
        has_1gb_entries,
        ebx,
        3
    );

    check_bit_fn!(
        doc = "Fully associative structure",
        is_fully_associative,
        edx,
        8
    );

    pub fn partitioning(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn ways(&self) -> u16 { panic!("STUB: not implemented") }

    pub fn sets(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn cache_type(&self) -> DatType { panic!("STUB: not implemented") }

    pub fn cache_level(&self) -> u8 { panic!("STUB: not implemented") }

    pub fn max_addressable_ids(&self) -> u16 { panic!("STUB: not implemented") }
}

impl Debug for DatInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

#[derive(Eq, PartialEq, Debug)]
pub enum DatType {
    
    Null = 0b00000,
    DataTLB = 0b00001,
    InstructionTLB = 0b00010,
    
    UnifiedTLB = 0b00011,
    LoadOnly = 0b0100,
    StoreOnly = 0b0101,
    Unknown,
}

impl fmt::Display for DatType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

pub struct SoCVendorInfo<R: CpuIdReader> {
    read: R,
    
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
}

impl<R: CpuIdReader> SoCVendorInfo<R> {
    pub fn get_soc_vendor_id(&self) -> u16 { panic!("STUB: not implemented") }

    pub fn get_project_id(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn get_stepping_id(&self) -> u32 { panic!("STUB: not implemented") }

    pub fn get_vendor_brand(&self) -> Option<SoCVendorBrand> { panic!("STUB: not implemented") }

    pub fn get_vendor_attributes(&self) -> Option<SoCVendorAttributesIter<R>> { panic!("STUB: not implemented") }
}

impl<R: CpuIdReader> fmt::Debug for SoCVendorInfo<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { panic!("STUB: not implemented") }
}

pub struct SoCVendorAttributesIter<R: CpuIdReader> {
    read: R,
    count: u32,
    current: u32,
}

impl<R: CpuIdReader> fmt::Debug for SoCVendorAttributesIter<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { panic!("STUB: not implemented") }
}

impl<R: CpuIdReader> Iterator for SoCVendorAttributesIter<R> {
    type Item = CpuIdResult;

    fn next(&mut self) -> Option<CpuIdResult> { panic!("STUB: not implemented") }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub struct SoCVendorBrand {
    data: [CpuIdResult; 3],
}

impl SoCVendorBrand {
    
    pub fn as_str(&self) -> &str { panic!("STUB: not implemented") }

    #[deprecated(
        since = "10.0.0",
        note = "Use idiomatic function name `as_str` instead"
    )]
    pub fn as_string(&self) -> &str { panic!("STUB: not implemented") }
}

impl fmt::Display for SoCVendorBrand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { panic!("STUB: not implemented") }
}

pub struct HypervisorInfo<R: CpuIdReader> {
    read: R,
    res: CpuIdResult,
}

impl<R: CpuIdReader> fmt::Debug for HypervisorInfo<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { panic!("STUB: not implemented") }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Hypervisor {
    Xen,
    VMware,
    HyperV,
    KVM,
    
    QEMU,
    Bhyve,
    QNX,
    ACRN,
    Unknown(u32, u32, u32),
}

impl<R: CpuIdReader> HypervisorInfo<R> {
    
    pub fn identify(&self) -> Hypervisor { panic!("STUB: not implemented") }

    pub fn tsc_frequency(&self) -> Option<u32> { panic!("STUB: not implemented") }

    pub fn apic_frequency(&self) -> Option<u32> { panic!("STUB: not implemented") }
}

#[cfg(doctest)]
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            
            extern "C" {}
        };
    }

    external_doc_test!(include_str!("../README.md"));
}
