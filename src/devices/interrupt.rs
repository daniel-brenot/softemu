use crate::Result;
use std::collections::HashMap;
use std::sync::Arc;

/// Interrupt controller for managing interrupts
pub struct InterruptController {
    interrupt_handlers: HashMap<u8, Arc<dyn Fn() -> Result<()> + Send + Sync>>,
    interrupt_queue: Vec<u8>,
    interrupt_enabled: bool,
}

impl InterruptController {
    pub fn new() -> Self {
        Self {
            interrupt_handlers: HashMap::new(),
            interrupt_queue: Vec::new(),
            interrupt_enabled: true,
        }
    }

    /// Register an interrupt handler
    pub fn register_handler<F>(&mut self, vector: u8, handler: F)
    where
        F: Fn() -> Result<()> + Send + Sync + 'static,
    {
        self.interrupt_handlers.insert(vector, Arc::new(handler));
        log::debug!("Registered interrupt handler for vector {}", vector);
    }

    /// Trigger an interrupt
    pub fn trigger_interrupt(&mut self, vector: u8) -> Result<()> {
        if !self.interrupt_enabled {
            return Ok(());
        }

        self.interrupt_queue.push(vector);
        log::debug!("Triggered interrupt vector {}", vector);
        Ok(())
    }

    /// Process pending interrupts
    pub fn process_interrupts(&mut self) -> Result<()> {
        while let Some(vector) = self.interrupt_queue.pop() {
            if let Some(handler) = self.interrupt_handlers.get(&vector) {
                handler()?;
            } else {
                log::warn!("No handler for interrupt vector {}", vector);
            }
        }
        Ok(())
    }

    /// Enable interrupts
    pub fn enable_interrupts(&mut self) {
        self.interrupt_enabled = true;
    }

    /// Disable interrupts
    pub fn disable_interrupts(&mut self) {
        self.interrupt_enabled = false;
    }

    /// Check if interrupts are enabled
    pub fn interrupts_enabled(&self) -> bool {
        self.interrupt_enabled
    }

    /// Get pending interrupt count
    pub fn pending_count(&self) -> usize {
        self.interrupt_queue.len()
    }

    /// Clear all pending interrupts
    pub fn clear_pending(&mut self) {
        self.interrupt_queue.clear();
    }
}

/// Interrupt vectors
pub mod vectors {
    pub const TIMER: u8 = 0x20;
    pub const KEYBOARD: u8 = 0x21;
    pub const SERIAL: u8 = 0x23;
    pub const NETWORK: u8 = 0x24;
    pub const SYSTEM_CALL: u8 = 0x80;
    pub const PAGE_FAULT: u8 = 0x0E;
    pub const GENERAL_PROTECTION: u8 = 0x0D;
    pub const DOUBLE_FAULT: u8 = 0x08;
    pub const DIVIDE_ERROR: u8 = 0x00;
    pub const DEBUG: u8 = 0x01;
    pub const NMI: u8 = 0x02;
    pub const BREAKPOINT: u8 = 0x03;
    pub const OVERFLOW: u8 = 0x04;
    pub const BOUND_RANGE: u8 = 0x05;
    pub const INVALID_OPCODE: u8 = 0x06;
    pub const DEVICE_NOT_AVAILABLE: u8 = 0x07;
    pub const INVALID_TSS: u8 = 0x0A;
    pub const SEGMENT_NOT_PRESENT: u8 = 0x0B;
    pub const STACK_FAULT: u8 = 0x0C;
    pub const ALIGNMENT_CHECK: u8 = 0x11;
    pub const MACHINE_CHECK: u8 = 0x12;
    pub const SIMD_FLOATING_POINT: u8 = 0x13;
    pub const VIRTUALIZATION: u8 = 0x14;
    pub const SECURITY: u8 = 0x1E;
}
