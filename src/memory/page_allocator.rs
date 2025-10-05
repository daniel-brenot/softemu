use crate::Result;
use crate::memory::{PAGE_SIZE_4K, PAGE_SIZE_2M, PAGE_SIZE_1G};
use std::collections::{HashMap, VecDeque, BTreeMap};
use std::sync::{Arc, Mutex};

/// Page size enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PageSize {
    Size4K = 4096,
    Size2M = 2 * 1024 * 1024,
    Size1G = 1024 * 1024 * 1024,
}

impl PageSize {
    /// Get the size in bytes
    pub fn size(&self) -> u64 {
        match self {
            PageSize::Size4K => PAGE_SIZE_4K,
            PageSize::Size2M => PAGE_SIZE_2M,
            PageSize::Size1G => PAGE_SIZE_1G,
        }
    }

    /// Get the alignment requirement
    pub fn alignment(&self) -> u64 {
        self.size()
    }

    /// Check if an address is aligned for this page size
    pub fn is_aligned(&self, addr: u64) -> bool {
        addr & (self.alignment() - 1) == 0
    }

    /// Align an address to this page size
    pub fn align(&self, addr: u64) -> u64 {
        (addr + self.alignment() - 1) & !(self.alignment() - 1)
    }
}

/// Memory backing type for pages
#[derive(Debug, Clone)]
pub enum MemoryBacking {
    /// Regular memory backed by guest memory
    GuestMemory,
    /// Memory-mapped file
    MappedFile(String),
    /// Device memory (MMIO)
    DeviceMemory,
    /// Zero-filled memory (lazy allocation)
    ZeroFilled,
}

/// Memory region information
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub start: u64,
    pub size: u64,
    pub backing: MemoryBacking,
    pub permissions: u64, // PageTableEntryFlags bits
    pub allocated: bool,
}

impl MemoryRegion {
    /// Create a new memory region
    pub fn new(start: u64, size: u64, backing: MemoryBacking, permissions: u64) -> Self {
        Self {
            start,
            size,
            backing,
            permissions,
            allocated: false,
        }
    }

    /// Check if an address is within this region
    pub fn contains(&self, addr: u64) -> bool {
        addr >= self.start && addr < self.start + self.size
    }

    /// Get the offset of an address within this region
    pub fn offset(&self, addr: u64) -> Option<u64> {
        if self.contains(addr) {
            Some(addr - self.start)
        } else {
            None
        }
    }
}

/// Page allocation information
#[derive(Debug, Clone)]
pub struct PageInfo {
    pub addr: u64,
    pub size: PageSize,
    pub backing: MemoryBacking,
    pub allocated: bool,
    pub ref_count: u32,
}

/// Advanced page allocator with support for different page sizes and memory regions
#[derive(Debug)]
pub struct PageAllocator {
    /// Free pages by size
    free_pages: HashMap<PageSize, VecDeque<u64>>,
    /// Allocated pages
    allocated_pages: HashMap<u64, PageInfo>,
    /// Memory regions
    memory_regions: BTreeMap<u64, MemoryRegion>,
    /// Next address for new allocations
    next_addr: u64,
    /// Maximum address for allocations
    max_addr: u64,
    /// Statistics
    stats: AllocationStats,
}

/// Allocation statistics
#[derive(Debug, Default, Clone)]
pub struct AllocationStats {
    pub total_allocated: u64,
    pub total_freed: u64,
    pub current_allocated: u64,
    pub allocation_requests: u64,
    pub free_requests: u64,
    pub fragmentation_count: u64,
}

impl PageAllocator {
    /// Create a new page allocator
    pub fn new(start_addr: u64, max_addr: u64) -> Self {
        let mut allocator = Self {
            free_pages: HashMap::new(),
            allocated_pages: HashMap::new(),
            memory_regions: BTreeMap::new(),
            next_addr: start_addr,
            max_addr,
            stats: AllocationStats::default(),
        };

        // Initialize free page lists
        allocator.free_pages.insert(PageSize::Size4K, VecDeque::new());
        allocator.free_pages.insert(PageSize::Size2M, VecDeque::new());
        allocator.free_pages.insert(PageSize::Size1G, VecDeque::new());

        allocator
    }

    /// Allocate a page of the specified size
    pub fn allocate_page(&mut self, size: PageSize, backing: MemoryBacking) -> Result<u64> {
        self.stats.allocation_requests += 1;

        // Try to reuse a free page first
        if let Some(free_pages) = self.free_pages.get_mut(&size) {
            if let Some(addr) = free_pages.pop_front() {
                // Verify the page is still free
                if !self.allocated_pages.contains_key(&addr) {
                    let page_info = PageInfo {
                        addr,
                        size,
                        backing: backing.clone(),
                        allocated: true,
                        ref_count: 1,
                    };
                    self.allocated_pages.insert(addr, page_info);
                    self.stats.total_allocated += 1;
                    self.stats.current_allocated += 1;
                    return Ok(addr);
                }
            }
        }

        // Allocate a new page
        let addr = self.allocate_new_page(size)?;
        let page_info = PageInfo {
            addr,
            size,
            backing,
            allocated: true,
            ref_count: 1,
        };
        self.allocated_pages.insert(addr, page_info);
        self.stats.total_allocated += 1;
        self.stats.current_allocated += 1;

        Ok(addr)
    }

    /// Allocate a new page from the address space
    fn allocate_new_page(&mut self, size: PageSize) -> Result<u64> {
        // Align the next address to the page size
        let aligned_addr = size.align(self.next_addr);
        
        // Check if we have enough space
        if aligned_addr + size.size() > self.max_addr {
            return Err(crate::EmulatorError::Memory(format!(
                "Out of memory: cannot allocate {} byte page at 0x{:x} (max: 0x{:x})",
                size.size(),
                aligned_addr,
                self.max_addr
            )));
        }

        // Check for conflicts with existing regions
        if self.has_region_conflict(aligned_addr, size.size()) {
            // Try to find a gap
            if let Some(gap_addr) = self.find_free_gap(size.size()) {
                self.next_addr = gap_addr + size.size();
                return Ok(gap_addr);
            } else {
                return Err(crate::EmulatorError::Memory(format!(
                    "No free space for {} byte page allocation",
                    size.size()
                )));
            }
        }

        let addr = aligned_addr;
        self.next_addr = addr + size.size();
        Ok(addr)
    }

    /// Check if there's a conflict with existing memory regions
    fn has_region_conflict(&self, addr: u64, size: u64) -> bool {
        let end_addr = addr + size;
        
        // Check for overlapping regions
        for (_, region) in self.memory_regions.range(..=end_addr) {
            if region.start < end_addr && region.start + region.size > addr {
                return true;
            }
        }
        
        false
    }

    /// Find a free gap in the address space
    fn find_free_gap(&self, size: u64) -> Option<u64> {
        let mut last_end = 0;
        
        for (_, region) in &self.memory_regions {
            if region.start - last_end >= size {
                return Some(last_end);
            }
            last_end = region.start + region.size;
        }
        
        // Check if there's space after the last region
        if self.max_addr - last_end >= size {
            Some(last_end)
        } else {
            None
        }
    }

    /// Deallocate a page
    pub fn deallocate_page(&mut self, addr: u64) -> Result<()> {
        self.stats.free_requests += 1;

        if let Some(page_info) = self.allocated_pages.remove(&addr) {
            if page_info.ref_count > 1 {
                // Page is still referenced, don't actually free it
                let mut updated_info = page_info;
                updated_info.ref_count -= 1;
                self.allocated_pages.insert(addr, updated_info);
                return Ok(());
            }

            // Add to free list for reuse
            if let Some(free_pages) = self.free_pages.get_mut(&page_info.size) {
                free_pages.push_back(addr);
            }

            self.stats.total_freed += 1;
            self.stats.current_allocated -= 1;
            Ok(())
        } else {
            Err(crate::EmulatorError::Memory(format!(
                "Attempted to free unallocated page at 0x{:x}",
                addr
            )))
        }
    }

    /// Increment reference count for a page
    pub fn increment_ref_count(&mut self, addr: u64) -> Result<()> {
        if let Some(page_info) = self.allocated_pages.get_mut(&addr) {
            page_info.ref_count += 1;
            Ok(())
        } else {
            Err(crate::EmulatorError::Memory(format!(
                "Attempted to increment ref count for unallocated page at 0x{:x}",
                addr
            )))
        }
    }

    /// Check if a page is allocated
    pub fn is_allocated(&self, addr: u64) -> bool {
        self.allocated_pages.contains_key(&addr)
    }

    /// Get page information
    pub fn get_page_info(&self, addr: u64) -> Option<&PageInfo> {
        self.allocated_pages.get(&addr)
    }

    /// Add a memory region
    pub fn add_memory_region(&mut self, region: MemoryRegion) -> Result<()> {
        // Check for conflicts
        if self.has_region_conflict(region.start, region.size) {
            return Err(crate::EmulatorError::Memory(format!(
                "Memory region at 0x{:x} conflicts with existing regions",
                region.start
            )));
        }

        self.memory_regions.insert(region.start, region);
        Ok(())
    }

    /// Remove a memory region
    pub fn remove_memory_region(&mut self, start_addr: u64) -> Result<()> {
        if self.memory_regions.remove(&start_addr).is_some() {
            Ok(())
        } else {
            Err(crate::EmulatorError::Memory(format!(
                "Memory region at 0x{:x} not found",
                start_addr
            )))
        }
    }

    /// Get memory region containing an address
    pub fn get_memory_region(&self, addr: u64) -> Option<&MemoryRegion> {
        for (_, region) in &self.memory_regions {
            if region.contains(addr) {
                return Some(region);
            }
        }
        None
    }

    /// Get allocation statistics
    pub fn get_stats(&self) -> &AllocationStats {
        &self.stats
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.stats = AllocationStats::default();
    }

    /// Get memory usage information
    pub fn get_memory_usage(&self) -> MemoryUsage {
        let mut total_allocated = 0;
        let mut total_free = 0;
        let mut page_counts = HashMap::new();

        // Count allocated pages
        for page_info in self.allocated_pages.values() {
            total_allocated += page_info.size.size();
            *page_counts.entry(page_info.size).or_insert(0) += 1;
        }

        // Count free pages
        for (size, free_pages) in &self.free_pages {
            total_free += size.size() * free_pages.len() as u64;
            *page_counts.entry(*size).or_insert(0) += free_pages.len();
        }

        MemoryUsage {
            total_allocated,
            total_free,
            page_counts,
            region_count: self.memory_regions.len(),
        }
    }

    /// Defragment memory by consolidating free pages
    pub fn defragment(&mut self) -> Result<()> {
        // This is a simplified defragmentation
        // In a real implementation, this would be more sophisticated
        self.stats.fragmentation_count += 1;
        Ok(())
    }
}

/// Memory usage information
#[derive(Debug)]
pub struct MemoryUsage {
    pub total_allocated: u64,
    pub total_free: u64,
    pub page_counts: HashMap<PageSize, usize>,
    pub region_count: usize,
}

/// Thread-safe page allocator wrapper
pub type SharedPageAllocator = Arc<Mutex<PageAllocator>>;

impl Default for PageAllocator {
    fn default() -> Self {
        Self::new(0x1000000, 0x100000000) // 16MB to 4GB
    }
}
