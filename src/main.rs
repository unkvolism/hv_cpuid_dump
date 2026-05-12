mod cpuid;
mod flags;
mod decoders;
mod leaf;
use cpuid::{cpuid, hypervisor_present, hypervisor_vendor, max_hypervisor_leaf};
use leaf::{HvCpuidLeaf, dump_leaf};
use std::process::exit;

fn main() {
    let result = cpuid(0x40000000, 0);

    if hypervisor_present() {
        println!("[!] Hypervisor present!");

        println!("\n[!] Registered CPUID leaf 0x40000000");
        println!("EAX = 0x{:08X}", result.eax);
        println!("EBX = 0x{:08X}", result.ebx);
        println!("ECX = 0x{:08X}", result.ecx);
        println!("EDX = 0x{:08X}", result.edx);

        println!("\n[+] Hypervisor vendor: {}", hypervisor_vendor().expect("Hypervisor vendor must be present"));
        println!("[+] Max hypervisor leaf: 0x{:08X}", max_hypervisor_leaf());

        for leaf_value in 0x40000000..=max_hypervisor_leaf() {
            match HvCpuidLeaf::try_from(leaf_value) {
                Ok(leaf) => dump_leaf(leaf),
                Err(_) => {
                    let regs = cpuid(leaf_value, 0);
                    println!("\n=== CPUID 0x{:08X} (Unknown leaf) ===", leaf_value);
                    println!("Raw: EAX={:08X} EBX={:08X} ECX={:08X} EDX={:08X}",
                             regs.eax, regs.ebx, regs.ecx, regs.edx);
                }
            }
        }
    } else {
        println!("Hypervisor not present!");
        exit(1);
    }
}
