use vm_memory::{GuestAddress, GuestMemoryMmap, GuestMemoryRegion, GuestRegionMmap, Bytes, GuestMemory as VmGuestMemory};
use crate::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Guest memory management using vm-memory crate
#[derive(Debug, Clone)]
pub struct GuestMemory {
    memory: Arc<RwLock<GuestMemoryMmap>>,
    size: u64,
}

impl GuestMemory {
    /// Create a new guest memory instance
    pub fn new(size: u64) -> Result<Self> {
        // Create a simple memory mapping for now
        // In a real implementation, this would use proper memory regions
        let memory = GuestMemoryMmap::new();

        Ok(Self {
            memory: Arc::new(RwLock::new(memory)),
            size,
        })
    }

    /// Get the total size of guest memory
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Read a byte from guest memory
    pub fn read_u8(&self, addr: u64) -> Result<u8> {
        let memory = self.memory.blocking_read();
        let guest_addr = GuestAddress(addr);
        memory.read_obj(guest_addr).map_err(|e| crate::EmulatorError::Memory(e.to_string()))
    }

    /// Read a 16-bit value from guest memory
    pub fn read_u16(&self, addr: u64) -> Result<u16> {
        let memory = self.memory.blocking_read();
        let guest_addr = GuestAddress(addr);
        memory.read_obj(guest_addr).map_err(|e| crate::EmulatorError::Memory(e.to_string()))
    }

    /// Read a 32-bit value from guest memory
    pub fn read_u32(&self, addr: u64) -> Result<u32> {
        let memory = self.memory.blocking_read();
        let guest_addr = GuestAddress(addr);
        memory.read_obj(guest_addr).map_err(|e| crate::EmulatorError::Memory(e.to_string()))
    }

    /// Read a 64-bit value from guest memory
    pub fn read_u64(&self, addr: u64) -> Result<u64> {
        let memory = self.memory.blocking_read();
        let guest_addr = GuestAddress(addr);
        memory.read_obj(guest_addr).map_err(|e| crate::EmulatorError::Memory(e.to_string()))
    }

    /// Write a byte to guest memory
    pub fn write_u8(&mut self, addr: u64, value: u8) -> Result<()> {
        let mut memory = self.memory.blocking_write();
        let guest_addr = GuestAddress(addr);
        memory.write_obj(value, guest_addr).map_err(|e| crate::EmulatorError::Memory(e.to_string()))
    }

    /// Write a 16-bit value to guest memory
    pub fn write_u16(&mut self, addr: u64, value: u16) -> Result<()> {
        let mut memory = self.memory.blocking_write();
        let guest_addr = GuestAddress(addr);
        memory.write_obj(value, guest_addr).map_err(|e| crate::EmulatorError::Memory(e.to_string()))
    }

    /// Write a 32-bit value to guest memory
    pub fn write_u32(&mut self, addr: u64, value: u32) -> Result<()> {
        let mut memory = self.memory.blocking_write();
        let guest_addr = GuestAddress(addr);
        memory.write_obj(value, guest_addr).map_err(|e| crate::EmulatorError::Memory(e.to_string()))
    }

    /// Write a 64-bit value to guest memory
    pub fn write_u64(&mut self, addr: u64, value: u64) -> Result<()> {
        let mut memory = self.memory.blocking_write();
        let guest_addr = GuestAddress(addr);
        memory.write_obj(value, guest_addr).map_err(|e| crate::EmulatorError::Memory(e.to_string()))
    }

    /// Read a slice of bytes from guest memory
    pub fn read_slice(&self, addr: u64, len: usize) -> Result<Vec<u8>> {
        let memory = self.memory.blocking_read();
        let guest_addr = GuestAddress(addr);
        let mut buffer = vec![0u8; len];
        memory.read_slice(&mut buffer, guest_addr).map_err(|e| crate::EmulatorError::Memory(e.to_string()))?;
        Ok(buffer)
    }

    /// Write a slice of bytes to guest memory
    pub fn write_slice(&mut self, addr: u64, data: &[u8]) -> Result<()> {
        let mut memory = self.memory.blocking_write();
        let guest_addr = GuestAddress(addr);
        memory.write_slice(data, guest_addr).map_err(|e| crate::EmulatorError::Memory(e.to_string()))
    }

    /// Check if an address is valid
    pub fn is_valid_address(&self, addr: u64) -> bool {
        let memory = self.memory.blocking_read();
        let guest_addr = GuestAddress(addr);
        memory.address_in_range(guest_addr)
    }

    /// Get a reference to the underlying memory for device access
    pub fn get_memory_ref(&self) -> Arc<RwLock<GuestMemoryMmap>> {
        self.memory.clone()
    }

    /// Load data from a file into guest memory
    pub fn load_from_file(&mut self, addr: u64, file_path: &std::path::Path) -> Result<()> {
        let data = std::fs::read(file_path)?;
        self.write_slice(addr, &data)
    }

    /// Create a memory mapping for a device
    pub fn create_device_mapping(&mut self, start_addr: u64, size: u64) -> Result<()> {
        // In a real implementation, this would create a memory mapping
        // for MMIO devices. For now, we'll just validate the range.
        if start_addr + size > self.size {
            return Err(crate::EmulatorError::Memory("Device mapping exceeds guest memory".to_string()));
        }
        Ok(())
    }
}

