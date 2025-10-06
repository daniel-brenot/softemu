use crate::{cpu::registers::CpuRegisters, memory::TlbEntry};
use uluru::LRUCache;

/// CPU execution state
#[derive(Debug)]
pub struct CpuState {
    pub registers: CpuRegisters,
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
            interrupt_pending: false,
            interrupt_vector: 0,
            data_tlb: LRUCache::new(),
            instruction_tlb: LRUCache::new(),
            shared_tlb: LRUCache::new(),
        }
    }

    pub fn trigger_interrupt(&mut self, vector: u8) {
        self.interrupt_pending = true;
        self.interrupt_vector = vector;
    }

    pub fn clear_interrupt(&mut self) {
        self.interrupt_pending = false;
        self.interrupt_vector = 0;
    }
}
