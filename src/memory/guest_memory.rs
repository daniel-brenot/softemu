use std::{cell::UnsafeCell};

use crate::cpu::fault::Fault;

/// Guest memory management
/// Has interior mutability for the raw memory so that it can 
/// be modified without requiring mutable references and locking.
/// 
/// Undefined behaviour is fine as we are letting the vm handle the behaviour
#[derive(Debug)]
pub struct GuestMemory {
    raw_memory: UnsafeCell<Vec<u8>>,
}

impl GuestMemory {
    /// Create a new guest memory instance
    pub fn new(size: u64) -> Self {
        // Create a simple memory mapping using a Vec as backing storage
        Self {
            raw_memory: UnsafeCell::new(vec![0u8; size as usize]),
        }
    }

    /// Get the total size of guest memory
    pub fn size(&self) -> u64 {
        // SAFETY: This is safe because the vec is never added to after creation
        unsafe { (&*self.raw_memory.get()).len() as u64 }
    }

    /// Read a byte from guest memory
    pub fn read_u8(&self, addr: u64) -> Result<u8, Fault> {
        unsafe {
            let memory = &*self.raw_memory.get();
            if addr >= self.size() {
                return Err(Fault::GeneralProtection);
            }
            Ok(memory[addr as usize])
        }
    }

    /// Read a 16-bit value from guest memory
    pub fn read_u16(&self, addr: u64) -> Result<u16, Fault> {
        unsafe {
            let memory = &*self.raw_memory.get();
            if addr + 1 >= self.size() {
                return Err(Fault::GeneralProtection);
            }
            let bytes = &memory[addr as usize..addr as usize + 2];
            Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
        }
    }

    /// Read a 32-bit value from guest memory
    pub fn read_u32(&self, addr: u64) -> Result<u32, Fault> {
        unsafe {
            let memory = &*self.raw_memory.get();
            if addr + 3 >= self.size() {
                return Err(Fault::GeneralProtection);
            }
            let bytes = &memory[addr as usize..addr as usize + 4];
            Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
        }
    }

    /// Read a 64-bit value from guest memory
    pub fn read_u64(&self, addr: u64) -> Result<u64, Fault> {
        unsafe {
            let memory = &*self.raw_memory.get();
            if addr + 7 >= self.size() {
                return Err(Fault::GeneralProtection);
            }
            let bytes = &memory[addr as usize..addr as usize + 8];
            Ok(u64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3],
                bytes[4], bytes[5], bytes[6], bytes[7]
            ]))
        }
    }

    /// Read a slice of bytes from guest memory
    pub fn read_slice(&self, addr: u64, len: usize) -> Result<Vec<u8>, Fault> {
        unsafe {
            let memory = &*self.raw_memory.get();
            if addr + len as u64 > self.size() {
                return Err(Fault::GeneralProtection);
            }
            Ok(memory[addr as usize..addr as usize + len].to_vec())
        }
    }

    /// Write a byte to guest memory
    pub fn write_u8(&self, addr: u64, value: u8) -> Result<(), Fault> {
        unsafe {
            let memory = &mut *self.raw_memory.get();
            if addr >= self.size() {
                return Err(Fault::GeneralProtection);
            }
            memory[addr as usize] = value;
            Ok(())
        }
    }

    /// Write a 16-bit value to guest memory
    pub fn write_u16(&self, addr: u64, value: u16) -> Result<(), Fault> {
        unsafe {
            let memory = &mut *self.raw_memory.get();
            if addr + 1 >= self.size() {
                return Err(Fault::GeneralProtection);
            }
            let bytes = value.to_le_bytes();
            memory[addr as usize..addr as usize + 2].copy_from_slice(&bytes);
            Ok(())
        }
    }

    /// Write a 32-bit value to guest memory
    pub fn write_u32(&self, addr: u64, value: u32) -> Result<(), Fault> {
        unsafe {
            let memory = &mut *self.raw_memory.get();
            if addr + 3 >= self.size() {
                return Err(Fault::GeneralProtection);
            }
            let bytes = value.to_le_bytes();
            memory[addr as usize..addr as usize + 4].copy_from_slice(&bytes);
            Ok(())
        }
    }

    /// Write a 64-bit value to guest memory
    pub fn write_u64(&self, addr: u64, value: u64) -> Result<(), Fault> {
        unsafe {
            let memory = &mut *self.raw_memory.get();
            if addr + 7 >= self.size() {
                return Err(Fault::GeneralProtection);
            }
            let bytes = value.to_le_bytes();
            memory[addr as usize..addr as usize + 8].copy_from_slice(&bytes);
            Ok(())
        }
    }

    /// Write a slice of bytes to guest memory
    pub fn write_slice(&self, addr: u64, data: &[u8]) -> Result<(), Fault> {
        unsafe {
            let memory = &mut *self.raw_memory.get();
            if addr + data.len() as u64 > self.size() {
                return Err(Fault::GeneralProtection);
            }
            memory[addr as usize..addr as usize + data.len()].copy_from_slice(data);
            Ok(())
        }
    }

    /// Check if an address is valid
    pub fn is_valid_address(&self, addr: u64) -> bool {
        addr < self.size()
    }

    /// Create a memory mapping for a device
    pub fn create_device_mapping(&self, start_addr: u64, size: u64) -> Result<(), Fault> {
        // In a real implementation, this would create a memory mapping
        // for MMIO devices. For now, we'll just validate the range.
        if start_addr + size > self.size() {
            return Err(Fault::GeneralProtection);
        }
        Ok(())
    }
}

