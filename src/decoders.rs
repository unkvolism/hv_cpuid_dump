use crate::cpuid::{CpuidResult, parse_vendor};
use crate::flags::{
    PartitionPrivilegeFlagsLow,
    PartitionPrivilegeFlagsHigh,
    ImplementationRecommendationsEdx,
    HardwareFeaturesEax,
    NestedHypervisorEnlightenmentsEax,
};

pub fn decode_partition_privileges(regs: &CpuidResult) {
    let CpuidResult { eax, ebx, edx, .. } = *regs;

    let low = PartitionPrivilegeFlagsLow::from_bits_truncate(regs.eax);
    let high = PartitionPrivilegeFlagsHigh::from_bits_truncate(regs.ebx);
    let impl_recommendations = ImplementationRecommendationsEdx::from_bits_truncate(regs.edx);

    println!("\n[#] Privilege Flags Low (EAX, raw: 0x{:08X}):", eax);
    for flag in low.iter() {
        println!("  [+] {:?}", flag);
    }

    println!("\n[#] Privilege Flags High (EBX, raw: 0x{:08X}):", ebx);
    for flag in high.iter() {
        println!("  [+] {:?}", flag);
    }

    println!("\n[#] Implementation Recommendations (EDX, raw: 0x{:08X}):", edx);
    for flag in impl_recommendations.iter() {
        println!("  [+] {:?}", flag);
    }
}

pub fn decode_system_identity(regs: &CpuidResult) {
    let build = regs.eax & 0xFFFFFF;
    let service_branch = regs.eax >> 24;
    let major = (regs.ebx >> 16) & 0xFFFF;
    let minor = regs.ebx & 0xFFFF;
    let service_pack = regs.ecx;
    let service_number = regs.edx & 0xFFFFFF;

    println!("\n[#] System Identity:");
    println!("  Build (EAX[0:23]): {}", build);
    println!("  Service Branch (EAX[24:31]): {}", service_branch);
    println!("  Major (EBX[16:31]): {}", major);
    println!("  Minor (EBX[0:15]): {}", minor);
    println!("  Service Pack (ECX): {}", service_pack);
    println!("  Service Number (EDX[0:23]): {}", service_number);
}

pub fn decode_implementation_limits(regs: &CpuidResult) {
    let max_vps = regs.eax;
    let max_logical_processors = regs.ebx;
    let max_physical_interrupts = regs.ecx;

    println!("\n[#] Implementation Limits:");
    println!("  Max VPs per partition (EAX): {}", max_vps);
    println!("  Max logical Processors (EBX): {}", max_logical_processors);
    println!("  Max physical interrupt vectors (ECX): {}", max_physical_interrupts);
    println!("  Reserved (EDX): 0x{:08X}", regs.edx);
}

pub fn decode_hypervisor_interface(regs: &CpuidResult) {

    let bytes = regs.eax.to_le_bytes();
    let signature = std::str::from_utf8(&bytes).expect("Interface name must be valid UTF-8 per CPUID");

    let note = match signature {
        "Hv#1" => "Microsoft Hyper-V TLFS",
        "\0\0\0\0" => "No Interface (Non-conformant hypervisor)",
        _ => "Unknown Interface",
    };
    println!("\n[#] Hypervisor Interface:");
    println!("  Signature (EAX): \"{}\" ({})", signature, note);
    println!("  EBX (reserved): 0x{:08X}", regs.ebx);
    println!("  ECX (reserved): 0x{:08X}", regs.ecx);
    println!("  EDX (reserved): 0x{:08X}", regs.edx);
}

pub fn decode_hardware_features(regs: &CpuidResult) {
    let features = HardwareFeaturesEax::from_bits_truncate(regs.eax);
    let know_bits = features.bits();
    let unknown_bits = regs.eax & !know_bits;

    println!("\n[#] Hardware Features (EAX, raw: 0x{:08X}):", regs.eax);
    for flag in features.iter() {
        println!("  [+] {:?}", flag);
    }

    if unknown_bits != 0 {
        println!("\n[?] Unknown bits set: 0x{:08X}", unknown_bits);
    }

    println!("  EBX (undocumented): 0x{:08X}", regs.ebx);
    println!("  ECX (reserved): 0x{:08X}", regs.ecx);
    println!("  EDX (reserved): 0x{:08X}", regs.edx);
}

pub fn decode_nested_hypervisor_enlightenments(regs: &CpuidResult) {
    let features = NestedHypervisorEnlightenmentsEax::from_bits_truncate(regs.eax);
    let know_bits = features.bits();
    let unknown_bits = regs.eax & !know_bits;
    println!("\n[#] Nested Hypervisor Enlightenments (EAX, raw: 0x{:08X}):", regs.eax);

    if unknown_bits != 0 {
        println!("\n[?] Unknown bits set: 0x{:08X}", unknown_bits);
    }

    if features.is_empty() {
        println!("  (no nested enlightenments exposed)");
    } else {
        for flag in features.iter() {
            println!("  [+] {:?}", flag);
        }
    }
    println!("  EBX (reserved): 0x{:08X}", regs.ebx);
    println!("  ECX (reserved): 0x{:08X}", regs.ecx);
    println!("  EDX (reserved): 0x{:08X}", regs.edx);
}

pub fn decode_vendor_and_max_leaf(regs: &CpuidResult) {
    println!("\n[#] Vendor and Max Leaf:");
    println!("  Max hypervisor leaf (EAX): 0x{:08X}", regs.eax);
    println!("  Vendor (EBX:ECX:EDX): \"{}\"", parse_vendor(regs));
}