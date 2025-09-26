use crate::Result;
use iced_x86::Instruction;

use crate::cpu::{registers::RFlags, CpuState, InstructionDecoder};

impl InstructionDecoder<'_> {
    pub fn execute_hlt(&self, _instruction: &Instruction, _state: &mut CpuState) -> Result<()> {
        // HLT - Halt
        // Simplified implementation - just log for now
        log::debug!("HLT instruction executed");
        Ok(())
    }
}