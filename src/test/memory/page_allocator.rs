use super::*;

#[test]
fn test_page_allocator_basic() {
    let mut allocator = PageAllocator::new(0x1000000, 0x2000000);
    
    // Allocate a 4KB page
    let addr1 = allocator.allocate_page(PageSize::Size4K, MemoryBacking::GuestMemory).unwrap();
    assert_eq!(addr1, 0x1000000);
    assert!(allocator.is_allocated(addr1));
    
    // Allocate another 4KB page
    let addr2 = allocator.allocate_page(PageSize::Size4K, MemoryBacking::GuestMemory).unwrap();
    assert_eq!(addr2, 0x1001000);
    
    // Deallocate first page
    allocator.deallocate_page(addr1).unwrap();
    assert!(!allocator.is_allocated(addr1));
    
    // Allocate again - should reuse the freed page
    let addr3 = allocator.allocate_page(PageSize::Size4K, MemoryBacking::GuestMemory).unwrap();
    assert_eq!(addr3, addr1);
}

#[test]
fn test_page_allocator_different_sizes() {
    let mut allocator = PageAllocator::new(0x1000000, 0x10000000);
    
    // Allocate different page sizes
    let addr_4k = allocator.allocate_page(PageSize::Size4K, MemoryBacking::GuestMemory).unwrap();
    let addr_2m = allocator.allocate_page(PageSize::Size2M, MemoryBacking::GuestMemory).unwrap();
    
    assert_eq!(addr_4k, 0x1000000);
    assert_eq!(addr_2m, 0x1001000); // Aligned to 2MB boundary
    
    // Check alignment
    assert!(PageSize::Size4K.is_aligned(addr_4k));
    assert!(PageSize::Size2M.is_aligned(addr_2m));
}

#[test]
fn test_memory_regions() {
    let mut allocator = PageAllocator::new(0x1000000, 0x10000000);
    
    let region = MemoryRegion::new(
        0x2000000,
        0x100000,
        MemoryBacking::MappedFile("test.bin".to_string()),
        0x7, // Present, Writable, User accessible
    );
    
    allocator.add_memory_region(region.clone()).unwrap();
    
    // Check region lookup
    let found_region = allocator.get_memory_region(0x2001000).unwrap();
    assert_eq!(found_region.start, region.start);
    assert_eq!(found_region.size, region.size);
    
    // Check offset calculation
    assert_eq!(found_region.offset(0x2001000), Some(0x1000));
    assert_eq!(found_region.offset(0x3000000), None);
}

#[test]
fn test_ref_counting() {
    let mut allocator = PageAllocator::new(0x1000000, 0x2000000);
    
    let addr = allocator.allocate_page(PageSize::Size4K, MemoryBacking::GuestMemory).unwrap();
    
    // Increment ref count
    allocator.increment_ref_count(addr).unwrap();
    
    // Deallocate - should not actually free due to ref count
    allocator.deallocate_page(addr).unwrap();
    assert!(allocator.is_allocated(addr));
    
    // Deallocate again - should actually free
    allocator.deallocate_page(addr).unwrap();
    assert!(!allocator.is_allocated(addr));
}