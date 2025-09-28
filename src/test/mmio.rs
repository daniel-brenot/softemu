use crate::memory::{MmioDevice, MmioManager};
use crate::Result;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// Test MMIO device that tracks all read/write operations
#[derive(Debug, Clone)]
pub struct TestMmioDevice {
    name: String,
    size: u64,
    // Track all operations for verification
    read_operations: Arc<Mutex<VecDeque<(u64, u8, u64)>>>, // (offset, size, result)
    write_operations: Arc<Mutex<VecDeque<(u64, u64, u8)>>>, // (offset, value, size)
    // Device state
    registers: Arc<Mutex<Vec<u64>>>, // Simple register array
    // Configuration
    read_return_value: u64,
    write_should_succeed: bool,
}

impl TestMmioDevice {
    pub fn new(name: &str, size: u64) -> Self {
        Self {
            name: name.to_string(),
            size,
            read_operations: Arc::new(Mutex::new(VecDeque::new())),
            write_operations: Arc::new(Mutex::new(VecDeque::new())),
            registers: Arc::new(Mutex::new(vec![0; (size / 8) as usize])),
            read_return_value: 0xDEADBEEF,
            write_should_succeed: true,
        }
    }

    pub fn with_read_return_value(mut self, value: u64) -> Self {
        self.read_return_value = value;
        self
    }

    pub fn with_write_should_succeed(mut self, should_succeed: bool) -> Self {
        self.write_should_succeed = should_succeed;
        self
    }

    /// Get all read operations that have been performed
    pub fn get_read_operations(&self) -> Vec<(u64, u8, u64)> {
        self.read_operations.lock().unwrap().iter().cloned().collect()
    }

    /// Get all write operations that have been performed
    pub fn get_write_operations(&self) -> Vec<(u64, u64, u8)> {
        self.write_operations.lock().unwrap().iter().cloned().collect()
    }

    /// Clear all operation history
    pub fn clear_operations(&self) {
        self.read_operations.lock().unwrap().clear();
        self.write_operations.lock().unwrap().clear();
    }

    /// Get the number of read operations performed
    pub fn read_count(&self) -> usize {
        self.read_operations.lock().unwrap().len()
    }

    /// Get the number of write operations performed
    pub fn write_count(&self) -> usize {
        self.write_operations.lock().unwrap().len()
    }

    /// Get the value of a register
    pub fn get_register(&self, index: usize) -> u64 {
        let registers = self.registers.lock().unwrap();
        if index < registers.len() {
            registers[index]
        } else {
            0
        }
    }

    /// Set the value of a register
    pub fn set_register(&self, index: usize, value: u64) {
        let mut registers = self.registers.lock().unwrap();
        if index < registers.len() {
            registers[index] = value;
        }
    }
}

impl MmioDevice for TestMmioDevice {
    fn read(&self, offset: u64, size: u8) -> Result<u64> {
        // Record the read operation
        {
            let mut ops = self.read_operations.lock().unwrap();
            ops.push_back((offset, size, self.read_return_value));
            // Keep only the last 100 operations to prevent memory bloat
            if ops.len() > 100 {
                ops.pop_front();
            }
        }

        // Return the configured value
        Ok(self.read_return_value)
    }

    fn write(&mut self, offset: u64, value: u64, size: u8) -> Result<()> {
        // Record the write operation
        {
            let mut ops = self.write_operations.lock().unwrap();
            ops.push_back((offset, value, size));
            // Keep only the last 100 operations to prevent memory bloat
            if ops.len() > 100 {
                ops.pop_front();
            }
        }

        if !self.write_should_succeed {
            return Err(crate::EmulatorError::Device("Test device write failure".to_string()));
        }

        // Store the value in the appropriate register
        let register_index = (offset / 8) as usize;
        self.set_register(register_index, value);

        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn size(&self) -> u64 {
        self.size
    }
}

/// Test MMIO device that can be configured to fail on specific operations
#[derive(Debug)]
pub struct FailingTestMmioDevice {
    base_device: TestMmioDevice,
    fail_on_read: bool,
    fail_on_write: bool,
    fail_addresses: Vec<u64>,
}

impl FailingTestMmioDevice {
    pub fn new(name: &str, size: u64) -> Self {
        Self {
            base_device: TestMmioDevice::new(name, size),
            fail_on_read: false,
            fail_on_write: false,
            fail_addresses: Vec::new(),
        }
    }

    pub fn fail_on_read(mut self, fail: bool) -> Self {
        self.fail_on_read = fail;
        self
    }

    pub fn fail_on_write(mut self, fail: bool) -> Self {
        self.fail_on_write = fail;
        self
    }

    pub fn add_fail_address(mut self, addr: u64) -> Self {
        self.fail_addresses.push(addr);
        self
    }

    pub fn get_base_device(&self) -> &TestMmioDevice {
        &self.base_device
    }
}

impl MmioDevice for FailingTestMmioDevice {
    fn read(&self, offset: u64, size: u8) -> Result<u64> {
        if self.fail_on_read || self.fail_addresses.contains(&offset) {
            return Err(crate::EmulatorError::Device("Test device read failure".to_string()));
        }
        self.base_device.read(offset, size)
    }

    fn write(&mut self, offset: u64, value: u64, size: u8) -> Result<()> {
        if self.fail_on_write || self.fail_addresses.contains(&offset) {
            return Err(crate::EmulatorError::Device("Test device write failure".to_string()));
        }
        self.base_device.write(offset, value, size)
    }

    fn name(&self) -> &str {
        self.base_device.name()
    }

    fn size(&self) -> u64 {
        self.base_device.size()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mmio_device_registration() {
        let mut manager = MmioManager::new();
        let device = Box::new(TestMmioDevice::new("test_device", 0x100));
        
        // Register device at address 0x1000
        let result = manager.register_device(0x1000, device);
        assert!(result.is_ok());
        
        // Check that device is registered
        assert!(manager.is_mmio_address(0x1000));
        assert!(manager.is_mmio_address(0x10FF)); // Last address in range
        assert!(!manager.is_mmio_address(0x1100)); // Outside range
        
        // Check device list
        let devices = manager.list_devices();
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0], (0x1000, 0x10FF, "test_device".to_string()));
    }

    #[test]
    fn test_mmio_device_overlap_detection() {
        let mut manager = MmioManager::new();
        
        // Register first device
        let device1 = Box::new(TestMmioDevice::new("device1", 0x100));
        assert!(manager.register_device(0x1000, device1).is_ok());
        
        // Try to register overlapping device
        let device2 = Box::new(TestMmioDevice::new("device2", 0x100));
        let result = manager.register_device(0x1080, device2);
        assert!(result.is_err());
        
        // Check error message
        if let Err(crate::EmulatorError::Device(msg)) = result {
            assert!(msg.contains("overlaps"));
        } else {
            panic!("Expected Device error");
        }
    }

    #[test]
    fn test_mmio_device_read_write() {
        let mut manager = MmioManager::new();
        let device = Box::new(TestMmioDevice::new("test_device", 0x100)
            .with_read_return_value(0x12345678));
        
        manager.register_device(0x1000, device).unwrap();
        
        // Test read
        let result = manager.read(0x1000, 4);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0x12345678);
        
        // Test write
        let result = manager.write(0x1000, 0xDEADBEEF, 4);
        assert!(result.is_ok());
    }

    #[test]
    fn test_mmio_device_operation_tracking() {
        let mut manager = MmioManager::new();
        let device = Box::new(TestMmioDevice::new("tracking_device", 0x100)
            .with_read_return_value(0xABCDEF00));
        
        manager.register_device(0x2000, device).unwrap();
        
        // Perform some operations
        manager.read(0x2000, 1).unwrap();
        manager.read(0x2004, 2).unwrap();
        manager.write(0x2008, 0x11111111, 4).unwrap();
        manager.write(0x2010, 0x22222222, 8).unwrap();
        
        // Get the device back to check operations
        // Note: This is a limitation of the current design - we can't easily get the device back
        // In a real test, we'd need to modify the MmioManager to allow device access
    }

    #[test]
    fn test_mmio_device_offset_calculation() {
        let mut manager = MmioManager::new();
        let device = Box::new(TestMmioDevice::new("offset_test", 0x100));
        
        manager.register_device(0x3000, device).unwrap();
        
        // Test that offsets are calculated correctly
        // Device is at 0x3000, so:
        // - Address 0x3000 should have offset 0
        // - Address 0x3004 should have offset 4
        // - Address 0x3008 should have offset 8
        
        manager.read(0x3000, 4).unwrap();
        manager.read(0x3004, 4).unwrap();
        manager.read(0x3008, 4).unwrap();
        
        // The actual offset calculation happens inside the device's read/write methods
        // We can't directly verify this without modifying the MmioManager interface
    }

    #[test]
    fn test_mmio_device_not_found() {
        let manager = MmioManager::new();
        
        // Try to read from unregistered address
        let result = manager.read(0x5000, 4);
        assert!(result.is_err());
        
        if let Err(crate::EmulatorError::Device(msg)) = result {
            assert!(msg.contains("No MMIO device at address"));
        } else {
            panic!("Expected Device error");
        }
    }

    #[test]
    fn test_failing_mmio_device() {
        let mut manager = MmioManager::new();
        let device = Box::new(FailingTestMmioDevice::new("failing_device", 0x100)
            .fail_on_read(true)
            .fail_on_write(false));
        
        manager.register_device(0x4000, device).unwrap();
        
        // Test failing read
        let result = manager.read(0x4000, 4);
        assert!(result.is_err());
        
        // Test successful write
        let result = manager.write(0x4000, 0x12345678, 4);
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_mmio_devices() {
        let mut manager = MmioManager::new();
        
        // Register multiple devices
        let device1 = Box::new(TestMmioDevice::new("device1", 0x100));
        let device2 = Box::new(TestMmioDevice::new("device2", 0x200));
        let device3 = Box::new(TestMmioDevice::new("device3", 0x50));
        
        manager.register_device(0x1000, device1).unwrap();
        manager.register_device(0x2000, device2).unwrap();
        manager.register_device(0x3000, device3).unwrap();
        
        // Test that all devices are accessible
        assert!(manager.is_mmio_address(0x1000));
        assert!(manager.is_mmio_address(0x10FF));
        assert!(manager.is_mmio_address(0x2000));
        assert!(manager.is_mmio_address(0x21FF));
        assert!(manager.is_mmio_address(0x3000));
        assert!(manager.is_mmio_address(0x304F));
        
        // Test that addresses between devices are not MMIO
        assert!(!manager.is_mmio_address(0x1100));
        assert!(!manager.is_mmio_address(0x1FFF));
        assert!(!manager.is_mmio_address(0x2200));
        assert!(!manager.is_mmio_address(0x2FFF));
        assert!(!manager.is_mmio_address(0x3050));
        
        // Check device list
        let devices = manager.list_devices();
        assert_eq!(devices.len(), 3);
        
        // Devices should be sorted by address
        assert_eq!(devices[0], (0x1000, 0x10FF, "device1".to_string()));
        assert_eq!(devices[1], (0x2000, 0x21FF, "device2".to_string()));
        assert_eq!(devices[2], (0x3000, 0x304F, "device3".to_string()));
    }
}
