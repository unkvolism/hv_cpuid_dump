# hv_cpuid_dump

Dumper for Hyper-V synthetic CPUID leaves (`0x40000000`–`0x4000000C`), written in Rust as an exercise in idiomatic CPUID parsing using the `bitflags` crate.

Decodes the documented fields from the Hyper-V TLFS and flags undocumented bits when present, which makes it useful for spotting features that haven't been catalogued publicly yet.

## What it does

For each synthetic CPUID leaf reported by the hypervisor:

- Reads `EAX`, `EBX`, `ECX`, `EDX` via the `__cpuid_count` intrinsic
- Prints the raw values
- Decodes the documented fields:
  - **`0x40000000`** - vendor string (`EBX:ECX:EDX`) and max leaf
  - **`0x40000001`** - interface signature (`Hv#1` for Microsoft Hyper-V TLFS)
  - **`0x40000002`** - build number, version, service pack/branch/number
  - **`0x40000003`** - partition privilege flags (low/high) and implementation recommendations
  - **`0x40000005`** - max VPs, max logical processors, max physical interrupt vectors
  - **`0x40000006`** - hardware features (APIC overlay, MSR bitmaps, SLAT, etc.)
  - **`0x4000000A`** - nested hypervisor enlightenments
- For leaves with `bitflags` decoders, separately reports any **unknown bits** set bits present in the value but not declared in the bitflags table. Useful for noticing newly-introduced flags

Leaves the tool doesn't have a decoder for (`0x40000004`, `0x40000007`, `0x40000008`, `0x40000009`, `0x4000000B`, `0x4000000C`) are still shown with their raw register values so you can eyeball them.

## Building and running

Requires Rust (stable) and a Windows host where you're running as a Hyper-V root partition (regular Windows desktop with Hyper-V/VBS enabled qualifies). Should also work in a Hyper-V guest, but output will reflect the guest's privileges, not the host's.

```
cargo run
```

No CLI args.

## Sample output

Tested on Windows 11 Pro 25H2 (OS Build 26200.8246), Intel x86_64 host:

```
[+] Hypervisor vendor: Microsoft Hv
[+] Max hypervisor leaf: 0x4000000C

=== CPUID 0x40000006 - HardwareFeatures ===
Raw: EAX=09F200AF EBX=00000027 ECX=00000000 EDX=00000000

[#] Hardware Features (EAX, raw: 0x09F200AF):
  [+] HardwareFeaturesEax(APIC_OVERLAY_ASSIST)
  [+] HardwareFeaturesEax(MSR_BITMAPS)
  [+] HardwareFeaturesEax(ARCHITECTURAL_PERF_COUNTERS)
  [+] HardwareFeaturesEax(SLAT)
  [+] HardwareFeaturesEax(INTERRUPT_REMAPPING)
  [+] HardwareFeaturesEax(DMA_PROTECTION)

[?] Unknown bits set: 0x09F20000
  EBX (undocumented): 0x00000027
  ECX (reserved): 0x00000000
  EDX (reserved): 0x00000000
```

## Observations on test host

Things worth noting from running on Windows 11 25H2 (Build 26200.8246, kernel reports 26100):

### Leaf `0x40000006` undocumented EAX bits and populated EBX

The public TLFS documents bits 0–9 of `EAX` for this leaf and declares `EBX` as reserved. Observation says otherwise:

- **`EAX` bits 17, 21, 25, 27** are set (`0x09F20000`). Not in the public spec
- **`EBX = 0x00000027`** (bits 0, 1, 2, 5). Documented as reserved, clearly isn't

Semantics for both unknown. Candidate features that could plausibly map here include MBEC, kCFG hardware acceleration, or other VBS-related additions in recent builds. Confirming would require reverse engineering `hvix64.exe`.

### Leaves not in the public TLFS

The hypervisor reports `max_leaf = 0x4000000C` but the public TLFS only documents up to `0x4000000A`. Two leaves past the documented range:

- **`0x4000000B`** and **`0x4000000C`**: both return all zeros. Could be placeholders or conditionally populated (e.g., only inside an isolated VM or after specific hypercalls)
- **`0x40000004`** (`EAX=00960E14`, `ECX=0x2E`) believed to be "Implementation Recommendations (legacy)" from older reverse engineering material
- **`0x40000007`** (`EAX=80000007`) likely CPU Management Features for root partition
- **`0x40000008`** and **`0x40000009`** zero on this host

## Acknowledgments

Written as a learning exercise for `bitflags` idioms in Rust, building on prior research from the Hyper-V community (Saar Amar's reverse engineering work, Quarkslab's VTL writeups, Microsoft's own TLFS).
