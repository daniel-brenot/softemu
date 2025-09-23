use crate::cpu::registers::{CpuRegisters, RFlags};
use crate::memory::GuestMemory;
use crate::Result;

/// CPU execution state
#[derive(Debug, Clone)]
pub struct CpuState {
    pub registers: CpuRegisters,
    pub memory: GuestMemory,
    pub running: bool,
    pub halted: bool,
    pub interrupt_pending: bool,
    pub interrupt_vector: u8,
    pub privilege_level: u8, // 0 = kernel, 3 = user
    pub long_mode: bool,
    pub paging_enabled: bool,
}

impl CpuState {
    pub fn new(memory: GuestMemory) -> Self {
        Self {
            registers: CpuRegisters::new(),
            memory,
            running: false,
            halted: false,
            interrupt_pending: false,
            interrupt_vector: 0,
            privilege_level: 0, // Start in kernel mode
            long_mode: true,    // x86_64 long mode
            paging_enabled: true,
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

    pub fn set_privilege_level(&mut self, level: u8) {
        self.privilege_level = level;
    }

    pub fn is_kernel_mode(&self) -> bool {
        self.privilege_level == 0
    }

    pub fn is_user_mode(&self) -> bool {
        self.privilege_level == 3
    }

    /// Read a byte from memory at the given address
    pub fn read_u8(&self, addr: u64) -> Result<u8> {
        self.memory.read_u8(addr)
    }

    /// Read a word (16-bit) from memory at the given address
    pub fn read_u16(&self, addr: u64) -> Result<u16> {
        self.memory.read_u16(addr)
    }

    /// Read a double word (32-bit) from memory at the given address
    pub fn read_u32(&self, addr: u64) -> Result<u32> {
        self.memory.read_u32(addr)
    }

    /// Read a quad word (64-bit) from memory at the given address
    pub fn read_u64(&self, addr: u64) -> Result<u64> {
        self.memory.read_u64(addr)
    }

    /// Write a byte to memory at the given address
    pub fn write_u8(&mut self, addr: u64, value: u8) -> Result<()> {
        self.memory.write_u8(addr, value)
    }

    /// Write a word (16-bit) to memory at the given address
    pub fn write_u16(&mut self, addr: u64, value: u16) -> Result<()> {
        self.memory.write_u16(addr, value)
    }

    /// Write a double word (32-bit) to memory at the given address
    pub fn write_u32(&mut self, addr: u64, value: u32) -> Result<()> {
        self.memory.write_u32(addr, value)
    }

    /// Write a quad word (64-bit) to memory at the given address
    pub fn write_u64(&mut self, addr: u64, value: u64) -> Result<()> {
        self.memory.write_u64(addr, value)
    }

    /// Virtual to physical address translation
    pub fn translate_address(&self, virt_addr: u64) -> Result<u64> {
        if !self.paging_enabled {
            return Ok(virt_addr);
        }

        // Simplified address translation for now
        // In a real implementation, this would walk the page tables
        // For now, we'll use identity mapping
        Ok(virt_addr)
    }

    /// Check if an address is valid for the current privilege level
    pub fn is_address_valid(&self, addr: u64, write: bool) -> bool {
        // Basic address validation
        if addr >= self.memory.size() {
            return false;
        }

        // Check privilege level restrictions
        if self.is_user_mode() {
            // User mode restrictions would go here
            // For now, allow all access
        }

        true
    }
}
