use crate::cpu::{CpuState, InstructionDecoder};
use crate::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

/// CPU core implementation
pub struct CpuCore {
    state: Arc<Mutex<CpuState>>,
    decoder: InstructionDecoder<'static>,
    core_id: u32,
}

impl CpuCore {
    pub fn new(state: Arc<Mutex<CpuState>>, core_id: u32) -> Self {
        Self {
            state,
            decoder: InstructionDecoder::new(),
            core_id,
        }
    }

    /// Execute one instruction cycle
    pub async fn execute_cycle(&mut self) -> Result<bool> {
        let mut state = self.state.lock().await;
        
        if !state.running || state.halted {
            return Ok(false);
        }

        // Handle pending interrupts
        if state.interrupt_pending {
            self.handle_interrupt(&mut state).await?;
        }

        // Fetch instruction
        let instruction_bytes = self.fetch_instruction(&state)?;
        
        // Decode instruction
        let instruction = self.decoder.decode_instruction(&instruction_bytes);
        
        // Execute instruction
        self.decoder.execute_instruction(&instruction, &mut state)?;
        
        // Update instruction pointer (unless instruction modified it)
        if !self.instruction_modifies_rip(&instruction) {
            state.registers.rip += instruction.len() as u64;
        }

        Ok(state.running)
    }

    /// Fetch instruction bytes from memory
    fn fetch_instruction(&self, state: &CpuState) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        let mut addr = state.registers.rip;
        
        // Fetch up to 15 bytes (maximum x86_64 instruction length)
        for _ in 0..15 {
            bytes.push(state.read_u8(addr)?);
            addr += 1;
        }
        
        Ok(bytes)
    }

    /// Check if instruction modifies RIP
    fn instruction_modifies_rip(&self, instruction: &iced_x86::Instruction) -> bool {
        matches!(instruction.mnemonic(), 
            iced_x86::Mnemonic::Jmp | 
            iced_x86::Mnemonic::Call | 
            iced_x86::Mnemonic::Ret |
            iced_x86::Mnemonic::Iret)
    }

    /// Handle interrupt
    async fn handle_interrupt(&self, state: &mut CpuState) -> Result<()> {
        let vector = state.interrupt_vector;
        state.clear_interrupt();

        // Save current state
        state.registers.rsp -= 8;
        state.write_u64(state.registers.rsp, state.registers.rip)?;
        
        state.registers.rsp -= 2;
        state.write_u16(state.registers.rsp, state.registers.cs)?;
        
        state.registers.rsp -= 8;
        state.write_u64(state.registers.rsp, state.registers.rflags)?;

        // Jump to interrupt handler
        // For now, use a simple interrupt table
        let handler_addr = self.get_interrupt_handler(vector);
        state.registers.rip = handler_addr;
        
        // Clear interrupt flag
        state.registers.set_flag(crate::cpu::registers::RFlags::INTERRUPT, false);

        log::debug!("Handling interrupt vector {}", vector);
        Ok(())
    }

    /// Get interrupt handler address
    fn get_interrupt_handler(&self, vector: u8) -> u64 {
        // Simple interrupt table - in a real implementation this would
        // be more sophisticated and use the IDT
        match vector {
            0x20 => 0x1000, // Timer interrupt
            0x21 => 0x2000, // Keyboard interrupt
            0x80 => 0x3000, // System call
            _ => 0x4000,    // Default handler
        }
    }

    /// Start CPU execution
    pub async fn start(&self) -> Result<()> {
        let mut state = self.state.lock().await;
        state.start();
        log::info!("CPU core {} started", self.core_id);
        Ok(())
    }

    /// Stop CPU execution
    pub async fn stop(&self) -> Result<()> {
        let mut state = self.state.lock().await;
        state.halt();
        log::info!("CPU core {} stopped", self.core_id);
        Ok(())
    }

    /// Trigger interrupt on this CPU core
    pub async fn trigger_interrupt(&self, vector: u8) -> Result<()> {
        let mut state = self.state.lock().await;
        state.trigger_interrupt(vector);
        Ok(())
    }

    /// Get CPU core ID
    pub fn core_id(&self) -> u32 {
        self.core_id
    }
}
