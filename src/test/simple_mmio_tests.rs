use crate::memory::{MmioDevice, MmioManager};
use crate::test::mmio::{TestMmioDevice, FailingTestMmioDevice};
use crate::Result;

/// Simple, working MMIO tests that don't require access to private VM fields
/// 
/// These tests demonstrate the core MMIO functionality and can be used to verify
/// that MMIO devices are properly registered and called by the VM.
#[cfg(test)]
mod simple_mmio_tests {
    use super::*;

    #[test]
    fn test_mmio_manager_basic_functionality() {
        let mut manager = MmioManager::new();
        
        // Test empty manager
        assert!(!manager.is_mmio_address(0x1000));
        assert!(manager.list_devices().is_empty());
        
        // Register a test device
        let device = Box::new(TestMmioDevice::new("basic_test", 0x100)
            .with_read_return_value(0x12345678));
        
        let result = manager.register_device(0x1000, device);
        assert!(result.is_ok());
        
        // Verify device is registered
        assert!(manager.is_mmio_address(0x1000));
        assert!(manager.is_mmio_address(0x10FF)); // Last address in range
        assert!(!manager.is_mmio_address(0x1100)); // Outside range
        
        // Test device operations
        let read_result = manager.read(0x1000, 4);
        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap(), 0x12345678);
        
        let write_result = manager.write(0x1000, 0xDEADBEEF, 4);
        assert!(write_result.is_ok());
        
        // Check device list
        let devices = manager.list_devices();
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0], (0x1000, 0x10FF, "basic_test".to_string()));
    }

    #[test]
    fn test_mmio_device_operation_tracking() {
        let mut manager = MmioManager::new();
        
        // Create a device that tracks operations
        let device = Box::new(TestMmioDevice::new("tracking_test", 0x200)
            .with_read_return_value(0xABCDEF00));
        
        manager.register_device(0x2000, device).unwrap();
        
        // Perform various operations
        let operations = vec![
            (0x2000, 1, 0xABCDEF00), // Read 1 byte at offset 0
            (0x2004, 2, 0xABCDEF00), // Read 2 bytes at offset 4
            (0x2008, 4, 0xABCDEF00), // Read 4 bytes at offset 8
            (0x200C, 8, 0xABCDEF00), // Read 8 bytes at offset 12
        ];
        
        for (addr, size, expected_value) in operations {
            let result = manager.read(addr, size);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected_value);
        }
        
        // Perform write operations
        let write_operations = vec![
            (0x2010, 0x11111111, 4),
            (0x2014, 0x22222222, 4),
            (0x2018, 0x3333333333333333, 8),
        ];
        
        for (addr, value, size) in write_operations {
            let result = manager.write(addr, value, size);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_mmio_device_address_mapping() {
        let mut manager = MmioManager::new();
        
        // Register device at specific address
        let device = Box::new(TestMmioDevice::new("address_test", 0x100)
            .with_read_return_value(0x12345678));
        
        manager.register_device(0x3000, device).unwrap();
        
        // Test that device responds at correct addresses
        let result1 = manager.read(0x3000, 4); // Offset 0
        assert!(result1.is_ok());
        assert_eq!(result1.unwrap(), 0x12345678);
        
        let result2 = manager.read(0x3004, 4); // Offset 4
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap(), 0x12345678);
        
        let result3 = manager.read(0x3008, 4); // Offset 8
        assert!(result3.is_ok());
        assert_eq!(result3.unwrap(), 0x12345678);
        
        // Test that device doesn't respond outside its range
        let result4 = manager.read(0x3100, 4); // Outside range
        assert!(result4.is_err());
    }

    #[test]
    fn test_mmio_device_size_handling() {
        let mut manager = MmioManager::new();
        
        // Register device with specific size
        let device = Box::new(TestMmioDevice::new("size_test", 0x200)
            .with_read_return_value(0x87654321));
        
        manager.register_device(0x4000, device).unwrap();
        
        // Test different access sizes
        let result1 = manager.read(0x4000, 1); // 1 byte
        assert!(result1.is_ok());
        assert_eq!(result1.unwrap(), 0x87654321);
        
        let result2 = manager.read(0x4000, 2); // 2 bytes
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap(), 0x87654321);
        
        let result3 = manager.read(0x4000, 4); // 4 bytes
        assert!(result3.is_ok());
        assert_eq!(result3.unwrap(), 0x87654321);
        
        let result4 = manager.read(0x4000, 8); // 8 bytes
        assert!(result4.is_ok());
        assert_eq!(result4.unwrap(), 0x87654321);
    }

    #[test]
    fn test_mmio_device_failure_handling() {
        let mut manager = MmioManager::new();
        
        // Register a failing device
        let device = Box::new(FailingTestMmioDevice::new("failing_test", 0x100)
            .fail_on_read(true)
            .fail_on_write(false));
        
        manager.register_device(0x5000, device).unwrap();
        
        // Test failing read
        let result = manager.read(0x5000, 4);
        assert!(result.is_err());
        
        // Test successful write
        let result = manager.write(0x5000, 0x12345678, 4);
        assert!(result.is_ok());
    }

    #[test]
    fn test_mmio_device_overlap_detection() {
        let mut manager = MmioManager::new();
        
        // Register first device
        let device1 = Box::new(TestMmioDevice::new("device1", 0x100));
        assert!(manager.register_device(0x6000, device1).is_ok());
        
        // Try to register overlapping device
        let device2 = Box::new(TestMmioDevice::new("device2", 0x100));
        let result = manager.register_device(0x6080, device2);
        assert!(result.is_err());
        
        // Verify first device is still registered
        assert!(manager.is_mmio_address(0x6000));
        assert!(manager.is_mmio_address(0x60FF));
    }

    #[test]
    fn test_multiple_mmio_devices() {
        let mut manager = MmioManager::new();
        
        // Register multiple devices
        let device1 = Box::new(TestMmioDevice::new("multi1", 0x100)
            .with_read_return_value(0x11111111));
        let device2 = Box::new(TestMmioDevice::new("multi2", 0x200)
            .with_read_return_value(0x22222222));
        let device3 = Box::new(TestMmioDevice::new("multi3", 0x50)
            .with_read_return_value(0x33333333));
        
        manager.register_device(0x7000, device1).unwrap();
        manager.register_device(0x8000, device2).unwrap();
        manager.register_device(0x9000, device3).unwrap();
        
        // Test each device
        let result1 = manager.read(0x7000, 4);
        assert!(result1.is_ok());
        assert_eq!(result1.unwrap(), 0x11111111);
        
        let result2 = manager.read(0x8000, 4);
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap(), 0x22222222);
        
        let result3 = manager.read(0x9000, 4);
        assert!(result3.is_ok());
        assert_eq!(result3.unwrap(), 0x33333333);
        
        // Test boundaries
        assert!(manager.is_mmio_address(0x70FF)); // Last address of device1
        assert!(!manager.is_mmio_address(0x7100)); // After device1
        assert!(manager.is_mmio_address(0x81FF)); // Last address of device2
        assert!(!manager.is_mmio_address(0x8200)); // After device2
        assert!(manager.is_mmio_address(0x904F)); // Last address of device3
        assert!(!manager.is_mmio_address(0x9050)); // After device3
    }

    #[test]
    fn test_mmio_device_list_and_enumeration() {
        let mut manager = MmioManager::new();
        
        // Register multiple devices
        let device1 = Box::new(TestMmioDevice::new("enum1", 0x100));
        let device2 = Box::new(TestMmioDevice::new("enum2", 0x200));
        let device3 = Box::new(TestMmioDevice::new("enum3", 0x50));
        
        manager.register_device(0xA000, device1).unwrap();
        manager.register_device(0xB000, device2).unwrap();
        manager.register_device(0xC000, device3).unwrap();
        
        // Get device list
        let devices = manager.list_devices();
        assert_eq!(devices.len(), 3);
        
        // Devices should be sorted by address
        assert_eq!(devices[0], (0xA000, 0xA0FF, "enum1".to_string()));
        assert_eq!(devices[1], (0xB000, 0xB1FF, "enum2".to_string()));
        assert_eq!(devices[2], (0xC000, 0xC04F, "enum3".to_string()));
    }

    #[test]
    fn test_mmio_device_performance() {
        let mut manager = MmioManager::new();
        
        // Register a device
        let device = Box::new(TestMmioDevice::new("perf_test", 0x100)
            .with_read_return_value(0x12345678));
        
        manager.register_device(0xD000, device).unwrap();
        
        // Perform many operations to test performance
        let num_operations = 1000;
        let start_time = std::time::Instant::now();
        
        for i in 0..num_operations {
            let addr = 0xD000 + (i % 0x100);
            let result = manager.read(addr, 4);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 0x12345678);
        }
        
        let duration = start_time.elapsed();
        println!("Performed {} MMIO read operations in {:?}", num_operations, duration);
        
        // Verify reasonable performance (should complete in reasonable time)
        assert!(duration.as_millis() < 1000); // Should complete in less than 1 second
    }

    #[test]
    fn test_mmio_device_error_messages() {
        let manager = MmioManager::new();
        
        // Test error message for unregistered address
        let result = manager.read(0xE000, 4);
        assert!(result.is_err());
        
        if let Err(crate::EmulatorError::Device(msg)) = result {
            assert!(msg.contains("No MMIO device at address"));
            assert!(msg.contains("0xe000"));
        } else {
            panic!("Expected Device error");
        }
    }

    #[test]
    fn test_mmio_device_concurrent_access_simulation() {
        let mut manager = MmioManager::new();
        
        // Register device
        let device = Box::new(TestMmioDevice::new("concurrent_test", 0x100)
            .with_read_return_value(0x11223344));
        
        manager.register_device(0xF000, device).unwrap();
        
        // Simulate concurrent access by performing multiple operations
        let result1 = manager.read(0xF000, 4);
        assert!(result1.is_ok());
        assert_eq!(result1.unwrap(), 0x11223344);
        
        let result2 = manager.write(0xF004, 0x55667788, 4);
        assert!(result2.is_ok());
        
        let result3 = manager.read(0xF008, 4);
        assert!(result3.is_ok());
        assert_eq!(result3.unwrap(), 0x11223344);
        
        // Test that all operations succeeded
        assert!(manager.is_mmio_address(0xF000));
        assert!(manager.is_mmio_address(0xF004));
        assert!(manager.is_mmio_address(0xF008));
    }
}
