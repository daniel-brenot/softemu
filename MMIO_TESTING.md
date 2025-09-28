# MMIO Testing Guide

This document explains how to test MMIO (Memory-Mapped I/O) device functionality in the softemu virtual machine.

## Overview

The MMIO testing suite provides comprehensive tests to verify that:
1. MMIO devices can be registered at specific addresses
2. MMIO devices respond correctly to read/write operations
3. MMIO device operations are properly tracked and verified
4. Error conditions are handled correctly
5. Multiple devices can coexist without conflicts

## Test Structure

The MMIO tests are organized into several modules:

### 1. Core MMIO Tests (`src/test/mmio.rs`)
- **TestMmioDevice**: A test device that tracks all read/write operations
- **FailingTestMmioDevice**: A device that can be configured to fail on specific operations
- Basic MMIO manager functionality tests

### 2. Simple MMIO Tests (`src/test/simple_mmio_tests.rs`)
- Working tests that don't require access to private VM fields
- Tests basic MMIO manager functionality
- Tests device registration, operation tracking, and error handling

### 3. VM Integration Tests (`src/test/vm_mmio.rs`)
- Tests MMIO functionality within the virtual machine context
- Tests device registration with the VM's MMIO manager
- Tests device operations through the VM interface

### 4. CPU-MMIO Integration Tests (`src/test/cpu_mmio_integration.rs`)
- Tests the integration between CPU memory operations and MMIO devices
- Demonstrates expected behavior when CPU accesses MMIO addresses
- Shows separation between regular memory and MMIO space

### 5. Comprehensive Tests (`src/test/comprehensive_mmio_test.rs`)
- End-to-end tests that demonstrate complete MMIO workflows
- Tests multiple devices, error scenarios, and performance characteristics

## Running the Tests

To run all MMIO tests:

```bash
cargo test test::mmio
cargo test test::simple_mmio_tests
cargo test test::vm_mmio
cargo test test::cpu_mmio_integration
cargo test test::comprehensive_mmio_test
```

To run a specific test:

```bash
cargo test test_mmio_device_registration
cargo test test_mmio_device_operation_tracking
cargo test test_vm_mmio_device_registration
```

## Test Device Usage

### Creating a Test Device

```rust
use crate::test::mmio::TestMmioDevice;

// Create a basic test device
let device = Box::new(TestMmioDevice::new("my_device", 0x100));

// Create a device with custom behavior
let device = Box::new(TestMmioDevice::new("custom_device", 0x200)
    .with_read_return_value(0x12345678)
    .with_write_should_succeed(true));
```

### Creating a Failing Test Device

```rust
use crate::test::mmio::FailingTestMmioDevice;

// Create a device that fails on read operations
let device = Box::new(FailingTestMmioDevice::new("failing_device", 0x100)
    .fail_on_read(true)
    .fail_on_write(false));

// Create a device that fails on specific addresses
let device = Box::new(FailingTestMmioDevice::new("selective_failing", 0x100)
    .add_fail_address(0x10)
    .add_fail_address(0x20));
```

## Test Scenarios

### 1. Basic Device Registration

```rust
#[test]
fn test_device_registration() {
    let mut manager = MmioManager::new();
    let device = Box::new(TestMmioDevice::new("test_device", 0x100));
    
    // Register device at address 0x1000
    let result = manager.register_device(0x1000, device);
    assert!(result.is_ok());
    
    // Verify device is accessible
    assert!(manager.is_mmio_address(0x1000));
    assert!(manager.is_mmio_address(0x10FF)); // Last address in range
    assert!(!manager.is_mmio_address(0x1100)); // Outside range
}
```

### 2. Device Operations

```rust
#[test]
fn test_device_operations() {
    let mut manager = MmioManager::new();
    let device = Box::new(TestMmioDevice::new("ops_device", 0x100)
        .with_read_return_value(0x12345678));
    
    manager.register_device(0x2000, device).unwrap();
    
    // Test read operation
    let result = manager.read(0x2000, 4);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0x12345678);
    
    // Test write operation
    let result = manager.write(0x2000, 0xDEADBEEF, 4);
    assert!(result.is_ok());
}
```

### 3. Error Handling

```rust
#[test]
fn test_error_handling() {
    let mut manager = MmioManager::new();
    
    // Test access to unregistered address
    let result = manager.read(0x3000, 4);
    assert!(result.is_err());
    
    // Test device overlap detection
    let device1 = Box::new(TestMmioDevice::new("device1", 0x100));
    let device2 = Box::new(TestMmioDevice::new("device2", 0x100));
    
    manager.register_device(0x4000, device1).unwrap();
    let result = manager.register_device(0x4080, device2); // Overlapping
    assert!(result.is_err());
}
```

## Integration with Virtual Machine

### Current Limitations

The current implementation has some limitations:

1. **No CPU-MMIO Integration**: The CPU directly accesses guest memory without checking for MMIO devices
2. **Private Field Access**: Some tests require access to private VM fields that aren't exposed
3. **Limited Device Access**: Once a device is registered, it's difficult to access its operation history

### Future Improvements

To fully integrate MMIO testing with the VM, the following changes would be needed:

1. **CPU Memory Integration**: Modify CPU memory access to check MMIO devices first
2. **Public MMIO Interface**: Add public methods to VirtualMachine for MMIO testing
3. **Device Access**: Allow access to registered devices for operation verification

### Example Integration Test

```rust
#[test]
fn test_vm_mmio_integration() {
    let mut vm = VirtualMachine::new(1024 * 1024, 1).unwrap();
    
    // Register a test device
    let device = Box::new(TestMmioDevice::new("vm_test", 0x100)
        .with_read_return_value(0x12345678));
    
    // This would require a public method on VirtualMachine
    // vm.register_test_device(0x10000, device).unwrap();
    
    // Test device operations through VM
    // let result = vm.test_mmio_read(0x10000, 4);
    // assert!(result.is_ok());
    // assert_eq!(result.unwrap(), 0x12345678);
}
```

## Best Practices

1. **Use Descriptive Names**: Give test devices meaningful names that describe their purpose
2. **Test Edge Cases**: Include tests for boundary conditions, error scenarios, and edge cases
3. **Verify Operation Tracking**: When possible, verify that devices are actually called
4. **Test Performance**: Include performance tests for high-frequency operations
5. **Document Expected Behavior**: Clearly document what each test verifies

## Troubleshooting

### Common Issues

1. **Test Failures**: Check that device addresses don't overlap with existing devices
2. **Compilation Errors**: Ensure all necessary imports are included
3. **Runtime Errors**: Verify that devices are properly registered before use

### Debug Tips

1. **Use Logging**: Enable debug logging to see MMIO operations
2. **Check Device Lists**: Use `list_devices()` to verify device registration
3. **Verify Address Ranges**: Ensure test addresses are within device ranges

## Conclusion

The MMIO testing suite provides a comprehensive framework for testing MMIO device functionality. While some integration tests require additional VM modifications, the core MMIO functionality is well-tested and can be used to verify that devices are properly registered and called by the virtual machine.

For questions or issues with MMIO testing, refer to the test source code or create an issue in the project repository.
