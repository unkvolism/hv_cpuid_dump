use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct PartitionPrivilegeFlagsLow: u32 {
        const ACCESS_VP_RUN_TIME_REG = 1 << 0;
        const ACCESS_PARTITION_REFERENCE_COUNTER = 1 << 1;
        const ACCESS_SYNIC_REGS = 1 << 2;
        const ACCESS_SYNTHETIC_TIMER_REGS = 1 << 3;
        const ACCESS_INTR_CTRL_REGS = 1 << 4;
        const ACCESS_HYPERCALL_MSRS = 1 << 5;
        const ACCESS_VP_INDEX = 1 << 6;
        const ACCESS_RESET_REG = 1 << 7;
        const ACCESS_STATS_REG = 1 << 8;
        const ACCESS_PARTITION_REFERENCE_TSC = 1 << 9;
        const ACCESS_GUEST_IDLE_REG = 1 << 10;
        const ACCESS_FREQUENCY_REGS = 1 << 11;
        const ACCESS_DEBUG_REGS = 1 << 12;
        const ACCESS_REENLIGHTENMENT_CONTROLS = 1 << 13;
        const ACCESS_ROOT_SCHEDULER_REG = 1 << 14;
        const ACCESS_TSC_INVARIANT_CONTROLS = 1 << 15;

    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct PartitionPrivilegeFlagsHigh: u32 {
        const CREATE_PARTITIONS = 1 << 0;
        const ACCESS_PARTITION_ID = 1 << 1;
        const ACCESS_MEMORY_POOL = 1 << 2;
        const POST_MESSAGES = 1 << 4;
        const SIGNAL_EVENTS = 1 << 5;
        const CREATE_PORT = 1 << 6;
        const CONNECT_PORT = 1 << 7;
        const ACCESS_STATS = 1 << 8;
        const DEBUGGING = 1 << 11;
        const CPU_MANAGEMENT = 1 << 12;
        const ACCESS_VSM = 1 << 16;
        const ACCESS_VP_REGISTERS = 1 << 17;
        const ENABLE_EXTENDED_HYPERCALLS = 1 << 20;
        const START_VIRTUAL_PROCESSOR = 1 << 21;
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct ImplementationRecommendationsEdx: u32 {
        const DEPRECATED_USE_HYPERCALL_FOR_REMOTE_FLUSH_AND_LOCAL = 1 << 0;
        const USE_HYPERCALL_FOR_ADDRESS_SPACE_SWITCH = 1 << 1;
        const USE_HYPERCALL_FOR_LOCAL_FLUSH = 1 << 2;
        const USE_HYPERCALL_FOR_REMOTE_FLUSH_AND_LOCAL_FLUSH_ENTIRE = 1 << 3;
        const USE_APIC_MSRS = 1 << 4;
        const USE_MSR_FOR_RESET = 1 << 5;
        const RELAXED_TIMING = 1 << 6;
        const USE_DMA_REMAPPING = 1 << 7;
        const USE_INTERRUPT_REMAPPING = 1 << 8;
        const USE_X2APIC_MSRS = 1 << 9;
        const DEPRECATE_AUTO_EOI = 1 << 10;
        const USE_SYNTHETIC_CLUSTER_IPI = 1 << 11;
        const USE_EX_PROCESSOR_MASKS = 1 << 12;
        const NESTED = 1 << 13;
        const USE_INT_FOR_MBEC_SYSTEM_CALLS = 1 << 14;
        const USE_VMCS_ENLIGHTENMENTS = 1 << 15;
        const USE_SYNCED_TIMELINE = 1 << 16;
        const USE_DIRECT_LOCAL_FLUSH_ENTIRE = 1 << 17;
        const NO_NON_ARCHITECTURAL_CORE_SHARING = 1 << 18;
        const USE_X2APIC = 1 << 19;
        const RESTORE_TIME_ON_RESUME = 1 << 20;
        const USE_HYPERCALL_FOR_MMIO_ACCESS = 1 << 21;
        const USE_GPA_PINNING_HYPERCALL = 1 << 22;
        const WAKE_VPS = 1 << 23;
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct HardwareFeaturesEax: u32 {
        const APIC_OVERLAY_ASSIST = 1 << 0;
        const MSR_BITMAPS = 1 << 1;
        const ARCHITECTURAL_PERF_COUNTERS = 1 << 2;
        const SLAT = 1 << 3;
        const DMA_REMAPPING = 1 << 4;
        const INTERRUPT_REMAPPING = 1 << 5;
        const MEMORY_PATROL_SCRUBBER = 1 << 6;
        const DMA_PROTECTION = 1 << 7;
        const HPET_REQUESTED = 1 << 8;
        const SYNTHETIC_TIMERS_VOLATILE = 1 << 9;
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct NestedHypervisorEnlightenmentsEax: u32 {
        const DIRECT_VIRTUAL_FLUSH = 1 << 0;
        const FLUSH_GUEST_PHYSICAL_ADDRESS_HYPERCALL = 1 << 1;
        const ENLIGHTENED_MSR_BITMAP = 1 << 2;
        const DIRECT_ACCESS_TO_HYPERCALL_MSR = 1 << 4;
        const DIRECT_ACCESS_TO_SYNIC_MSRS = 1 << 5;
        const DIRECT_ACCESS_TO_VP_ASSIST_PAGE_MSRS = 1 << 6;
    }
}