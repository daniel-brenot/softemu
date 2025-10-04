use crate::cpu::CpuState;
use crate::Result;
use std::sync::Arc;
use std::sync::Mutex;

/// Synchronous CPU core implementation
pub struct CpuCore {
    state: Arc<Mutex<CpuState>>,
    core_id: u32,
}

impl CpuCore {
    /// Create a new synchronous CPU core
    pub fn new(state: Arc<Mutex<CpuState>>, core_id: u32) -> Self {
        Self {
            state,
            core_id,
        }
    }

    /// Start the CPU core
    pub fn start(&mut self) -> Result<()> {
        let mut state = self.state.lock().unwrap();
        state.running = true;
        state.halted = false;
        
        // Set up initial state
        state.registers.rip = 0x100000; // Start at kernel entry point
        state.registers.rsp = 0x1F000000; // Set up stack pointer (496MB - well within 512MB limit)
        state.registers.cs = 0x08; // Code segment
        state.registers.ds = 0x10; // Data segment
        state.registers.ss = 0x18; // Stack segment
        
        log::info!("CPU core {} started", self.core_id);
        Ok(())
    }

    /// Stop the CPU core
    pub fn stop(&mut self) -> Result<()> {
        let mut state = self.state.lock().unwrap();
        state.running = false;
        log::info!("CPU core {} stopped", self.core_id);
        Ok(())
    }

    /// Execute one instruction cycle
    pub fn execute_cycle(&mut self) -> Result<bool> {
        let mut state = self.state.lock().unwrap();
        
        if !state.running || state.halted {
            return Ok(false);
        }

        // Handle pending interrupts
        if state.interrupt_pending {
            self.handle_interrupt(&mut state)?;
        }

        Ok(state.running)
    }

    /// Handle an interrupt
    fn handle_interrupt(&self, state: &mut CpuState) -> Result<()> {
        // Save current state to stack
        state.write_u64(state.registers.rsp, state.registers.rip)?;
        state.registers.rsp -= 8;
        
        state.write_u16(state.registers.rsp, state.registers.cs)?;
        state.registers.rsp -= 2;
        
        state.write_u64(state.registers.rsp, state.registers.rflags)?;
        state.registers.rsp -= 8;
        
        // Set interrupt flag
        state.registers.rflags |= 0x200; // IF flag
        
        // Jump to interrupt handler
        state.registers.rip = 0x1000; // Placeholder interrupt handler address
        
        state.interrupt_pending = false;
        Ok(())
    }

    /// Fetch instruction from memory
    fn fetch_instruction(&self, state: &CpuState) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        let mut addr = state.registers.rip;
        
        // log::debug!("Fetching instruction at RIP=0x{:x}, memory size=0x{:x}", addr, state.memory.size());
        
        // Read up to 15 bytes (maximum x86_64 instruction length)
        for _ in 0..15 {
            bytes.push(state.read_u8(addr)?);
            addr += 1;
        }
        
        Ok(bytes)
    }

    /// Get the core ID
    pub fn core_id(&self) -> u32 {
        self.core_id
    }

    /// Get a reference to the CPU state
    pub fn get_state(&self) -> Arc<Mutex<CpuState>> {
        self.state.clone()
    }
}
