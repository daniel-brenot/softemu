use std::sync::{Arc, RwLock};

/// Standalone memory example that demonstrates basic memory operations
/// This example doesn't depend on any external crates
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("SoftEmu Standalone Memory Example");
    println!("==================================");

    // Create guest memory with 512MB
    let mut memory = StandaloneMemory::new(512 * 1024 * 1024)?;
    println!("Created guest memory with 512MB");

    // Test basic memory operations
    println!("Testing memory operations...");
    
    // Write some test data
    let test_data = vec![0x48, 0xC7, 0xC0, 0x78, 0x56, 0x34, 0x12, 0xF4]; // mov $0x12345678, %rax; hlt
    memory.write_slice(0x100000, &test_data)?;
    println!("✓ Wrote test data to address 0x100000");

    // Read it back
    let read_data = memory.read_slice(0x100000, test_data.len())?;
    if read_data == test_data {
        println!("✓ Memory read/write verification successful");
    } else {
        println!("✗ Memory read/write verification failed");
        return Ok(());
    }

    // Test individual byte operations
    memory.write_u8(0x200000, 0xAB)?;
    let byte = memory.read_u8(0x200000)?;
    if byte == 0xAB {
        println!("✓ Byte read/write verification successful");
    } else {
        println!("✗ Byte read/write verification failed");
    }

    // Test 32-bit operations
    memory.write_u32(0x300000, 0x12345678)?;
    let word = memory.read_u32(0x300000)?;
    if word == 0x12345678 {
        println!("✓ 32-bit read/write verification successful");
    } else {
        println!("✗ 32-bit read/write verification failed");
    }

    // Test 64-bit operations
    memory.write_u64(0x400000, 0x123456789ABCDEF0)?;
    let qword = memory.read_u64(0x400000)?;
    if qword == 0x123456789ABCDEF0 {
        println!("✓ 64-bit read/write verification successful");
    } else {
        println!("✗ 64-bit read/write verification failed");
    }

    // Test bounds checking
    match memory.read_u8(512 * 1024 * 1024) {
        Ok(_) => println!("✗ Bounds checking failed - should have errored"),
        Err(_) => println!("✓ Bounds checking working correctly"),
    }

    println!("All memory operations completed successfully!");
    Ok(())
}

/// Standalone memory implementation
#[derive(Debug, Clone)]
struct StandaloneMemory {
    memory: Arc<RwLock<Vec<u8>>>,
    size: u64,
}

impl StandaloneMemory {
    /// Create a new memory instance
    pub fn new(size: u64) -> Result<Self, String> {
        let memory = vec![0u8; size as usize];
        Ok(Self {
            memory: Arc::new(RwLock::new(memory)),
            size,
        })
    }

    /// Read a byte from memory
    pub fn read_u8(&self, addr: u64) -> Result<u8, String> {
        let memory = self.memory.read().map_err(|e| e.to_string())?;
        if addr >= self.size {
            return Err("Address out of bounds".to_string());
        }
        Ok(memory[addr as usize])
    }

    /// Read a 32-bit value from memory
    pub fn read_u32(&self, addr: u64) -> Result<u32, String> {
        let memory = self.memory.read().map_err(|e| e.to_string())?;
        if addr + 3 >= self.size {
            return Err("Address out of bounds".to_string());
        }
        let bytes = &memory[addr as usize..addr as usize + 4];
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    /// Read a 64-bit value from memory
    pub fn read_u64(&self, addr: u64) -> Result<u64, String> {
        let memory = self.memory.read().map_err(|e| e.to_string())?;
        if addr + 7 >= self.size {
            return Err("Address out of bounds".to_string());
        }
        let bytes = &memory[addr as usize..addr as usize + 8];
        Ok(u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7]
        ]))
    }

    /// Write a byte to memory
    pub fn write_u8(&mut self, addr: u64, value: u8) -> Result<(), String> {
        let mut memory = self.memory.write().map_err(|e| e.to_string())?;
        if addr >= self.size {
            return Err("Address out of bounds".to_string());
        }
        memory[addr as usize] = value;
        Ok(())
    }

    /// Write a 32-bit value to memory
    pub fn write_u32(&mut self, addr: u64, value: u32) -> Result<(), String> {
        let mut memory = self.memory.write().map_err(|e| e.to_string())?;
        if addr + 3 >= self.size {
            return Err("Address out of bounds".to_string());
        }
        let bytes = value.to_le_bytes();
        memory[addr as usize..addr as usize + 4].copy_from_slice(&bytes);
        Ok(())
    }

    /// Write a 64-bit value to memory
    pub fn write_u64(&mut self, addr: u64, value: u64) -> Result<(), String> {
        let mut memory = self.memory.write().map_err(|e| e.to_string())?;
        if addr + 7 >= self.size {
            return Err("Address out of bounds".to_string());
        }
        let bytes = value.to_le_bytes();
        memory[addr as usize..addr as usize + 8].copy_from_slice(&bytes);
        Ok(())
    }

    /// Read a slice of bytes from memory
    pub fn read_slice(&self, addr: u64, len: usize) -> Result<Vec<u8>, String> {
        let memory = self.memory.read().map_err(|e| e.to_string())?;
        if addr + len as u64 > self.size {
            return Err("Address out of bounds".to_string());
        }
        Ok(memory[addr as usize..addr as usize + len].to_vec())
    }

    /// Write a slice of bytes to memory
    pub fn write_slice(&mut self, addr: u64, data: &[u8]) -> Result<(), String> {
        let mut memory = self.memory.write().map_err(|e| e.to_string())?;
        if addr + data.len() as u64 > self.size {
            return Err("Address out of bounds".to_string());
        }
        memory[addr as usize..addr as usize + data.len()].copy_from_slice(data);
        Ok(())
    }
}
