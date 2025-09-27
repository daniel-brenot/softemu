use bitflags::bitflags;

/// x86_64 CPU registers
#[derive(Debug, Clone, Default)]
pub struct CpuRegisters {
    // General purpose registers (64-bit)
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64, // Instruction pointer

    // Segment registers
    pub cs: u16,
    pub ds: u16,
    pub es: u16,
    pub fs: u16,
    pub gs: u16,
    pub ss: u16,

    // Control registers
    pub cr0: u64,
    pub cr2: u64,
    pub cr3: u64,
    pub cr4: u64,
    pub cr8: u64,

    // Debug registers
    pub dr0: u64,
    pub dr1: u64,
    pub dr2: u64,
    pub dr3: u64,
    pub dr6: u64,
    pub dr7: u64,

    // Model-specific registers
    pub msr_efer: u64,
    pub msr_star: u64,
    pub msr_lstar: u64,
    pub msr_cstar: u64,
    pub msr_sfmask: u64,
    pub msr_fs_base: u64,
    pub msr_gs_base: u64,
    pub msr_kernel_gs_base: u64,

    // Flags register
    pub rflags: u64,

    // K-mask registers (AVX-512 mask registers)
    pub k0: u64,
    pub k1: u64,
    pub k2: u64,
    pub k3: u64,
    pub k4: u64,
    pub k5: u64,
    pub k6: u64,
    pub k7: u64,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct RFlags: u64 {
        const CARRY = 1 << 0;
        const PARITY = 1 << 2;
        const AUXILIARY = 1 << 4;
        const ZERO = 1 << 6;
        const SIGN = 1 << 7;
        const TRAP = 1 << 8;
        const INTERRUPT = 1 << 9;
        const DIRECTION = 1 << 10;
        const OVERFLOW = 1 << 11;
        const IOPL_LOW = 1 << 12;
        const IOPL_HIGH = 1 << 13;
        const NESTED_TASK = 1 << 14;
        const RESUME = 1 << 16;
        const VIRTUAL_8086 = 1 << 17;
        const ALIGNMENT_CHECK = 1 << 18;
        const VIRTUAL_INTERRUPT = 1 << 19;
        const VIRTUAL_INTERRUPT_PENDING = 1 << 20;
        const ID = 1 << 21;
    }
}

impl CpuRegisters {
    pub fn new() -> Self {
        Self {
            // Initialize with default values
            cs: 0x08, // Kernel code segment
            ds: 0x10, // Kernel data segment
            es: 0x10,
            fs: 0x10,
            gs: 0x10,
            ss: 0x10,
            rflags: RFlags::INTERRUPT.bits(),
            ..Default::default()
        }
    }

    pub fn get_gp_register(&self, index: u8) -> u64 {
        match index {
            0 => self.rax,
            1 => self.rcx,
            2 => self.rdx,
            3 => self.rbx,
            4 => self.rsp,
            5 => self.rbp,
            6 => self.rsi,
            7 => self.rdi,
            8 => self.r8,
            9 => self.r9,
            10 => self.r10,
            11 => self.r11,
            12 => self.r12,
            13 => self.r13,
            14 => self.r14,
            15 => self.r15,
            _ => 0,
        }
    }

    pub fn set_gp_register(&mut self, index: u8, value: u64) {
        match index {
            0 => self.rax = value,
            1 => self.rcx = value,
            2 => self.rdx = value,
            3 => self.rbx = value,
            4 => self.rsp = value,
            5 => self.rbp = value,
            6 => self.rsi = value,
            7 => self.rdi = value,
            8 => self.r8 = value,
            9 => self.r9 = value,
            10 => self.r10 = value,
            11 => self.r11 = value,
            12 => self.r12 = value,
            13 => self.r13 = value,
            14 => self.r14 = value,
            15 => self.r15 = value,
            _ => {}
        }
    }

    pub fn get_flags(&self) -> RFlags {
        RFlags::from_bits_truncate(self.rflags)
    }

    pub fn set_flags(&mut self, flags: RFlags) {
        self.rflags = flags.bits();
    }

    pub fn set_flag(&mut self, flag: RFlags, value: bool) {
        if value {
            self.rflags |= flag.bits();
        } else {
            self.rflags &= !flag.bits();
        }
    }

    pub fn get_flag(&self, flag: RFlags) -> bool {
        self.rflags & flag.bits() != 0
    }

    /// Get K-mask register value
    pub fn get_k_register(&self, index: u8) -> u64 {
        match index {
            0 => self.k0,
            1 => self.k1,
            2 => self.k2,
            3 => self.k3,
            4 => self.k4,
            5 => self.k5,
            6 => self.k6,
            7 => self.k7,
            _ => 0,
        }
    }

    /// Set K-mask register value
    pub fn set_k_register(&mut self, index: u8, value: u64) {
        match index {
            0 => self.k0 = value,
            1 => self.k1 = value,
            2 => self.k2 = value,
            3 => self.k3 = value,
            4 => self.k4 = value,
            5 => self.k5 = value,
            6 => self.k6 = value,
            7 => self.k7 = value,
            _ => {}
        }
    }
}
