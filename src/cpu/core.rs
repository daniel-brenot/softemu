use crate::cpu::{CpuState, InstructionDecoder};
use crate::Result;
use std::sync::Arc;
use std::sync::Mutex;

/// Synchronous CPU core implementation
pub struct CpuCore {
    state: Arc<Mutex<CpuState>>,
    decoder: InstructionDecoder<'static>,
    core_id: u32,
}

impl CpuCore {
    /// Create a new synchronous CPU core
    pub fn new(state: Arc<Mutex<CpuState>>, core_id: u32) -> Self {
        Self {
            state,
            decoder: InstructionDecoder::new(),
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

        // Fetch instruction
        let instruction_bytes = self.fetch_instruction(&state)?;
        
        // Decode instruction
        let mut instruction = self.decoder.decode_instruction(&instruction_bytes);
        
        // If 64-bit decoding fails, try 32-bit decoding as fallback
        if instruction.mnemonic() == iced_x86::Mnemonic::INVALID {
            log::debug!("64-bit decode failed, trying 32-bit decode for instruction at RIP=0x{:x}", state.registers.rip);
            let mut decoder_32 = iced_x86::Decoder::new(32, &instruction_bytes, iced_x86::DecoderOptions::NONE);
            instruction = decoder_32.decode();
            
            if instruction.mnemonic() != iced_x86::Mnemonic::INVALID {
                log::debug!("Successfully decoded as 32-bit instruction: {:?}", instruction.mnemonic());
            } else {
                log::error!("Invalid instruction at RIP=0x{:x}, bytes: {:02x?}", 
                           state.registers.rip, 
                           &instruction_bytes[..instruction.len()]);
            }
        }
        
        // Execute instruction
        self.decoder.execute_instruction(&instruction, &mut state)?;
        
        // Update instruction pointer (unless instruction modified it)
        if !self.instruction_modifies_rip(&instruction) {
            state.registers.rip += instruction.len() as u64;
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

    /// Check if instruction modifies RIP
    fn instruction_modifies_rip(&self, instruction: &iced_x86::Instruction) -> bool {
        match instruction.mnemonic() {
            iced_x86::Mnemonic::Call | 
            iced_x86::Mnemonic::Ret | 
            iced_x86::Mnemonic::Jmp => true,
            _ => false,
        }
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
