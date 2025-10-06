use crate::{cpu::registers::CpuRegisters, memory::TlbEntry};
use uluru::LRUCache;

/// CPU execution state
#[derive(Debug)]
pub struct CpuState {
    pub registers: CpuRegisters,
    pub running: bool,
    pub halted: bool,
    pub interrupt_pending: bool,
    pub interrupt_vector: u8,
    /// TLB for data access
    pub data_tlb: LRUCache<TlbEntry, 64>,
    /// TLB for instruction access
    pub instruction_tlb: LRUCache<TlbEntry, 128>,
    /// TLB for shared access
    pub shared_tlb: LRUCache<TlbEntry, 2048>,
}

impl CpuState {
    pub fn new() -> Self {
        Self {
            registers: CpuRegisters::new(),
            running: false,
            halted: false,
            interrupt_pending: false,
            interrupt_vector: 0,
            data_tlb: LRUCache::new(),
            instruction_tlb: LRUCache::new(),
            shared_tlb: LRUCache::new(),
        }
    }

    pub fn start(&mut self) {
        self.running = true;
        self.halted = false;
    }

    pub fn halt(&mut self) {
        self.running = false;
        self.halted = true;
    }

    pub fn trigger_interrupt(&mut self, vector: u8) {
        self.interrupt_pending = true;
        self.interrupt_vector = vector;
    }

    pub fn clear_interrupt(&mut self) {
        self.interrupt_pending = false;
        self.interrupt_vector = 0;
    }

    /// Set the privilege level on the lowest 2 bits of the cs register
    pub fn set_privilege_level(&mut self, level: u8) {
        self.registers.cs = (self.registers.cs & 0xFFFFFFFC) | (level as u16);
    }

    pub fn is_kernel_mode(&self) -> bool {
        self.registers.cs & 0x3 == 0
    }

    pub fn is_user_mode(&self) -> bool {
        self.registers.cs & 0x3 == 3
    }
}
