use crate::Result;
use bitflags::bitflags;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// Page size constants for x86_64
pub const PAGE_SIZE_4K: u64 = 4096;
pub const PAGE_SIZE_2M: u64 = 2 * 1024 * 1024;
pub const PAGE_SIZE_1G: u64 = 1024 * 1024 * 1024;

// Page table entry flags for x86_64
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PageTableEntryFlags: u64 {
        /// Present bit - page is in memory
        const PRESENT = 1 << 0;
        /// Read/Write bit - 0 = read-only, 1 = read/write
        const WRITABLE = 1 << 1;
        /// User/Supervisor bit - 0 = supervisor only, 1 = user accessible
        const USER_ACCESSIBLE = 1 << 2;
        /// Page Write Through - 0 = write-back, 1 = write-through
        const PAGE_WRITE_THROUGH = 1 << 3;
        /// Page Cache Disable - 0 = cacheable, 1 = not cacheable
        const PAGE_CACHE_DISABLE = 1 << 4;
        /// Accessed bit - set by CPU when page is accessed
        const ACCESSED = 1 << 5;
        /// Dirty bit - set by CPU when page is written to
        const DIRTY = 1 << 6;
        /// Page Size bit - 0 = 4KB page, 1 = 2MB/1GB page (depending on level)
        const PAGE_SIZE = 1 << 7;
        /// Global bit - 0 = not global, 1 = global (not flushed from TLB)
        const GLOBAL = 1 << 8;
        /// Execute Disable bit - 0 = executable, 1 = not executable
        const EXECUTE_DISABLE = 1 << 63;
    }
}

/// Page table entry for x86_64
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageTableEntry {
    pub value: u64,
}

impl PageTableEntry {
    /// Create a new page table entry
    pub fn new(physical_addr: u64, flags: PageTableEntryFlags) -> Self {
        // Clear the lower 12 bits and set the physical address
        let addr = (physical_addr & !0xFFF) | flags.bits();
        Self { value: addr }
    }

    /// Check if the entry is present
    pub fn is_present(&self) -> bool {
        self.value & PageTableEntryFlags::PRESENT.bits() != 0
    }

    /// Check if the page is writable
    pub fn is_writable(&self) -> bool {
        self.value & PageTableEntryFlags::WRITABLE.bits() != 0
    }

    /// Check if the page is user accessible
    pub fn is_user_accessible(&self) -> bool {
        self.value & PageTableEntryFlags::USER_ACCESSIBLE.bits() != 0
    }

    /// Check if the page is executable
    pub fn is_executable(&self) -> bool {
        self.value & PageTableEntryFlags::EXECUTE_DISABLE.bits() == 0
    }

    /// Get the physical address from the entry
    pub fn get_physical_address(&self) -> u64 {
        self.value & !0xFFF
    }

    /// Get the flags from the entry
    pub fn get_flags(&self) -> PageTableEntryFlags {
        PageTableEntryFlags::from_bits_truncate(self.value)
    }

    /// Set the accessed bit
    pub fn set_accessed(&mut self) {
        self.value |= PageTableEntryFlags::ACCESSED.bits();
    }

    /// Set the dirty bit
    pub fn set_dirty(&mut self) {
        self.value |= PageTableEntryFlags::DIRTY.bits();
    }

    /// Check if the accessed bit is set
    pub fn is_accessed(&self) -> bool {
        self.value & PageTableEntryFlags::ACCESSED.bits() != 0
    }

    /// Check if the dirty bit is set
    pub fn is_dirty(&self) -> bool {
        self.value & PageTableEntryFlags::DIRTY.bits() != 0
    }
}

/// Page table structure for x86_64
#[derive(Debug)]
pub struct PageTable {
    entries: Vec<PageTableEntry>,
    level: u8, // 0 = PML4, 1 = PDPT, 2 = PD, 3 = PT
}

impl PageTable {
    /// Create a new page table
    pub fn new(level: u8) -> Self {
        Self {
            entries: vec![PageTableEntry { value: 0 }; 512], // 512 entries per table
            level,
        }
    }

    /// Get an entry from the page table
    pub fn get_entry(&self, index: usize) -> PageTableEntry {
        if index < 512 {
            self.entries[index]
        } else {
            PageTableEntry { value: 0 }
        }
    }

    /// Set an entry in the page table
    pub fn set_entry(&mut self, index: usize, entry: PageTableEntry) {
        if index < 512 {
            self.entries[index] = entry;
        }
    }

    /// Get the level of this page table
    pub fn get_level(&self) -> u8 {
        self.level
    }
}

/// Page fault error code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageFaultErrorCode {
    pub value: u64,
}

impl PageFaultErrorCode {
    /// Create a new page fault error code
    pub fn new(
        present: bool,
        write: bool,
        user: bool,
        reserved: bool,
        instruction_fetch: bool,
    ) -> Self {
        let mut value = 0u64;
        if present {
            value |= 1 << 0;
        }
        if write {
            value |= 1 << 1;
        }
        if user {
            value |= 1 << 2;
        }
        if reserved {
            value |= 1 << 3;
        }
        if instruction_fetch {
            value |= 1 << 4;
        }
        Self { value }
    }

    /// Check if the page was present
    pub fn was_present(&self) -> bool {
        self.value & (1 << 0) != 0
    }

    /// Check if it was a write access
    pub fn was_write(&self) -> bool {
        self.value & (1 << 1) != 0
    }

    /// Check if it was a user access
    pub fn was_user(&self) -> bool {
        self.value & (1 << 2) != 0
    }

    /// Check if there was a reserved bit violation
    pub fn was_reserved(&self) -> bool {
        self.value & (1 << 3) != 0
    }

    /// Check if it was an instruction fetch
    pub fn was_instruction_fetch(&self) -> bool {
        self.value & (1 << 4) != 0
    }
}

/// Translation Lookaside Buffer (TLB) entry
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TlbEntry {
    pub virtual_addr: u64,
    pub physical_addr: u64,
    pub flags: PageTableEntryFlags,
    pub asid: u16, // Address Space Identifier
}

/// TLB implementation
#[derive(Debug)]
pub struct TranslationLookasideBuffer {
    entries: HashMap<u64, TlbEntry>,
    max_entries: usize,
}

impl TranslationLookasideBuffer {
    /// Create a new TLB
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: HashMap::new(),
            max_entries,
        }
    }

    /// Look up a virtual address in the TLB
    pub fn lookup(&self, virtual_addr: u64, asid: u16) -> Option<TlbEntry> {
        let page_addr = virtual_addr & !0xFFF; // Align to page boundary
        self.entries.get(&page_addr).and_then(|entry| {
            if entry.asid == asid || entry.flags.contains(PageTableEntryFlags::GLOBAL) {
                Some(*entry)
            } else {
                None
            }
        })
    }

    /// Insert an entry into the TLB
    pub fn insert(&mut self, entry: TlbEntry) {
        let page_addr = entry.virtual_addr & !0xFFF;
        
        // If TLB is full, remove a random entry
        if self.entries.len() >= self.max_entries {
            if let Some(&key) = self.entries.keys().next() {
                self.entries.remove(&key);
            }
        }
        
        self.entries.insert(page_addr, entry);
    }

    /// Invalidate a specific entry
    pub fn invalidate(&mut self, virtual_addr: u64, asid: u16) {
        let page_addr = virtual_addr & !0xFFF;
        if let Some(entry) = self.entries.get(&page_addr) {
            if entry.asid == asid {
                self.entries.remove(&page_addr);
            }
        }
    }

    /// Invalidate all entries for an address space
    pub fn invalidate_asid(&mut self, asid: u16) {
        self.entries.retain(|_, entry| entry.asid != asid);
    }

    /// Invalidate all entries (except global ones)
    pub fn invalidate_all(&mut self) {
        self.entries.retain(|_, entry| entry.flags.contains(PageTableEntryFlags::GLOBAL));
    }

    /// Clear the entire TLB
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

/// Page table manager for x86_64
#[derive(Debug)]
pub struct PageTableManager {
    /// Current page table root (CR3 value)
    pub cr3: u64,
    /// Page tables indexed by physical address
    page_tables: HashMap<u64, Arc<RwLock<PageTable>>>,
    /// TLB for address translation caching
    tlb: Arc<RwLock<TranslationLookasideBuffer>>,
    /// Current Address Space Identifier
    current_asid: u16,
}

impl PageTableManager {
    /// Create a new page table manager
    pub fn new() -> Self {
        Self {
            cr3: 0,
            page_tables: HashMap::new(),
            tlb: Arc::new(RwLock::new(TranslationLookasideBuffer::new(1024))),
            current_asid: 0,
        }
    }

    /// Set the CR3 register (page table root)
    pub fn set_cr3(&mut self, cr3: u64) {
        self.cr3 = cr3;
        // Invalidate TLB when changing page tables
        if let Ok(mut tlb) = self.tlb.write() {
            tlb.invalidate_all();
        }
    }

    /// Get or create a page table at the given physical address
    fn get_or_create_page_table(&mut self, physical_addr: u64, level: u8) -> Arc<RwLock<PageTable>> {
        if let Some(table) = self.page_tables.get(&physical_addr) {
            table.clone()
        } else {
            let table = Arc::new(RwLock::new(PageTable::new(level)));
            self.page_tables.insert(physical_addr, table.clone());
            table
        }
    }

    /// Walk the page table hierarchy to translate a virtual address
    pub fn translate_address(
        &mut self,
        virtual_addr: u64,
        is_write: bool,
        is_user: bool,
        is_instruction_fetch: bool,
    ) -> Result<u64> {
        // Check TLB first
        if let Ok(tlb) = self.tlb.read() {
            if let Some(entry) = tlb.lookup(virtual_addr, self.current_asid) {
                // Check permissions
                if is_write && !entry.flags.contains(PageTableEntryFlags::WRITABLE) {
                    return Err(crate::EmulatorError::Memory(format!(
                        "Page fault: write to read-only page at 0x{:x}",
                        virtual_addr
                    )));
                }
                if is_user && !entry.flags.contains(PageTableEntryFlags::USER_ACCESSIBLE) {
                    return Err(crate::EmulatorError::Memory(format!(
                        "Page fault: user access to supervisor page at 0x{:x}",
                        virtual_addr
                    )));
                }
                if is_instruction_fetch && !entry.flags.contains(PageTableEntryFlags::EXECUTE_DISABLE) {
                    return Ok(entry.physical_addr + (virtual_addr & 0xFFF));
                }
                return Ok(entry.physical_addr + (virtual_addr & 0xFFF));
            }
        }

        // TLB miss - walk page tables
        self.walk_page_tables(virtual_addr, is_write, is_user, is_instruction_fetch)
    }

    /// Walk the page table hierarchy
    fn walk_page_tables(
        &mut self,
        virtual_addr: u64,
        is_write: bool,
        is_user: bool,
        is_instruction_fetch: bool,
    ) -> Result<u64> {
        if self.cr3 == 0 {
            // Identity mapping when CR3 is 0
            return Ok(virtual_addr);
        }

        // Extract page table indices
        let pml4_index = ((virtual_addr >> 39) & 0x1FF) as usize;
        let pdpt_index = ((virtual_addr >> 30) & 0x1FF) as usize;
        let pd_index = ((virtual_addr >> 21) & 0x1FF) as usize;
        let pt_index = ((virtual_addr >> 12) & 0x1FF) as usize;

        // Walk PML4 (Level 4)
        let pml4_table = self.get_or_create_page_table(self.cr3, 0);
        let pml4_entry = {
            let table = pml4_table.read().unwrap();
            table.get_entry(pml4_index)
        };

        if !pml4_entry.is_present() {
            return Err(crate::EmulatorError::Memory(format!(
                "Page fault: PML4 entry not present for address 0x{:x}",
                virtual_addr
            )));
        }

        let pdpt_addr = pml4_entry.get_physical_address();

        // Walk PDPT (Level 3)
        let pdpt_table = self.get_or_create_page_table(pdpt_addr, 1);
        let pdpt_entry = {
            let table = pdpt_table.read().unwrap();
            table.get_entry(pdpt_index)
        };

        if !pdpt_entry.is_present() {
            return Err(crate::EmulatorError::Memory(format!(
                "Page fault: PDPT entry not present for address 0x{:x}",
                virtual_addr
            )));
        }

        // Check for 1GB page
        if pdpt_entry.get_flags().contains(PageTableEntryFlags::PAGE_SIZE) {
            let physical_addr = pdpt_entry.get_physical_address() + (virtual_addr & 0x3FFFFFFF);
            self.update_tlb(virtual_addr, physical_addr, pdpt_entry.get_flags());
            return Ok(physical_addr);
        }

        let pd_addr = pdpt_entry.get_physical_address();

        // Walk PD (Level 2)
        let pd_table = self.get_or_create_page_table(pd_addr, 2);
        let pd_entry = {
            let table = pd_table.read().unwrap();
            table.get_entry(pd_index)
        };

        if !pd_entry.is_present() {
            return Err(crate::EmulatorError::Memory(format!(
                "Page fault: PD entry not present for address 0x{:x}",
                virtual_addr
            )));
        }

        // Check for 2MB page
        if pd_entry.get_flags().contains(PageTableEntryFlags::PAGE_SIZE) {
            let physical_addr = pd_entry.get_physical_address() + (virtual_addr & 0x1FFFFF);
            self.update_tlb(virtual_addr, physical_addr, pd_entry.get_flags());
            return Ok(physical_addr);
        }

        let pt_addr = pd_entry.get_physical_address();

        // Walk PT (Level 1)
        let pt_table = self.get_or_create_page_table(pt_addr, 3);
        let pt_entry = {
            let table = pt_table.read().unwrap();
            table.get_entry(pt_index)
        };

        if !pt_entry.is_present() {
            return Err(crate::EmulatorError::Memory(format!(
                "Page fault: PT entry not present for address 0x{:x}",
                virtual_addr
            )));
        }

        // Check permissions
        if is_write && !pt_entry.is_writable() {
            return Err(crate::EmulatorError::Memory(format!(
                "Page fault: write to read-only page at 0x{:x}",
                virtual_addr
            )));
        }
        if is_user && !pt_entry.is_user_accessible() {
            return Err(crate::EmulatorError::Memory(format!(
                "Page fault: user access to supervisor page at 0x{:x}",
                virtual_addr
            )));
        }
        if is_instruction_fetch && !pt_entry.is_executable() {
            return Err(crate::EmulatorError::Memory(format!(
                "Page fault: execute from non-executable page at 0x{:x}",
                virtual_addr
            )));
        }

        let physical_addr = pt_entry.get_physical_address() + (virtual_addr & 0xFFF);
        self.update_tlb(virtual_addr, physical_addr, pt_entry.get_flags());
        Ok(physical_addr)
    }

    /// Update TLB with a new translation
    fn update_tlb(&self, virtual_addr: u64, physical_addr: u64, flags: PageTableEntryFlags) {
        if let Ok(mut tlb) = self.tlb.write() {
            let entry = TlbEntry {
                virtual_addr,
                physical_addr,
                flags,
                asid: self.current_asid,
            };
            tlb.insert(entry);
        }
    }

    /// Create a page table entry
    pub fn create_page_table_entry(
        &mut self,
        virtual_addr: u64,
        physical_addr: u64,
        flags: PageTableEntryFlags,
    ) -> Result<()> {
        if self.cr3 == 0 {
            return Err(crate::EmulatorError::Memory(
                "Cannot create page table entry: CR3 not set".to_string(),
            ));
        }

        // Extract page table indices
        let pml4_index = ((virtual_addr >> 39) & 0x1FF) as usize;
        let pdpt_index = ((virtual_addr >> 30) & 0x1FF) as usize;
        let pd_index = ((virtual_addr >> 21) & 0x1FF) as usize;
        let pt_index = ((virtual_addr >> 12) & 0x1FF) as usize;

        // Create or get PML4 table
        let pml4_table = self.get_or_create_page_table(self.cr3, 0);
        let mut pml4_entry = {
            let table = pml4_table.read().unwrap();
            table.get_entry(pml4_index)
        };

        if !pml4_entry.is_present() {
            // Create PDPT table
            let pdpt_addr = self.allocate_page_table()?;
            pml4_entry = PageTableEntry::new(
                pdpt_addr,
                PageTableEntryFlags::PRESENT | PageTableEntryFlags::WRITABLE | PageTableEntryFlags::USER_ACCESSIBLE,
            );
            {
                let mut table = pml4_table.write().unwrap();
                table.set_entry(pml4_index, pml4_entry);
            }
        }

        let pdpt_addr = pml4_entry.get_physical_address();

        // Create or get PDPT table
        let pdpt_table = self.get_or_create_page_table(pdpt_addr, 1);
        let mut pdpt_entry = {
            let table = pdpt_table.read().unwrap();
            table.get_entry(pdpt_index)
        };

        if !pdpt_entry.is_present() {
            // Create PD table
            let pd_addr = self.allocate_page_table()?;
            pdpt_entry = PageTableEntry::new(
                pd_addr,
                PageTableEntryFlags::PRESENT | PageTableEntryFlags::WRITABLE | PageTableEntryFlags::USER_ACCESSIBLE,
            );
            {
                let mut table = pdpt_table.write().unwrap();
                table.set_entry(pdpt_index, pdpt_entry);
            }
        }

        let pd_addr = pdpt_entry.get_physical_address();

        // Create or get PD table
        let pd_table = self.get_or_create_page_table(pd_addr, 2);
        let mut pd_entry = {
            let table = pd_table.read().unwrap();
            table.get_entry(pd_index)
        };

        if !pd_entry.is_present() {
            // Create PT table
            let pt_addr = self.allocate_page_table()?;
            pd_entry = PageTableEntry::new(
                pt_addr,
                PageTableEntryFlags::PRESENT | PageTableEntryFlags::WRITABLE | PageTableEntryFlags::USER_ACCESSIBLE,
            );
            {
                let mut table = pd_table.write().unwrap();
                table.set_entry(pd_index, pd_entry);
            }
        }

        let pt_addr = pd_entry.get_physical_address();

        // Create or get PT table and set the final entry
        let pt_table = self.get_or_create_page_table(pt_addr, 3);
        let pt_entry = PageTableEntry::new(physical_addr, flags);
        {
            let mut table = pt_table.write().unwrap();
            table.set_entry(pt_index, pt_entry);
        }

        // Invalidate TLB entry
        if let Ok(mut tlb) = self.tlb.write() {
            tlb.invalidate(virtual_addr, self.current_asid);
        }

        Ok(())
    }

    /// Allocate a new page table (simplified - in real implementation this would use a page allocator)
    fn allocate_page_table(&self) -> Result<u64> {
        // This is a simplified implementation
        // In a real system, this would use a proper page allocator
        static mut NEXT_PAGE_TABLE_ADDR: u64 = 0x100000; // Start at 1MB
        unsafe {
            let addr = NEXT_PAGE_TABLE_ADDR;
            NEXT_PAGE_TABLE_ADDR += PAGE_SIZE_4K;
            Ok(addr)
        }
    }

    /// Invalidate TLB entry
    pub fn invalidate_tlb_entry(&self, virtual_addr: u64) {
        if let Ok(mut tlb) = self.tlb.write() {
            tlb.invalidate(virtual_addr, self.current_asid);
        }
    }

    /// Invalidate all TLB entries
    pub fn invalidate_tlb(&self) {
        if let Ok(mut tlb) = self.tlb.write() {
            tlb.invalidate_all();
        }
    }

    /// Set the current Address Space Identifier
    pub fn set_asid(&mut self, asid: u16) {
        self.current_asid = asid;
    }

    /// Get the current Address Space Identifier
    pub fn get_asid(&self) -> u16 {
        self.current_asid
    }
}

impl Default for PageTableManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::{GuestMemory, MmioManager, MemoryManager};

    #[test]
    fn test_page_table_entry() {
        let entry = PageTableEntry::new(
            0x1000,
            PageTableEntryFlags::PRESENT | PageTableEntryFlags::WRITABLE,
        );
        assert!(entry.is_present());
        assert!(entry.is_writable());
        assert!(!entry.is_user_accessible());
        assert_eq!(entry.get_physical_address(), 0x1000);
    }

    #[test]
    fn test_tlb() {
        let mut tlb = TranslationLookasideBuffer::new(10);
        let entry = TlbEntry {
            virtual_addr: 0x1000,
            physical_addr: 0x2000,
            flags: PageTableEntryFlags::PRESENT,
            asid: 0,
        };
        tlb.insert(entry);
        assert_eq!(tlb.lookup(0x1000, 0), Some(entry));
        assert_eq!(tlb.lookup(0x1000, 1), None);
    }

    #[test]
    fn test_paging_basic_functionality() {
        // Create guest memory
        let guest_memory = GuestMemory::new(1024 * 1024 * 1024).unwrap(); // 1GB
        let mmio_manager = MmioManager::new();
        let mmio_space_size = 0x100000; // 1MB MMIO space
        
        // Create memory manager
        let mut memory_manager = MemoryManager::new(guest_memory, mmio_manager, mmio_space_size);
        
        // Test 1: Identity mapping when paging is disabled
        let test_addr = 0x1000;
        let translated = memory_manager.translate_address(test_addr).unwrap();
        assert_eq!(test_addr, translated);
        
        // Test 2: Enable paging and set up page tables
        memory_manager.set_paging_enabled(true);
        
        // Set CR3 to a page table root
        let cr3 = 0x100000; // 1MB
        memory_manager.set_cr3(cr3);
        
        // Test 3: Create a page table entry
        let virtual_addr = 0x2000;
        let physical_addr = 0x3000;
        let flags = PageTableEntryFlags::PRESENT | PageTableEntryFlags::WRITABLE | PageTableEntryFlags::USER_ACCESSIBLE;
        
        memory_manager.create_page_table_entry(virtual_addr, physical_addr, flags).unwrap();
        
        // Test 4: Address translation with paging enabled
        let translated = memory_manager.translate_address_with_access(virtual_addr, false, false, false).unwrap();
        assert_eq!(physical_addr, translated);
        
        // Test 5: TLB functionality
        // First access should populate TLB
        let _ = memory_manager.translate_address_with_access(virtual_addr, false, false, false).unwrap();
        
        // Second access should use TLB
        let _ = memory_manager.translate_address_with_access(virtual_addr, false, false, false).unwrap();
        
        // Test 6: Page fault handling
        let fault_addr = 0x4000;
        let error_code = PageFaultErrorCode::new(false, false, false, false, false); // Page not present
        
        // This should trigger a page fault
        match memory_manager.translate_address_with_access(fault_addr, false, false, false) {
            Ok(_) => panic!("Expected page fault"),
            Err(_) => {
                // Handle the page fault
                memory_manager.handle_page_fault(fault_addr, error_code, fault_addr).unwrap();
            }
        }
        
        // Test 7: Permission checking
        let readonly_virtual = 0x5000;
        let readonly_physical = 0x6000;
        let readonly_flags = PageTableEntryFlags::PRESENT | PageTableEntryFlags::USER_ACCESSIBLE; // No WRITABLE flag
        
        memory_manager.create_page_table_entry(readonly_virtual, readonly_physical, readonly_flags).unwrap();
        
        // Read should work
        let _ = memory_manager.translate_address_with_access(readonly_virtual, false, false, false).unwrap();
        
        // Write should fail
        assert!(memory_manager.translate_address_with_access(readonly_virtual, true, false, false).is_err());
        
        // Test 8: User vs supervisor access
        let supervisor_virtual = 0x7000;
        let supervisor_physical = 0x8000;
        let supervisor_flags = PageTableEntryFlags::PRESENT | PageTableEntryFlags::WRITABLE; // No USER_ACCESSIBLE flag
        
        memory_manager.create_page_table_entry(supervisor_virtual, supervisor_physical, supervisor_flags).unwrap();
        
        // Supervisor access should work
        let _ = memory_manager.translate_address_with_access(supervisor_virtual, false, false, false).unwrap();
        
        // User access should fail
        assert!(memory_manager.translate_address_with_access(supervisor_virtual, false, true, false).is_err());
        
        // Test 9: TLB invalidation
        memory_manager.invalidate_tlb_entry(virtual_addr);
        
        // Access should still work (will repopulate TLB)
        let _ = memory_manager.translate_address_with_access(virtual_addr, false, false, false).unwrap();
    }
}
