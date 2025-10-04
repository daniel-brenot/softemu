use crate::memory::{GuestMemory, MmioManager};
use crate::Result;
use std::sync::{Arc, Mutex};

/// Memory manager that handles routing between guest memory and MMIO devices
/// with proper address space separation and offset subtraction
#[derive(Debug)]
pub struct MemoryManager {
    guest_memory: GuestMemory,
    mmio_manager: MmioManager,
    mmio_space_size: u64,
}

impl MemoryManager {
    /// Create a new memory manager
    pub fn new(guest_memory: GuestMemory, mmio_manager: MmioManager, mmio_space_size: u64) -> Self {
        Self {
            guest_memory,
            mmio_manager,
            mmio_space_size,
        }
    }

    /// Get the total address space size (guest memory + MMIO space)
    pub fn total_address_space_size(&self) -> u64 {
        self.guest_memory.size() + self.mmio_space_size
    }

    /// Get the MMIO space size
    pub fn mmio_space_size(&self) -> u64 {
        self.mmio_space_size
    }

    /// Get the guest memory size
    pub fn guest_memory_size(&self) -> u64 {
        self.guest_memory.size()
    }

    /// Check if an address is in MMIO space
    pub fn is_mmio_address(&self, addr: u64) -> bool {
        addr < self.mmio_space_size
    }

    /// Check if an address is in guest memory space
    pub fn is_guest_memory_address(&self, addr: u64) -> bool {
        addr >= self.mmio_space_size && addr < self.total_address_space_size()
    }

    /// Convert guest memory address to actual guest memory offset
    /// This subtracts the MMIO space size from the address
    fn guest_memory_offset(&self, addr: u64) -> u64 {
        addr - self.mmio_space_size
    }

    /// Read a byte from memory (routes to MMIO or guest memory)
    pub fn read_u8(&self, addr: u64) -> Result<u8> {
        if self.is_mmio_address(addr) {
            Ok(self.mmio_manager.read(addr, 1)? as u8)
        } else if self.is_guest_memory_address(addr) {
            let guest_addr = self.guest_memory_offset(addr);
            self.guest_memory.read_u8(guest_addr)
        } else {
            Err(crate::EmulatorError::Memory(format!(
                "Address 0x{:x} is outside valid address space (0x{:x})", 
                addr, self.total_address_space_size()
            )))
        }
    }

    /// Read a word (16-bit) from memory
    pub fn read_u16(&self, addr: u64) -> Result<u16> {
        if self.is_mmio_address(addr) {
            Ok(self.mmio_manager.read(addr, 2)? as u16)
        } else if self.is_guest_memory_address(addr) {
            let guest_addr = self.guest_memory_offset(addr);
            self.guest_memory.read_u16(guest_addr)
        } else {
            Err(crate::EmulatorError::Memory(format!(
                "Address 0x{:x} is outside valid address space (0x{:x})", 
                addr, self.total_address_space_size()
            )))
        }
    }

    /// Read a double word (32-bit) from memory
    pub fn read_u32(&self, addr: u64) -> Result<u32> {
        if self.is_mmio_address(addr) {
            Ok(self.mmio_manager.read(addr, 4)? as u32)
        } else if self.is_guest_memory_address(addr) {
            let guest_addr = self.guest_memory_offset(addr);
            self.guest_memory.read_u32(guest_addr)
        } else {
            Err(crate::EmulatorError::Memory(format!(
                "Address 0x{:x} is outside valid address space (0x{:x})", 
                addr, self.total_address_space_size()
            )))
        }
    }

    /// Read a quad word (64-bit) from memory
    pub fn read_u64(&self, addr: u64) -> Result<u64> {
        if self.is_mmio_address(addr) {
            self.mmio_manager.read(addr, 8)
        } else if self.is_guest_memory_address(addr) {
            let guest_addr = self.guest_memory_offset(addr);
            self.guest_memory.read_u64(guest_addr)
        } else {
            Err(crate::EmulatorError::Memory(format!(
                "Address 0x{:x} is outside valid address space (0x{:x})", 
                addr, self.total_address_space_size()
            )))
        }
    }

    /// Write a byte to memory
    pub fn write_u8(&mut self, addr: u64, value: u8) -> Result<()> {
        if self.is_mmio_address(addr) {
            self.mmio_manager.write(addr, value as u64, 1)
        } else if self.is_guest_memory_address(addr) {
            let guest_addr = self.guest_memory_offset(addr);
            self.guest_memory.write_u8(guest_addr, value)
        } else {
            Err(crate::EmulatorError::Memory(format!(
                "Address 0x{:x} is outside valid address space (0x{:x})", 
                addr, self.total_address_space_size()
            )))
        }
    }

    /// Write a word (16-bit) to memory
    pub fn write_u16(&mut self, addr: u64, value: u16) -> Result<()> {
        if self.is_mmio_address(addr) {
            self.mmio_manager.write(addr, value as u64, 2)
        } else if self.is_guest_memory_address(addr) {
            let guest_addr = self.guest_memory_offset(addr);
            self.guest_memory.write_u16(guest_addr, value)
        } else {
            Err(crate::EmulatorError::Memory(format!(
                "Address 0x{:x} is outside valid address space (0x{:x})", 
                addr, self.total_address_space_size()
            )))
        }
    }

    /// Write a double word (32-bit) to memory
    pub fn write_u32(&mut self, addr: u64, value: u32) -> Result<()> {
        if self.is_mmio_address(addr) {
            self.mmio_manager.write(addr, value as u64, 4)
        } else if self.is_guest_memory_address(addr) {
            let guest_addr = self.guest_memory_offset(addr);
            self.guest_memory.write_u32(guest_addr, value)
        } else {
            Err(crate::EmulatorError::Memory(format!(
                "Address 0x{:x} is outside valid address space (0x{:x})", 
                addr, self.total_address_space_size()
            )))
        }
    }

    /// Write a quad word (64-bit) to memory
    pub fn write_u64(&mut self, addr: u64, value: u64) -> Result<()> {
        if self.is_mmio_address(addr) {
            self.mmio_manager.write(addr, value, 8)
        } else if self.is_guest_memory_address(addr) {
            let guest_addr = self.guest_memory_offset(addr);
            self.guest_memory.write_u64(guest_addr, value)
        } else {
            Err(crate::EmulatorError::Memory(format!(
                "Address 0x{:x} is outside valid address space (0x{:x})", 
                addr, self.total_address_space_size()
            )))
        }
    }

    /// Read a slice of bytes from memory
    pub fn read_slice(&self, addr: u64, len: usize) -> Result<Vec<u8>> {
        if self.is_mmio_address(addr) {
            // For MMIO, we need to read byte by byte since devices might not support bulk reads
            let mut result = Vec::with_capacity(len);
            for i in 0..len {
                result.push(self.read_u8(addr + i as u64)?);
            }
            Ok(result)
        } else if self.is_guest_memory_address(addr) {
            let guest_addr = self.guest_memory_offset(addr);
            self.guest_memory.read_slice(guest_addr, len)
        } else {
            Err(crate::EmulatorError::Memory(format!(
                "Address 0x{:x} is outside valid address space (0x{:x})", 
                addr, self.total_address_space_size()
            )))
        }
    }

    /// Write a slice of bytes to memory
    pub fn write_slice(&mut self, addr: u64, data: &[u8]) -> Result<()> {
        if self.is_mmio_address(addr) {
            // For MMIO, we need to write byte by byte since devices might not support bulk writes
            for (i, &byte) in data.iter().enumerate() {
                self.write_u8(addr + i as u64, byte)?;
            }
            Ok(())
        } else if self.is_guest_memory_address(addr) {
            let guest_addr = self.guest_memory_offset(addr);
            self.guest_memory.write_slice(guest_addr, data)
        } else {
            Err(crate::EmulatorError::Memory(format!(
                "Address 0x{:x} is outside valid address space (0x{:x})", 
                addr, self.total_address_space_size()
            )))
        }
    }

    /// Check if an address is valid for the current privilege level
    pub fn is_address_valid(&self, addr: u64, _write: bool) -> bool {
        addr < self.total_address_space_size()
    }

    /// Get a reference to the guest memory (for direct access when needed)
    pub fn get_guest_memory(&self) -> &GuestMemory {
        &self.guest_memory
    }

    /// Get a mutable reference to the guest memory
    pub fn get_guest_memory_mut(&mut self) -> &mut GuestMemory {
        &mut self.guest_memory
    }

    /// Virtual to physical address translation
    pub fn translate_address(&self, virt_addr: u64) -> Result<u64> {
        // For now, we use identity mapping
        // In a real implementation, this would walk the page tables
        Ok(virt_addr)
    }
}
