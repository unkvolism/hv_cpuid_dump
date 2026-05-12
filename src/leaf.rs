use crate::cpuid::{cpuid};
use crate::decoders::{
    decode_partition_privileges,
    decode_system_identity,
    decode_implementation_limits,
    decode_hypervisor_interface,
    decode_hardware_features,
    decode_nested_hypervisor_enlightenments,
    decode_vendor_and_max_leaf,
};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HvCpuidLeaf {
    VendorAndMaxLeaf = 0x40000000,
    HypervisorInterface = 0x40000001,
    SystemIdentity = 0x40000002,
    PartitionPrivilegeFlags = 0x40000003,
    ImplementationLimits = 0x40000005,
    HardwareFeatures = 0x40000006,
    NestedHypervisorEnlightenments = 0x4000000A,
}

impl TryFrom<u32> for HvCpuidLeaf {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x40000000 => Ok(HvCpuidLeaf::VendorAndMaxLeaf),
            0x40000001 => Ok(HvCpuidLeaf::HypervisorInterface),
            0x40000002 => Ok(HvCpuidLeaf::SystemIdentity),
            0x40000003 => Ok(HvCpuidLeaf::PartitionPrivilegeFlags),
            0x40000005 => Ok(HvCpuidLeaf::ImplementationLimits),
            0x40000006 => Ok(HvCpuidLeaf::HardwareFeatures),
            0x4000000A => Ok(HvCpuidLeaf::NestedHypervisorEnlightenments),
            _ => Err(value)
        }
    }
}

pub fn dump_leaf(leaf: HvCpuidLeaf) {
    let raw_value = leaf as u32;
    let regs = cpuid(raw_value, 0);

    println!("\n=== CPUID 0x{:08X} - {:?} ===", raw_value, leaf);
    println!("Raw: EAX={:08X} EBX={:08X} ECX={:08X} EDX={:08X}",
             regs.eax, regs.ebx, regs.ecx, regs.edx);

    match leaf {
        HvCpuidLeaf::PartitionPrivilegeFlags => decode_partition_privileges(&regs),
        HvCpuidLeaf::SystemIdentity => decode_system_identity(&regs),
        HvCpuidLeaf::ImplementationLimits => decode_implementation_limits(&regs),
        HvCpuidLeaf::HypervisorInterface => decode_hypervisor_interface(&regs),
        HvCpuidLeaf::HardwareFeatures => decode_hardware_features(&regs),
        HvCpuidLeaf::NestedHypervisorEnlightenments => decode_nested_hypervisor_enlightenments(&regs),
        HvCpuidLeaf::VendorAndMaxLeaf => decode_vendor_and_max_leaf(&regs),
    }
}