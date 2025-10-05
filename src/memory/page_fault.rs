use crate::Result;
use crate::memory::{
    PageFaultErrorCode, PAGE_SIZE_4K, 
    PageAllocator, PageSize, MemoryBacking, SharedPageAllocator
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Page fault handler trait
pub trait PageFaultHandler: Send + Sync {
    /// Handle a page fault
    fn handle_page_fault(
        &mut self,
        virtual_addr: u64,
        error_code: PageFaultErrorCode,
        cr2: u64,
    ) -> Result<()>;
}

/// Simple page fault handler that allocates pages on demand
#[derive(Debug)]
pub struct SimplePageFaultHandler {
    /// Physical memory allocator
    page_allocator: SharedPageAllocator,
    /// Track allocated virtual pages
    allocated_virtual_pages: HashMap<u64, u64>, // virtual_addr -> physical_addr
}

impl SimplePageFaultHandler {
    /// Create a new simple page fault handler
    pub fn new() -> Self {
        Self {
            page_allocator: Arc::new(Mutex::new(PageAllocator::new(0x1000000, 0x100000000))), // 16MB to 4GB
            allocated_virtual_pages: HashMap::new(),
        }
    }

    /// Create a new page fault handler with a custom page allocator
    pub fn with_allocator(page_allocator: SharedPageAllocator) -> Self {
        Self {
            page_allocator,
            allocated_virtual_pages: HashMap::new(),
        }
    }

    /// Allocate a new physical page
    fn allocate_physical_page(&mut self) -> Result<u64> {
        let mut allocator = self.page_allocator.lock().unwrap();
        allocator.allocate_page(PageSize::Size4K, MemoryBacking::GuestMemory)
    }

    /// Check if a virtual page is already allocated
    pub fn is_page_allocated(&self, virtual_addr: u64) -> bool {
        let page_addr = virtual_addr & !(PAGE_SIZE_4K - 1);
        self.allocated_virtual_pages.contains_key(&page_addr)
    }

    /// Get the physical address for a virtual page
    pub fn get_physical_address(&self, virtual_addr: u64) -> Option<u64> {
        let page_addr = virtual_addr & !(PAGE_SIZE_4K - 1);
        self.allocated_virtual_pages.get(&page_addr).copied()
    }

    /// Mark a virtual page as allocated with its physical address
    pub fn mark_page_allocated(&mut self, virtual_addr: u64, physical_addr: u64) {
        let page_addr = virtual_addr & !(PAGE_SIZE_4K - 1);
        self.allocated_virtual_pages.insert(page_addr, physical_addr);
    }

    /// Get a reference to the page allocator
    pub fn get_page_allocator(&self) -> &SharedPageAllocator {
        &self.page_allocator
    }
}

impl Default for SimplePageFaultHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl PageFaultHandler for SimplePageFaultHandler {
    fn handle_page_fault(
        &mut self,
        virtual_addr: u64,
        error_code: PageFaultErrorCode,
        cr2: u64,
    ) -> Result<()> {
        log::debug!(
            "Page fault at virtual address 0x{:x}, error code: 0x{:x}, CR2: 0x{:x}",
            virtual_addr,
            error_code.value,
            cr2
        );

        // Check if this is a valid page fault (page not present)
        if error_code.was_present() {
            // Page was present but access was denied
            if error_code.was_write() && !error_code.was_user() {
                log::error!("Write to read-only page at 0x{:x}", virtual_addr);
                return Err(crate::EmulatorError::Memory(format!(
                    "Write to read-only page at 0x{:x}",
                    virtual_addr
                )));
            }
            if error_code.was_user() {
                log::error!("User access to supervisor page at 0x{:x}", virtual_addr);
                return Err(crate::EmulatorError::Memory(format!(
                    "User access to supervisor page at 0x{:x}",
                    virtual_addr
                )));
            }
            if error_code.was_instruction_fetch() {
                log::error!("Execute from non-executable page at 0x{:x}", virtual_addr);
                return Err(crate::EmulatorError::Memory(format!(
                    "Execute from non-executable page at 0x{:x}",
                    virtual_addr
                )));
            }
        }

        // Allocate a new page if not already allocated
        if !self.is_page_allocated(virtual_addr) {
            let physical_addr = self.allocate_physical_page()?;
            self.mark_page_allocated(virtual_addr, physical_addr);
            log::info!(
                "Allocated new page for virtual address 0x{:x} -> physical address 0x{:x}",
                virtual_addr,
                physical_addr
            );
        }

        Ok(())
    }
}

/// Page fault manager that coordinates page fault handling
pub struct PageFaultManager {
    handler: Box<dyn PageFaultHandler>,
}

impl std::fmt::Debug for PageFaultManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PageFaultManager")
            .field("handler", &"<dyn PageFaultHandler>")
            .finish()
    }
}

impl PageFaultManager {
    /// Create a new page fault manager
    pub fn new(handler: Box<dyn PageFaultHandler>) -> Self {
        Self { handler }
    }

    /// Handle a page fault
    pub fn handle_page_fault(
        &mut self,
        virtual_addr: u64,
        error_code: PageFaultErrorCode,
        cr2: u64,
    ) -> Result<()> {
        self.handler.handle_page_fault(virtual_addr, error_code, cr2)
    }

    /// Get a reference to the page fault handler
    pub fn get_handler(&self) -> &dyn PageFaultHandler {
        self.handler.as_ref()
    }

    /// Get a mutable reference to the page fault handler
    pub fn get_handler_mut(&mut self) -> &mut dyn PageFaultHandler {
        self.handler.as_mut()
    }
}

/// Page fault statistics
#[derive(Debug, Default)]
pub struct PageFaultStats {
    pub total_faults: u64,
    pub page_not_present: u64,
    pub write_to_readonly: u64,
    pub user_access_denied: u64,
    pub execute_denied: u64,
    pub reserved_bit_violation: u64,
}

impl PageFaultStats {
    /// Record a page fault
    pub fn record_fault(&mut self, error_code: PageFaultErrorCode) {
        self.total_faults += 1;
        
        if !error_code.was_present() {
            self.page_not_present += 1;
        }
        if error_code.was_write() && !error_code.was_user() {
            self.write_to_readonly += 1;
        }
        if error_code.was_user() {
            self.user_access_denied += 1;
        }
        if error_code.was_instruction_fetch() {
            self.execute_denied += 1;
        }
        if error_code.was_reserved() {
            self.reserved_bit_violation += 1;
        }
    }

    /// Reset statistics
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_fault_error_code() {
        let error_code = PageFaultErrorCode::new(true, false, false, false, false);
        assert!(error_code.was_present());
        assert!(!error_code.was_write());
        assert!(!error_code.was_user());
    }

    #[test]
    fn test_simple_page_fault_handler() {
        let mut handler = SimplePageFaultHandler::new();
        let error_code = PageFaultErrorCode::new(false, false, false, false, false);
        
        // Should not error for a simple page not present fault
        assert!(handler.handle_page_fault(0x1000, error_code, 0x1000).is_ok());
        assert!(handler.is_page_allocated(0x1000));
    }
}
