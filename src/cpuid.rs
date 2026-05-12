use core::arch::x86_64::__cpuid_count;
pub use core::arch::x86_64::CpuidResult;

pub fn cpuid(leaf: u32, sub_leaf: u32) -> CpuidResult {
    __cpuid_count(leaf, sub_leaf)
}

pub fn hypervisor_present() -> bool {
    cpuid(0x1, 0).ecx & (1 << 31) != 0
}

pub fn hypervisor_vendor() -> Option<String> {
    if !hypervisor_present() {
        None
    } else {
        let regs = cpuid(0x40000000, 0);
        Some(parse_vendor(&regs))
    }
}

pub fn max_hypervisor_leaf() -> u32 {
    cpuid(0x40000000, 0).eax
}

pub fn parse_vendor(regs: &CpuidResult) -> String {
    let mut vendor_bytes = [0u8; 12];
    vendor_bytes[0..4].copy_from_slice(&regs.ebx.to_le_bytes());
    vendor_bytes[4..8].copy_from_slice(&regs.ecx.to_le_bytes());
    vendor_bytes[8..12].copy_from_slice(&regs.edx.to_le_bytes());
    std::str::from_utf8(&vendor_bytes)
        .expect("Vendor must be valid UTF-8 per CPUID")
        .to_string()
}