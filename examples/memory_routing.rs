use softemu::memory::{GuestMemory, MemoryManager, MmioManager};
use softemu::devices::SerialConsole;
use std::sync::{Arc, Mutex};

fn main() {
    println!("Memory Routing Test");
    println!("==================");
    
    // Create guest memory (512MB)
    let guest_memory = GuestMemory::new(512 * 1024 * 1024).unwrap();
    
    // Create MMIO manager
    let mut mmio_manager = MmioManager::new();
    
    // Register a serial console at COM1 (0x3F8)
    let console = Box::new(SerialConsole::new());
    mmio_manager.register_device(0x3F8, console).unwrap();
    
    // Create shared MMIO manager reference
    let mmio_manager_ref = Arc::new(Mutex::new(mmio_manager));
    
    // Define MMIO space size (first 1MB is reserved for MMIO)
    let mmio_space_size = 0x100000; // 1MB
    
    // Create memory manager
    let mut memory_manager = MemoryManager::new(guest_memory, mmio_manager_ref, mmio_space_size);
    
    println!("Memory Manager created:");
    println!("  - Total address space: 0x{:x} bytes", memory_manager.total_address_space_size());
    println!("  - MMIO space: 0x{:x} bytes (0x0 - 0x{:x})", 
             memory_manager.mmio_space_size(), 
             memory_manager.mmio_space_size() - 1);
    println!("  - Guest memory: 0x{:x} bytes (0x{:x} - 0x{:x})", 
             memory_manager.guest_memory_size(),
             memory_manager.mmio_space_size(),
             memory_manager.total_address_space_size() - 1);
    println!();
    
    // Test MMIO access
    println!("Testing MMIO access:");
    println!("  - Address 0x3F8 (COM1) is MMIO: {}", memory_manager.is_mmio_address(0x3F8));
    println!("  - Address 0x100000 (1MB) is MMIO: {}", memory_manager.is_mmio_address(0x100000));
    println!();
    
    // Test guest memory access
    println!("Testing guest memory access:");
    println!("  - Address 0x100000 (1MB) is guest memory: {}", memory_manager.is_guest_memory_address(0x100000));
    println!("  - Address 0x3F8 (COM1) is guest memory: {}", memory_manager.is_guest_memory_address(0x3F8));
    println!();
    
    // Test address routing with offset subtraction
    println!("Testing address routing with offset subtraction:");
    
    // Write to guest memory at address 0x100001 (1MB + 1 byte)
    // This should be routed to guest memory at offset 1 (0x100001 - 0x100000 = 1)
    let test_value = 0x42u8;
    memory_manager.write_u8(0x100001, test_value).unwrap();
    
    // Read back from the same address
    let read_value = memory_manager.read_u8(0x100001).unwrap();
    println!("  - Wrote 0x{:02x} to address 0x100001", test_value);
    println!("  - Read 0x{:02x} from address 0x100001", read_value);
    println!("  - Values match: {}", test_value == read_value);
    
    // Test that the value is actually stored at the correct offset in guest memory
    let guest_memory = memory_manager.get_guest_memory();
    let direct_read = guest_memory.read_u8(1).unwrap(); // Offset 1 in guest memory
    println!("  - Direct read from guest memory offset 1: 0x{:02x}", direct_read);
    println!("  - Direct read matches: {}", test_value == direct_read);
    println!();
    
    // Test MMIO access
    println!("Testing MMIO access:");
    // Read from COM1 status register (offset 5 = LSR)
    let lsr_value = memory_manager.read_u8(0x3FD).unwrap(); // 0x3F8 + 5
    println!("  - Read LSR from COM1 (0x3FD): 0x{:02x}", lsr_value);
    
    // Write to COM1 data register (offset 0 = THR)
    memory_manager.write_u8(0x3F8, b'H').unwrap();
    memory_manager.write_u8(0x3F8, b'i').unwrap();
    memory_manager.write_u8(0x3F8, b'!').unwrap();
    memory_manager.write_u8(0x3F8, b'\n').unwrap();
    println!("  - Wrote 'Hi!' to COM1 data register");
    println!();
    
    // Test boundary conditions
    println!("Testing boundary conditions:");
    println!("  - Address 0x{:x} (last MMIO address) is MMIO: {}", 
             memory_manager.mmio_space_size() - 1,
             memory_manager.is_mmio_address(memory_manager.mmio_space_size() - 1));
    println!("  - Address 0x{:x} (first guest memory address) is guest memory: {}", 
             memory_manager.mmio_space_size(),
             memory_manager.is_guest_memory_address(memory_manager.mmio_space_size()));
    println!();
    
    println!("Memory routing test completed successfully!");
}
