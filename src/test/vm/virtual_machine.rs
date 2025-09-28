use crate::memory::{MmioDevice, MmioManager};
use crate::vm::VirtualMachine;
use crate::Result;
use std::sync::{Arc, Mutex};

/// Test MMIO device that tracks all operations
#[derive(Debug)]
struct TestMmioDevice {
    name: String,
    size: u64,
    read_count: u64,
    write_count: u64,
    last_read_offset: u64,
    last_read_size: u8,
    last_write_offset: u64,
    last_write_value: u64,
    last_write_size: u8,
    read_values: std::collections::HashMap<u64, u64>, // offset -> value
}

impl TestMmioDevice {
    fn new(name: &str, size: u64) -> Self {
        Self {
            name: name.to_string(),
            size,
            read_count: 0,
            write_count: 0,
            last_read_offset: 0,
            last_read_size: 0,
            last_write_offset: 0,
            last_write_value: 0,
            last_write_size: 0,
            read_values: std::collections::HashMap::new(),
        }
    }

    fn get_read_count(&self) -> u64 {
        self.read_count
    }

    fn get_write_count(&self) -> u64 {
        self.write_count
    }

    fn get_last_read_offset(&self) -> u64 {
        self.last_read_offset
    }

    fn get_last_write_offset(&self) -> u64 {
        self.last_write_offset
    }

    fn get_last_write_value(&self) -> u64 {
        self.last_write_value
    }

    fn set_read_value(&mut self, offset: u64, value: u64) {
        self.read_values.insert(offset, value);
    }
}

impl MmioDevice for TestMmioDevice {
    fn read(&self, offset: u64, size: u8) -> Result<u64> {
        // This is a read-only device for testing
        // Return a predictable value based on offset
        Ok(offset * 2 + size as u64)
    }

    fn write(&mut self, offset: u64, value: u64, size: u8) -> Result<()> {
        self.write_count += 1;
        self.last_write_offset = offset;
        self.last_write_value = value;
        self.last_write_size = size;
        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn size(&self) -> u64 {
        self.size
    }
}

/// Test that verifies MMIO integration in the VM
#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_vm() -> Result<VirtualMachine> {
        VirtualMachine::new(8 * 1024 * 1024, 1) // 8MB memory, 1 CPU core
    }

    #[test]
    fn test_vm_mmio_device_registration() -> Result<()> {
        println!("Creating test VM...");
        let mut vm = create_test_vm()?;
        println!("VM created successfully");
        
        // Create a test MMIO device
        let test_device = Box::new(TestMmioDevice::new("TestDevice", 0x100));
        
        // Register the device at a specific address
        let mmio_addr = 0x1000;
        println!("Registering MMIO device at address 0x{:x}", mmio_addr);
        vm.register_mmio_device(mmio_addr, test_device)?;
        println!("MMIO device registered successfully");
        
        // Verify the device is registered by checking if the address is MMIO
        let mmio_manager = vm.get_mmio_manager();
        let mmio_manager = mmio_manager.lock().unwrap();
        assert!(mmio_manager.is_mmio_address(mmio_addr));
        assert!(!mmio_manager.is_mmio_address(mmio_addr + 0x200)); // Outside device range
        drop(mmio_manager); // Release the lock before calling read_memory
        
        // Test that we can read from the MMIO device
        println!("Reading from MMIO device at address 0x{:x}", mmio_addr);
        let device_value = vm.read_memory(mmio_addr, 4)?;
        println!("Successfully read value: {}", device_value);
        assert_eq!(device_value, 4); // offset * 2 + size = 0 * 2 + 4 = 4
        
        Ok(())
    }

    #[test]
    fn test_vm_memory_access_routing() -> Result<()> {
        let mut vm = create_test_vm()?;
        
        // Test that regular memory access works
        let test_addr = 0x1000; // Use a smaller address within 1MB memory
        let test_value = 0x12345678;
        
        // Write to regular memory using VM's memory routing
        vm.write_memory(test_addr, test_value, 4)?;
        
        // Read from regular memory using VM's memory routing
        let read_value = vm.read_memory(test_addr, 4)?;
        assert_eq!(read_value, test_value);
        
        // Test that MMIO addresses are routed to devices
        let mmio_addr = 0x3F8; // COM1 address from VM setup
        
        // The VM should route this to the MMIO device, not guest memory
        // This should not panic and should return a value from the device
        let _device_value = vm.read_memory(mmio_addr, 1)?;
        
        // Test writing to MMIO device
        vm.write_memory(mmio_addr, 0x42, 1)?;
        
        Ok(())
    }

    #[test]
    fn test_cpu_memory_access_integration() -> Result<()> {
        let mut vm = create_test_vm()?;
        
        // Register a test MMIO device
        let test_device = Box::new(TestMmioDevice::new("CpuTestDevice", 0x100));
        let mmio_addr = 0x2000;
        vm.register_mmio_device(mmio_addr, test_device)?;
        
        // Test that regular memory access works through VM routing
        let test_addr = 0x5000; // Use a different address for regular memory
        let test_value = 0xDEADBEEF;
        
        // Write through VM memory routing
        vm.write_memory(test_addr, test_value, 4)?;
        
        // Read through VM memory routing
        let read_value = vm.read_memory(test_addr, 4)?;
        assert_eq!(read_value, test_value);
        
        // Test that MMIO addresses are routed to devices
        // Read from MMIO device
        let device_value = vm.read_memory(mmio_addr, 4)?;
        // The test device returns offset * 2 + size, so for offset 0, size 4: 0 * 2 + 4 = 4
        assert_eq!(device_value, 4);
        
        // Write to MMIO device
        vm.write_memory(mmio_addr, 0x12345678, 4)?;
        
        Ok(())
    }

    #[test]
    fn test_cpu_state_mmio_integration() -> Result<()> {
        let mut vm = create_test_vm()?;
        
        // Register a test MMIO device
        let test_device = Box::new(TestMmioDevice::new("CpuStateTestDevice", 0x100));
        let mmio_addr = 0x3000;
        vm.register_mmio_device(mmio_addr, test_device)?;
        
        // Create a CPU state with MMIO manager
        let memory = vm.get_memory().clone();
        let mmio_manager = vm.get_mmio_manager().clone();
        let mut cpu_state = crate::cpu::state::CpuState::new_with_mmio(memory, mmio_manager);
        
        // Test regular memory access
        let test_addr = 0x6000; // Use a smaller address within 1MB memory
        let test_value = 0xCAFEBABE;
        
        // Write through CPU state
        cpu_state.write_u32(test_addr, test_value)?;
        
        // Read through CPU state
        let read_value = cpu_state.read_u32(test_addr)?;
        assert_eq!(read_value, test_value);
        
        // Test MMIO access through CPU state
        // Read from MMIO device
        let device_value = cpu_state.read_u32(mmio_addr)?;
        // The test device returns offset * 2 + size, so for offset 0, size 4: 0 * 2 + 4 = 4
        assert_eq!(device_value, 4);
        
        // Write to MMIO device
        cpu_state.write_u32(mmio_addr, 0x87654321)?;
        
        // Test reading from different offset in MMIO device
        let device_value2 = cpu_state.read_u32(mmio_addr + 4)?;
        // For offset 4, size 4: 4 * 2 + 4 = 12
        assert_eq!(device_value2, 12);
        
        Ok(())
    }
}
