use crate::Result;
use std::sync::{Arc, RwLock};

/// Guest memory management - simplified implementation
#[derive(Debug, Clone)]
pub struct GuestMemory {
    memory: Arc<RwLock<Vec<u8>>>,
    size: u64,
}

impl GuestMemory {
    /// Create a new guest memory instance
    pub fn new(size: u64) -> Result<Self> {
        // Create a simple memory mapping using a Vec as backing storage
        let memory = vec![0u8; size as usize];

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
        let memory = self.memory.read().unwrap();
        if addr >= self.size {
            log::warn!("Memory read_u8 out of bounds: addr=0x{:x}, size=0x{:x}, returning 0", addr, self.size);
            return Ok(0); // Return 0 for invalid memory accesses instead of crashing
        }
        Ok(memory[addr as usize])
    }

    /// Read a 16-bit value from guest memory
    pub fn read_u16(&self, addr: u64) -> Result<u16> {
        let memory = self.memory.read().unwrap();
        if addr + 1 >= self.size {
            log::warn!("Memory read_u16 out of bounds: addr=0x{:x}, size=0x{:x}, returning 0", addr, self.size);
            return Ok(0); // Return 0 for invalid memory accesses instead of crashing
        }
        let bytes = &memory[addr as usize..addr as usize + 2];
        Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    /// Read a 32-bit value from guest memory
    pub fn read_u32(&self, addr: u64) -> Result<u32> {
        let memory = self.memory.read().unwrap();
        if addr + 3 >= self.size {
            log::warn!("Memory read_u32 out of bounds: addr=0x{:x}, size=0x{:x}, returning 0", addr, self.size);
            return Ok(0); // Return 0 for invalid memory accesses instead of crashing
        }
        let bytes = &memory[addr as usize..addr as usize + 4];
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    /// Read a 64-bit value from guest memory
    pub fn read_u64(&self, addr: u64) -> Result<u64> {
        let memory = self.memory.read().unwrap();
        if addr + 7 >= self.size {
            log::warn!("Memory read_u64 out of bounds: addr=0x{:x}, size=0x{:x}, returning 0", addr, self.size);
            return Ok(0); // Return 0 for invalid memory accesses instead of crashing
        }
        let bytes = &memory[addr as usize..addr as usize + 8];
        Ok(u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7]
        ]))
    }

    /// Write a byte to guest memory
    pub fn write_u8(&mut self, addr: u64, value: u8) -> Result<()> {
        let mut memory = self.memory.write().unwrap();
        if addr >= self.size {
            log::warn!("Memory write_u8 out of bounds: addr=0x{:x}, size=0x{:x}, ignoring write", addr, self.size);
            return Ok(()); // Ignore invalid memory writes instead of crashing
        }
        memory[addr as usize] = value;
        Ok(())
    }

    /// Write a 16-bit value to guest memory
    pub fn write_u16(&mut self, addr: u64, value: u16) -> Result<()> {
        let mut memory = self.memory.write().unwrap();
        if addr + 1 >= self.size {
            log::warn!("Memory write_u16 out of bounds: addr=0x{:x}, size=0x{:x}, ignoring write", addr, self.size);
            return Ok(()); // Ignore invalid memory writes instead of crashing
        }
        let bytes = value.to_le_bytes();
        memory[addr as usize..addr as usize + 2].copy_from_slice(&bytes);
        Ok(())
    }

    /// Write a 32-bit value to guest memory
    pub fn write_u32(&mut self, addr: u64, value: u32) -> Result<()> {
        let mut memory = self.memory.write().unwrap();
        if addr + 3 >= self.size {
            log::warn!("Memory write_u32 out of bounds: addr=0x{:x}, size=0x{:x}, ignoring write", addr, self.size);
            return Ok(()); // Ignore invalid memory writes instead of crashing
        }
        let bytes = value.to_le_bytes();
        memory[addr as usize..addr as usize + 4].copy_from_slice(&bytes);
        Ok(())
    }

    /// Write a 64-bit value to guest memory
    pub fn write_u64(&mut self, addr: u64, value: u64) -> Result<()> {
        let mut memory = self.memory.write().unwrap();
        if addr + 7 >= self.size {
            log::warn!("Memory write_u64 out of bounds: addr=0x{:x}, size=0x{:x}, ignoring write", addr, self.size);
            return Ok(()); // Ignore invalid memory writes instead of crashing
        }
        let bytes = value.to_le_bytes();
        memory[addr as usize..addr as usize + 8].copy_from_slice(&bytes);
        Ok(())
    }

    /// Read a slice of bytes from guest memory
    pub fn read_slice(&self, addr: u64, len: usize) -> Result<Vec<u8>> {
        let memory = self.memory.read().unwrap();
        if addr + len as u64 > self.size {
            return Err(crate::EmulatorError::Memory("Address out of bounds".to_string()));
        }
        Ok(memory[addr as usize..addr as usize + len].to_vec())
    }

    /// Write a slice of bytes to guest memory
    pub fn write_slice(&mut self, addr: u64, data: &[u8]) -> Result<()> {
        let mut memory = self.memory.write().unwrap();
        if addr + data.len() as u64 > self.size {
            return Err(crate::EmulatorError::Memory("Address out of bounds".to_string()));
        }
        memory[addr as usize..addr as usize + data.len()].copy_from_slice(data);
        Ok(())
    }

    /// Check if an address is valid
    pub fn is_valid_address(&self, addr: u64) -> bool {
        addr < self.size
    }

    /// Get a reference to the underlying memory for device access
    pub fn get_memory_ref(&self) -> Arc<RwLock<Vec<u8>>> {
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

