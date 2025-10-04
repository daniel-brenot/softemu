use crate::cpu::CpuState;
use crate::Result;

/// Synchronous CPU core implementation
pub struct CpuCore {
    state: CpuState,
    core_id: u32,
}

impl CpuCore {
    /// Create a new synchronous CPU core
    pub fn new(core_id: u32) -> Self {
        Self {
            state: CpuState::new(),
            core_id,
        }
    }

    /// Start the CPU core
    pub fn start(&mut self) -> Result<()> {
        self.state.running = true;
        self.state.halted = false;
        
        // Set up initial state
        self.state.registers.rip = 0x100000; // Start at kernel entry point
        self.state.registers.rsp = 0x1F000000; // Set up stack pointer (496MB - well within 512MB limit)
        self.state.registers.cs = 0x08; // Code segment
        self.state.registers.ds = 0x10; // Data segment
        self.state.registers.ss = 0x18; // Stack segment
        
        log::info!("CPU core {} started", self.core_id);
        Ok(())
    }

    /// Stop the CPU core
    pub fn stop(&mut self) -> Result<()> {
        self.state.running = false;
        log::info!("CPU core {} stopped", self.core_id);
        Ok(())
    }

    /// Handle an interrupt
    fn handle_interrupt(&mut self) {
        // Save current state to stack
        self.state.write_u64(self.state.registers.rsp, self.state.registers.rip)?;
        self.state.registers.rsp -= 8;
        
        self.state.write_u16(self.state.registers.rsp, self.state.registers.cs)?;
        self.state.registers.rsp -= 2;
        
        self.state.write_u64(self.state.registers.rsp, self.state.registers.rflags)?;
        self.state.registers.rsp -= 8;
        
        // Set interrupt flag
        self.state.registers.rflags |= 0x200; // IF flag
        
        // Jump to interrupt handler
        self.state.registers.rip = 0x1000; // Placeholder interrupt handler address
        
        self.state.interrupt_pending = false;
        Ok(())
    }

    /// Get the core ID
    pub fn core_id(&self) -> u32 {
        self.core_id
    }
}
