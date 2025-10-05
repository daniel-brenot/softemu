use softemu::memory::{
    GuestMemory, MmioManager, MemoryManager, PageTableEntryFlags, PageFaultErrorCode,
    SimplePageFaultHandler, PageFaultManager,
};
use softemu::Result;

fn main() -> Result<()> {
    env_logger::init();
    
    println!("Testing memory paging functionality...");
    
    // Create guest memory
    let guest_memory = GuestMemory::new(1024 * 1024 * 1024)?; // 1GB
    let mmio_manager = MmioManager::new();
    let mmio_space_size = 0x100000; // 1MB MMIO space
    
    // Create memory manager
    let mut memory_manager = MemoryManager::new(guest_memory, mmio_manager, mmio_space_size);
    
    // Test 1: Identity mapping when paging is disabled
    println!("\n=== Test 1: Identity mapping (paging disabled) ===");
    let test_addr = 0x1000;
    let translated = memory_manager.translate_address(test_addr)?;
    println!("Virtual address 0x{:x} -> Physical address 0x{:x}", test_addr, translated);
    assert_eq!(test_addr, translated);
    
    // Test 2: Enable paging and set up page tables
    println!("\n=== Test 2: Setting up paging ===");
    memory_manager.set_paging_enabled(true);
    
    // Set CR3 to a page table root
    let cr3 = 0x100000; // 1MB
    memory_manager.set_cr3(cr3);
    println!("Set CR3 to 0x{:x}", cr3);
    
    // Test 3: Create a page table entry
    println!("\n=== Test 3: Creating page table entry ===");
    let virtual_addr = 0x2000;
    let physical_addr = 0x3000;
    let flags = PageTableEntryFlags::PRESENT | PageTableEntryFlags::WRITABLE | PageTableEntryFlags::USER_ACCESSIBLE;
    
    memory_manager.create_page_table_entry(virtual_addr, physical_addr, flags)?;
    println!("Created page table entry: 0x{:x} -> 0x{:x}", virtual_addr, physical_addr);
    
    // Test 4: Address translation with paging enabled
    println!("\n=== Test 4: Address translation with paging ===");
    let translated = memory_manager.translate_address_with_access(virtual_addr, false, false, false)?;
    println!("Virtual address 0x{:x} -> Physical address 0x{:x}", virtual_addr, translated);
    assert_eq!(physical_addr, translated);
    
    // Test 5: TLB functionality
    println!("\n=== Test 5: TLB functionality ===");
    // First access should populate TLB
    let _ = memory_manager.translate_address_with_access(virtual_addr, false, false, false)?;
    println!("First access - TLB populated");
    
    // Second access should use TLB
    let _ = memory_manager.translate_address_with_access(virtual_addr, false, false, false)?;
    println!("Second access - should use TLB");
    
    // Test 6: Page fault handling
    println!("\n=== Test 6: Page fault handling ===");
    let fault_addr = 0x4000;
    let error_code = PageFaultErrorCode::new(false, false, false, false, false); // Page not present
    
    // This should trigger a page fault
    match memory_manager.translate_address_with_access(fault_addr, false, false, false) {
        Ok(_) => println!("Unexpected: No page fault occurred"),
        Err(e) => {
            println!("Expected page fault: {}", e);
            // Handle the page fault
            memory_manager.handle_page_fault(fault_addr, error_code, fault_addr)?;
            println!("Page fault handled successfully");
        }
    }
    
    // Test 7: Permission checking
    println!("\n=== Test 7: Permission checking ===");
    let readonly_virtual = 0x5000;
    let readonly_physical = 0x6000;
    let readonly_flags = PageTableEntryFlags::PRESENT | PageTableEntryFlags::USER_ACCESSIBLE; // No WRITABLE flag
    
    memory_manager.create_page_table_entry(readonly_virtual, readonly_physical, readonly_flags)?;
    
    // Read should work
    let _ = memory_manager.translate_address_with_access(readonly_virtual, false, false, false)?;
    println!("Read access to read-only page: OK");
    
    // Write should fail
    match memory_manager.translate_address_with_access(readonly_virtual, true, false, false) {
        Ok(_) => println!("Unexpected: Write to read-only page succeeded"),
        Err(e) => println!("Expected: Write to read-only page failed: {}", e),
    }
    
    // Test 8: User vs supervisor access
    println!("\n=== Test 8: User vs supervisor access ===");
    let supervisor_virtual = 0x7000;
    let supervisor_physical = 0x8000;
    let supervisor_flags = PageTableEntryFlags::PRESENT | PageTableEntryFlags::WRITABLE; // No USER_ACCESSIBLE flag
    
    memory_manager.create_page_table_entry(supervisor_virtual, supervisor_physical, supervisor_flags)?;
    
    // Supervisor access should work
    let _ = memory_manager.translate_address_with_access(supervisor_virtual, false, false, false)?;
    println!("Supervisor access to supervisor page: OK");
    
    // User access should fail
    match memory_manager.translate_address_with_access(supervisor_virtual, false, true, false) {
        Ok(_) => println!("Unexpected: User access to supervisor page succeeded"),
        Err(e) => println!("Expected: User access to supervisor page failed: {}", e),
    }
    
    // Test 9: TLB invalidation
    println!("\n=== Test 9: TLB invalidation ===");
    memory_manager.invalidate_tlb_entry(virtual_addr);
    println!("Invalidated TLB entry for 0x{:x}", virtual_addr);
    
    // Access should still work (will repopulate TLB)
    let _ = memory_manager.translate_address_with_access(virtual_addr, false, false, false)?;
    println!("Access after TLB invalidation: OK");
    
    println!("\n=== All paging tests completed successfully! ===");
    
    Ok(())
}
