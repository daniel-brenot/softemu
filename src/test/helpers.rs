use iced_x86::{Decoder, DecoderOptions, Instruction};

use crate::cpu::{CpuState, InstructionDecoder};
use crate::memory::{GuestMemory, MemoryManager, MmioManager};
use crate::Result;
use std::sync::{Arc, Mutex};

pub fn decode_instruction(bytes: &[u8]) -> Instruction {
    let mut decoder = Decoder::new(64, bytes, DecoderOptions::NONE);
    decoder.decode()
}

pub fn execute_instruction(bytes: &[u8], state: &mut CpuState) -> Result<()> {
    let instruction = decode_instruction(bytes);
    let decoder = InstructionDecoder::new();
    decoder.execute_instruction(&instruction, state)?;
    Ok(())
}

/// Create a test CPU state with a simple memory setup
pub fn create_test_cpu_state() -> Result<CpuState> {
    // Create guest memory (1MB for tests)
    let guest_memory = GuestMemory::new(1024 * 1024)?;
    
    // Create MMIO manager
    let mmio_manager = MmioManager::new();
    
    // Create shared MMIO manager reference
    let mmio_manager_ref = Arc::new(Mutex::new(mmio_manager));
    
    // Define MMIO space size (first 1MB is reserved for MMIO)
    let mmio_space_size = 0x100000; // 1MB
    
    // Create memory manager
    let memory_manager = Arc::new(Mutex::new(MemoryManager::new(guest_memory, mmio_manager_ref, mmio_space_size)));
    
    // Create CPU state
    Ok(CpuState::new(memory_manager))
}

/// Helper function to write to memory in tests
pub fn write_memory(state: &mut CpuState, addr: u64, value: u64) -> Result<()> {
    state.write_u64(addr, value)
}

/// Helper function to read from memory in tests
pub fn read_memory(state: &CpuState, addr: u64) -> Result<u64> {
    state.read_u64(addr)
}
