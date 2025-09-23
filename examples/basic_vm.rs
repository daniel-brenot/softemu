use anyhow::Result;
use softemu::VirtualMachine;
use std::path::Path;

/// Basic example of creating and running a virtual machine
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    println!("SoftEmu Basic Example");
    println!("====================");

    // Create a virtual machine with 512MB RAM and 1 CPU core
    let mut vm = VirtualMachine::new(512 * 1024 * 1024, 1)?;
    println!("Created VM with 512MB RAM and 1 CPU core");

    // Load a kernel (you would need to provide a real kernel image)
    // For this example, we'll create a simple test kernel
    let test_kernel = create_test_kernel();
    vm.get_memory_mut().write_slice(0x100000, &test_kernel)?;
    println!("Loaded test kernel at 0x100000");

    // Set up initial CPU state
    // In a real scenario, this would be done by the kernel loader
    println!("Setting up initial CPU state...");

    // Start the virtual machine
    println!("Starting virtual machine...");
    
    // Note: In a real scenario, you would call vm.run().await
    // For this example, we'll just demonstrate the setup
    println!("VM setup complete. In a real scenario, vm.run().await would start execution.");

    Ok(())
}

/// Create a simple test kernel for demonstration
fn create_test_kernel() -> Vec<u8> {
    // This is a very simple "kernel" that just contains some basic instructions
    // In reality, you would load a real Linux kernel or other OS kernel
    
    // Simple x86_64 instructions:
    // mov $0x12345678, %rax
    // hlt
    vec![
        0x48, 0xC7, 0xC0, 0x78, 0x56, 0x34, 0x12, // mov $0x12345678, %rax
        0xF4,                                      // hlt
    ]
}
