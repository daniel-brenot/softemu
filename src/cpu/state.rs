use crate::{cpu::registers::CpuRegisters, memory::{MemoryManager, TlbEntry}};
use uluru::LRUCache;

/// CPU execution state
#[derive(Debug)]
pub struct CpuState {
    /// Core ID
    pub core_id: u32,
    /// CPU registers
    pub registers: CpuRegisters,
    /// TLB for data access
    pub data_tlb: LRUCache<TlbEntry, 64>,
    /// TLB for instruction access
    pub instruction_tlb: LRUCache<TlbEntry, 128>,
    /// TLB for shared access
    pub shared_tlb: LRUCache<TlbEntry, 2048>,
}

impl CpuState {
    pub fn new(core_id: u32) -> Self {
        Self {
            core_id,
            registers: CpuRegisters::new(),
            data_tlb: LRUCache::new(),
            instruction_tlb: LRUCache::new(),
            shared_tlb: LRUCache::new(),
        }
    }

    /// Handle an interrupt
    fn handle_interrupt(&mut self, memory: &MemoryManager) {
        // Save current state to stack before jumping to interrupt handler
        memory.write_u64(self.registers.rsp, self.registers.rip);
        self.registers.rsp -= 8;
        
        memory.write_u64(self.registers.rsp, self.registers.cs as u64);
        self.registers.rsp -= 8;
        
        memory.write_u64(self.registers.rsp, self.registers.rflags);
        self.registers.rsp -= 8;
        
        // Set interrupt flag
        self.registers.rflags |= 0x200;
        
        // Jump to interrupt handler
        // TODO: Implement interrupt handler
        self.registers.rip = 0x1000;
    }

    /// Get the core ID
    pub fn core_id(&self) -> u32 {
        self.core_id
    }
}
