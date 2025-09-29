use anyhow::Result;
use softemu::VirtualMachine;
use std::path::Path;

/// Example of creating a virtual machine with network support
fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    println!("SoftEmu Network Example");
    println!("=======================");

    // Create a virtual machine with 1GB RAM and 2 CPU cores
    let mut vm = VirtualMachine::new(1024 * 1024 * 1024, 2)?;
    println!("Created VM with 1GB RAM and 2 CPU cores");

    // Load a test kernel
    let test_kernel = create_network_test_kernel();
    {
        let mut memory_manager = vm.get_memory_manager().lock().unwrap();
        memory_manager.write_slice(0x100000, &test_kernel)?;
    }
    println!("Loaded network test kernel at 0x100000");

    // Enable network support
    // Note: You would need to specify a real network interface
    // For this example, we'll just show how it would be done
    println!("Network support would be enabled with:");
    println!("  vm.enable_network(\"eth0\").await?;");

    // Demonstrate network device creation
    println!("Network devices that would be available:");
    println!("  - Ethernet interface");
    println!("  - ARP protocol support");
    println!("  - ICMP ping support");
    println!("  - Basic TCP/UDP support");

    println!("VM setup complete with network support.");

    Ok(())
}

/// Create a test kernel that demonstrates network functionality
fn create_network_test_kernel() -> Vec<u8> {
    // This would be a more complex kernel that includes network drivers
    // For this example, we'll create a simple kernel that sets up network
    
    // Simple x86_64 instructions for network setup:
    // mov $0x1000, %rax    ; Network buffer address
    // mov $0x2000, %rbx    ; Network control register
    // mov $1, %rcx         ; Enable network
    // mov %rcx, (%rbx)     ; Write to control register
    // hlt                  ; Halt
    
    vec![
        0x48, 0xC7, 0xC0, 0x00, 0x10, 0x00, 0x00, // mov $0x1000, %rax
        0x48, 0xC7, 0xC3, 0x00, 0x20, 0x00, 0x00, // mov $0x2000, %rbx
        0x48, 0xC7, 0xC1, 0x01, 0x00, 0x00, 0x00, // mov $1, %rcx
        0x48, 0x89, 0x0B,                          // mov %rcx, (%rbx)
        0xF4,                                      // hlt
    ]
}
