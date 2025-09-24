use anyhow::Result;
use softemu::VirtualMachine;

/// Simple example that just demonstrates VM creation and memory access
fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    println!("SoftEmu Simple Example");
    println!("======================");

    // Create a virtual machine with 512MB RAM and 1 CPU core
    let mut vm = VirtualMachine::new(512 * 1024 * 1024, 1)?;
    println!("Created VM with 512MB RAM and 1 CPU core");

    // Load a simple test kernel
    let test_kernel = create_test_kernel();
    vm.get_memory_mut().write_slice(0x100000, &test_kernel)?;
    println!("Loaded test kernel at 0x100000");

    // Verify the kernel was loaded correctly
    let loaded_kernel = vm.get_memory().read_slice(0x100000, test_kernel.len())?;
    if loaded_kernel == test_kernel {
        println!("✓ Kernel verification successful");
    } else {
        println!("✗ Kernel verification failed");
    }

    println!("VM setup complete!");
    Ok(())
}

/// Create a simple test kernel for demonstration
fn create_test_kernel() -> Vec<u8> {
    // Simple x86_64 instructions:
    // mov $0x12345678, %rax
    // hlt
    vec![
        0x48, 0xC7, 0xC0, 0x78, 0x56, 0x34, 0x12, // mov $0x12345678, %rax
        0xF4,                                      // hlt
    ]
}
